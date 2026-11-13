//! Lecture de documents LEGI authentiques (`tests/common/legi_extracts.rs`)
//! et skill de résolution normative temporelle sur ces documents.

mod common;

use common::legi_extracts::EXTRACTS;
use common::{ARTICLE_16_11, CODE_CIVIL, FOREIGN_ARTICLE, date, id, legi_source, provision};
use legaliste::corpus::legi_text::{self, ExtractError, TextExtract};
use legaliste::domain::anomaly::{Standing, TemporalAnomaly};
use legaliste::domain::legi::{LegiId, LegiStatus};
use legaliste::execution::skill::{self, Reservation, TemporalResolutionReport};
use legaliste::execution::tool::ResolutionOutcome;
use legaliste::infrastructure::legi_archive;
use legaliste::simulation::{DatedFact, FactStatus, Scenario};

fn build<'a>(documents: impl IntoIterator<Item = &'a str>) -> Result<TextExtract, ExtractError> {
    let stream: String = documents.into_iter().collect();
    legi_text::build(legi_source(), id(CODE_CIVIL), &stream)
}

fn extract() -> TextExtract {
    build(EXTRACTS.iter().map(|e| e.xml)).unwrap()
}

fn report(extract: &TextExtract, article: &str, situation: &str) -> TemporalResolutionReport {
    let fact = DatedFact::new(
        "Situation synthétique",
        date(situation),
        FactStatus::Established,
    );
    let scenario = Scenario::new("Extraits LEGI", id(article), fact.unwrap()).unwrap();
    skill::resolve_temporally(&scenario, extract.corpus()).unwrap()
}

fn ids(versions: &[legaliste::domain::provision::ProvisionVersion]) -> Vec<LegiId> {
    versions.iter().map(|v| v.id()).collect()
}

#[test]
fn extracts_are_byte_identical_to_archive_members() {
    for extract in &EXTRACTS {
        let digest = legi_archive::sha256_bytes(extract.xml.as_bytes()).unwrap();
        assert_eq!(digest, extract.sha256, "{}", extract.member);
    }
}

#[test]
fn reads_versions_as_published() {
    let extract = extract();

    assert_eq!(extract.document_count(), 10);
    assert_eq!(extract.corpus().state().provision_count(), 4);
    assert_eq!(
        extract.corpus().provision(id("LEGIARTI000006419309")),
        Some(&provision(ARTICLE_16_11))
    );
    // Versions citées par les listes des art. 210, 214 et 311-21, dont
    // les documents ne figurent pas parmi les extraits.
    let without_document: Vec<LegiId> = [
        "LEGIARTI000006422707",
        "LEGIARTI000006422754",
        "LEGIARTI000006424850",
        "LEGIARTI000006424851",
        "LEGIARTI000020558688",
        "LEGIARTI000020558840",
        "LEGIARTI000027432045",
        "LEGIARTI000043895575",
    ]
    .map(id)
    .to_vec();
    assert_eq!(extract.versions_without_document(), without_document);
}

#[test]
fn reading_order_does_not_change_the_corpus() {
    assert_eq!(
        build(EXTRACTS.iter().rev().map(|e| e.xml)).unwrap(),
        extract()
    );
}

#[test]
fn distinguishes_documentary_situations() {
    let extract = extract();

    let unique = report(&extract, "LEGIARTI000047569313", "2010-06-15");
    let ResolutionOutcome::InForce(version) = unique.outcome() else {
        panic!("version unique attendue : {:?}", unique.outcome());
    };
    assert_eq!(version.id(), id("LEGIARTI000006419309"));

    let absent = report(&extract, FOREIGN_ARTICLE, "2010-06-15");
    assert_eq!(absent.outcome(), &ResolutionOutcome::NotInCorpus);
    assert_eq!(absent.reservations(), [Reservation::DocumentaryAbsence]);

    let gap = report(&extract, "LEGIARTI000047569313", "2006-01-01");
    let ResolutionOutcome::NoVersionInForce { previous, next } = gap.outcome() else {
        panic!("période sans version attendue : {:?}", gap.outcome());
    };
    assert_eq!(
        previous.as_ref().map(|v| v.id()),
        Some(id("LEGIARTI000006419308"))
    );
    assert_eq!(
        next.as_ref().map(|v| v.id()),
        Some(id("LEGIARTI000006419309"))
    );

    let concurrent = report(&extract, "LEGIARTI000006422706", "1994-02-15");
    let ResolutionOutcome::Ambiguous(versions) = concurrent.outcome() else {
        panic!(
            "versions concurrentes attendues : {:?}",
            concurrent.outcome()
        );
    };
    assert_eq!(
        ids(versions),
        [id("LEGIARTI000006422706"), id("LEGIARTI000006422707")]
    );
    assert_eq!(
        concurrent.reservations(),
        [
            Reservation::ConcurrentVersions,
            Reservation::AnomaliesDetected(1)
        ]
    );
}

#[test]
fn preserves_official_anomalies() {
    let extract = extract();

    assert_eq!(
        report(&extract, "LEGIARTI000006424849", "2010-01-01").anomalies(),
        [TemporalAnomaly::InvertedPeriod(id("LEGIARTI000006424849"))]
    );
    assert_eq!(
        report(&extract, "LEGIARTI000055027581", "2000-01-01").anomalies(),
        [TemporalAnomaly::StatusConflict {
            version: id("LEGIARTI000055027581"),
            status: LegiStatus::Vigueur,
            standing: Standing::Ended,
        }]
    );
    assert_eq!(
        report(&extract, "LEGIARTI000006422706", "2000-01-01").anomalies(),
        [TemporalAnomaly::Overlap {
            first: id("LEGIARTI000006422706"),
            second: id("LEGIARTI000006422707"),
        }]
    );
}

#[test]
fn altered_documents_are_rejected_not_corrected() {
    let altered = |rank: usize, from: &str, to: &str| {
        assert!(EXTRACTS[rank].xml.contains(from), "motif absent : {from}");
        let mut documents: Vec<String> = EXTRACTS.iter().map(|e| e.xml.to_string()).collect();
        documents[rank] = documents[rank].replacen(from, to, 1);
        build(documents.iter().map(String::as_str))
    };

    assert_eq!(
        altered(
            0,
            "<DATE_FIN>2004-08-07</DATE_FIN>",
            "<DATE_FIN>2004-08-08</DATE_FIN>"
        ),
        Err(ExtractError::MetadataDiffersFromList(id(
            "LEGIARTI000006419307"
        )))
    );
    assert_eq!(
        altered(
            1,
            "fin=\"2011-03-16\" id=\"LEGIARTI000006419309\"",
            "fin=\"2011-03-17\" id=\"LEGIARTI000006419309\""
        ),
        Err(ExtractError::DivergentVersionLists(id(
            "LEGIARTI000006419308"
        )))
    );
    assert!(matches!(
        altered(2, "</VERSIONS>", ""),
        Err(ExtractError::Xml { document: 3, .. })
    ));
}
