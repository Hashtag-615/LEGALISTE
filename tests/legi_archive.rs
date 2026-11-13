//! Parcours réel : le Code civil lu dans l'archive LEGI de référence.
//!
//! Ces tests exigent l'archive (zone corpus public, hors dépôt) : son
//! absence les fait échouer, jamais réussir (CODE_POLICY, § 8).
//! L'archive est un flux gzip : sa lecture, environ une minute, n'a lieu
//! qu'une fois par exécution.

mod common;

use std::fs;
use std::path::PathBuf;
use std::slice;
use std::sync::OnceLock;
use std::time::{Duration, Instant, SystemTime};

use common::legi_extracts::EXTRACTS;
use common::{
    ARCHIVE, ARCHIVE_SHA256, ARTICLE_16_11, CODE_CIVIL, FOREIGN_ARTICLE, Scratch, date, id,
    legi_source, provision,
};
use legaliste::corpus::Corpus;
use legaliste::corpus::legi_text::{self, TextExtract};
use legaliste::domain::anomaly::{self, TemporalAnomaly};
use legaliste::domain::legi::LegiId;
use legaliste::execution::skill::{self, Reservation, TemporalResolutionReport};
use legaliste::execution::tool::ResolutionOutcome;
use legaliste::infrastructure::{legi_archive, legi_store};
use legaliste::simulation::{DatedFact, FactStatus, Scenario};

/// Empreinte du contenu logique de la base du Code civil, mesurée le
/// 2026-11-13 : toute lecture et tout import de la même archive doivent
/// la reproduire.
const CODE_CIVIL_CONTENT_SHA256: &str =
    "6f486ca619340110a48423062dcbe0d9c3c5e7a9add6990032572a498cf26542";

struct Reading {
    extract: TextExtract,
    duration: Duration,
    length: u64,
    modified: SystemTime,
}

fn archive() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("CORPUS/00_SOURCES_OFFICIELLES/LEGI")
        .join(ARCHIVE)
}

fn reading() -> &'static Reading {
    static READING: OnceLock<Reading> = OnceLock::new();
    READING.get_or_init(|| {
        let path = archive();
        let metadata = fs::metadata(&path).unwrap_or_else(|error| {
            panic!(
                "archive de référence introuvable ({}) : {error}",
                path.display()
            )
        });
        let started = Instant::now();
        let extract = legi_archive::load_text(&path, legi_source(), id(CODE_CIVIL)).unwrap();
        Reading {
            extract,
            duration: started.elapsed(),
            length: metadata.len(),
            modified: metadata.modified().unwrap(),
        }
    })
}

fn report(article: &str, situation: &str, status: FactStatus) -> TemporalResolutionReport {
    report_on(reading().extract.corpus(), article, situation, status)
}

fn report_on(
    corpus: &Corpus,
    article: &str,
    situation: &str,
    status: FactStatus,
) -> TemporalResolutionReport {
    let fact = DatedFact::new("Situation synthétique", date(situation), status).unwrap();
    let scenario = Scenario::new("Code civil, archive LEGI", id(article), fact).unwrap();
    skill::resolve_temporally(&scenario, corpus).unwrap()
}

fn in_force(report: &TemporalResolutionReport) -> Option<LegiId> {
    match report.outcome() {
        ResolutionOutcome::InForce(version) => Some(version.id()),
        _ => None,
    }
}

/// Volumes mesurés indépendamment (analyse `awk` du 2026-11-13).
#[test]
fn reads_the_whole_code_civil_with_its_provenance() {
    let extract = &reading().extract;
    let state = extract.corpus().state();

    assert_eq!(extract.document_count(), 6395);
    assert_eq!(state.provision_count(), 3540);
    assert_eq!(state.version_count(), 6395);
    assert!(extract.versions_without_document().is_empty());
    assert_eq!(state.source().name(), ARCHIVE);
    assert_eq!(state.source().sha256(), ARCHIVE_SHA256);
    assert_eq!(state.source().state_date(), date("2026-11-08"));
}

#[test]
fn article_16_11_matches_extracts_and_transcription() {
    let from_archive = reading()
        .extract
        .corpus()
        .provision(id("LEGIARTI000006419309"));
    let extracts: String = EXTRACTS.iter().map(|e| e.xml).collect();
    let from_extracts = legi_text::build(legi_source(), id(CODE_CIVIL), &extracts).unwrap();

    assert_eq!(
        from_archive,
        from_extracts.corpus().provision(id("LEGIARTI000006419309"))
    );
    assert_eq!(from_archive, Some(&provision(ARTICLE_16_11)));
}

#[test]
fn resolves_article_16_11_at_several_dates() {
    let article = "LEGIARTI000047569313";
    let cases = [
        ("2010-06-15", Some("LEGIARTI000006419309")),
        ("2023-05-20", Some("LEGIARTI000039367698")),
        ("2023-05-21", Some("LEGIARTI000047569313")),
        ("2026-11-13", Some("LEGIARTI000047569313")),
        ("2006-01-01", None),
        ("1994-07-29", None),
    ];

    for (situation, expected) in cases {
        let report = report(article, situation, FactStatus::Established);
        assert_eq!(in_force(&report), expected.map(id), "au {situation}");
        assert!(matches!(report.anomalies(), [TemporalAnomaly::Gap { .. }]));
        assert_eq!(
            report,
            self::report(article, situation, FactStatus::Established)
        );
    }

    let ResolutionOutcome::NoVersionInForce { previous, next } =
        report(article, "2006-01-01", FactStatus::Established)
            .outcome()
            .clone()
    else {
        panic!("lacune 2005-2007 attendue");
    };
    assert_eq!(previous.map(|v| v.id()), Some(id("LEGIARTI000006419308")));
    assert_eq!(next.map(|v| v.id()), Some(id("LEGIARTI000006419309")));
}

#[test]
fn distinguishes_absence_and_concurrence_in_the_real_corpus() {
    let absent = report(FOREIGN_ARTICLE, "2010-06-15", FactStatus::Established);
    assert_eq!(absent.outcome(), &ResolutionOutcome::NotInCorpus);
    assert_eq!(absent.reservations(), [Reservation::DocumentaryAbsence]);

    let concurrent = report("LEGIARTI000006422707", "1994-02-15", FactStatus::Alleged);
    assert!(matches!(concurrent.outcome(), ResolutionOutcome::Ambiguous(v) if v.len() == 2));
    assert_eq!(
        concurrent.reservations(),
        [
            Reservation::SituationDateNotEstablished(FactStatus::Alleged),
            Reservation::ConcurrentVersions,
            Reservation::AnomaliesDetected(1),
        ]
    );
}

/// Périodes inversées, vides et listes chevauchantes : mêmes nombres que
/// l'analyse indépendante de l'archive.
#[test]
fn anomaly_counts_match_independent_scan() {
    let counts = anomaly_counts(reading().extract.corpus());
    let [inverted, empty, overlapping, gaps, conflicts, missing] = counts;

    eprintln!(
        "Code civil : {inverted} périodes inversées, {empty} vides, {overlapping} dispositions \
         à chevauchement, {gaps} lacunes, {conflicts} conflits d'état, {missing} états absents"
    );
    assert_eq!((inverted, empty, overlapping), (37, 1, 22));
}

/// Périodes inversées, périodes vides, dispositions à chevauchement,
/// lacunes, conflits d'état, états absents.
fn anomaly_counts(corpus: &Corpus) -> [usize; 6] {
    let source_date = corpus.source().state_date();
    let mut counts = [0; 6];

    for provision in corpus.provisions() {
        let anomalies = anomaly::detect(provision, source_date);
        let count = |predicate: fn(&TemporalAnomaly) -> bool| {
            anomalies.iter().filter(|a| predicate(a)).count()
        };
        counts[0] += count(|a| matches!(a, TemporalAnomaly::InvertedPeriod(_)));
        counts[2] += usize::from(count(|a| matches!(a, TemporalAnomaly::Overlap { .. })) > 0);
        counts[3] += count(|a| matches!(a, TemporalAnomaly::Gap { .. }));
        counts[4] += count(|a| matches!(a, TemporalAnomaly::StatusConflict { .. }));
        counts[5] += count(|a| matches!(a, TemporalAnomaly::MissingStatus(_)));
    }
    counts[1] = corpus
        .provisions()
        .iter()
        .flat_map(|p| p.versions())
        .filter(|v| v.period().is_empty() && !v.period().is_inverted())
        .count();
    counts
}

/// Base SQLite bâtie depuis la lecture réelle : deux imports
/// indépendants, relecture en lecture seule, comparaison avec la
/// lecture directe de l'archive, temps mesurés.
#[test]
fn code_civil_through_sqlite() {
    let reading = reading();
    let scratch = Scratch::new("code-civil");
    let first = scratch.file("premier.sqlite3");
    let second = scratch.file("second.sqlite3");

    let started = Instant::now();
    legi_store::import(&first, slice::from_ref(&reading.extract)).unwrap();
    let import = started.elapsed();
    legi_store::import(&second, slice::from_ref(&reading.extract)).unwrap();

    let content = legi_store::logical_content(&first).unwrap();
    assert_eq!(content, legi_store::logical_content(&second).unwrap());
    let digest = legi_archive::sha256_bytes(content.as_bytes()).unwrap();

    let file_digest = legi_archive::sha256_file(&first).unwrap();
    let started = Instant::now();
    let stored = legi_store::load_text(&first, id(CODE_CIVIL)).unwrap();
    let load = started.elapsed();
    assert_eq!(stored, reading.extract);
    assert_eq!(
        anomaly_counts(stored.corpus()),
        anomaly_counts(reading.extract.corpus())
    );

    let dates = [
        "1994-07-29",
        "2006-01-01",
        "2010-06-15",
        "2023-05-20",
        "2023-05-21",
        "2026-11-13",
    ];
    let started = Instant::now();
    let from_store: Vec<_> = dates
        .iter()
        .map(|d| {
            report_on(
                stored.corpus(),
                "LEGIARTI000047569313",
                d,
                FactStatus::Established,
            )
        })
        .collect();
    let resolve = started.elapsed() / dates.len() as u32;
    for (report, situation) in from_store.iter().zip(dates) {
        assert_eq!(
            report,
            &self::report("LEGIARTI000047569313", situation, FactStatus::Established)
        );
    }
    assert_eq!(legi_archive::sha256_file(&first).unwrap(), file_digest);

    eprintln!(
        "Code civil, temps mesurés : lecture de l'archive {:.1} s ; import SQLite {:.2} s ; \
         consultation (contrôles et relecture) {} ms ; résolution {} µs ; base {} octets ; \
         contenu logique {digest}",
        reading.duration.as_secs_f64(),
        import.as_secs_f64(),
        load.as_millis(),
        resolve.as_micros(),
        fs::metadata(&first).unwrap().len()
    );
    assert_eq!(digest, CODE_CIVIL_CONTENT_SHA256);
}

#[test]
fn reading_leaves_the_archive_unchanged() {
    let reading = reading();
    let metadata = fs::metadata(archive()).unwrap();

    assert_eq!(metadata.len(), reading.length);
    assert_eq!(metadata.modified().unwrap(), reading.modified);
    assert_eq!(
        legi_archive::sha256_file(&archive()).unwrap(),
        ARCHIVE_SHA256
    );
}
