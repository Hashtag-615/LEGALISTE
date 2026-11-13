//! Environnement d'exécution : outils fermés, skill de résolution
//! normative temporelle, frontière entre simulation et corpus.

mod common;

use common::{ARCHIVE_SHA256, ARTICLE_16_11, corpus, date, id};
use legaliste::domain::anomaly::TemporalAnomaly;
use legaliste::execution::skill::{self, Reservation, TEMPORAL_RESOLUTION};
use legaliste::execution::tool::{
    AnomalyOutcome, Executor, ResolutionOutcome, ToolError, ToolKind, ToolRequest,
};
use legaliste::simulation::{DatedFact, FactStatus, Scenario};

/// Article d'un autre texte, réel mais absent du corpus réduit.
const ABSENT_ARTICLE: &str = "LEGIARTI000006850357";

fn scenario(provision: &str, situation_date: &str, status: FactStatus) -> Scenario {
    let fact = DatedFact::new("Situation synthétique", date(situation_date), status).unwrap();
    Scenario::new("Scénario d'essai", id(provision), fact).unwrap()
}

#[test]
fn simulation_cannot_modify_the_corpus() {
    let corpus = corpus(ARTICLE_16_11);
    let before = corpus.clone();

    for (provision, situation_date) in [
        ("LEGIARTI000006419309", "2010-06-15"),
        ("LEGIARTI000006419309", "2006-01-01"),
        (ABSENT_ARTICLE, "2010-06-15"),
    ] {
        let scenario = scenario(provision, situation_date, FactStatus::Hypothetical);
        skill::resolve_temporally(&scenario, &corpus).unwrap();
    }

    assert_eq!(corpus, before);
}

#[test]
fn refuses_tools_outside_the_allowlist_and_traces_the_refusal() {
    let corpus = corpus(ARTICLE_16_11);
    let mut executor = Executor::new(&corpus, &[ToolKind::CorpusState]);
    let article = id("LEGIARTI000006419309");

    assert!(executor.corpus_state().is_ok());
    assert_eq!(
        executor.resolve_provision(article, date("2010-06-15")),
        Err(ToolError::NotAuthorized(ToolKind::ResolveProvision))
    );
    assert_eq!(
        executor.detect_anomalies(article),
        Err(ToolError::NotAuthorized(ToolKind::DetectAnomalies))
    );

    let outcomes: Vec<_> = executor
        .trace()
        .iter()
        .map(|e| e.outcome().clone())
        .collect();
    assert_eq!(
        outcomes,
        [
            Ok(vec![]),
            Err(ToolError::NotAuthorized(ToolKind::ResolveProvision)),
            Err(ToolError::NotAuthorized(ToolKind::DetectAnomalies)),
        ]
    );
}

#[test]
fn rejects_identifier_that_is_not_an_article() {
    let corpus = corpus(ARTICLE_16_11);
    let mut executor = Executor::new(&corpus, TEMPORAL_RESOLUTION.allowed_tools);
    let text = id("LEGITEXT000006070721");

    assert_eq!(
        executor.resolve_provision(text, date("2010-06-15")),
        Err(ToolError::NotAnArticle(text))
    );
}

#[test]
fn documentary_absence_is_not_absence_of_norm() {
    let corpus = corpus(ARTICLE_16_11);

    let absent = skill::resolve_temporally(
        &scenario(ABSENT_ARTICLE, "2010-06-15", FactStatus::Established),
        &corpus,
    )
    .unwrap();
    assert_eq!(absent.outcome(), &ResolutionOutcome::NotInCorpus);
    assert_eq!(absent.reservations(), [Reservation::DocumentaryAbsence]);

    let in_gap = skill::resolve_temporally(
        &scenario(
            "LEGIARTI000006419309",
            "2006-01-01",
            FactStatus::Established,
        ),
        &corpus,
    )
    .unwrap();
    let ResolutionOutcome::NoVersionInForce { previous, next } = in_gap.outcome() else {
        panic!(
            "aucune version en vigueur attendue : {:?}",
            in_gap.outcome()
        );
    };
    assert_eq!(
        previous.as_ref().map(|v| v.id()),
        Some(id("LEGIARTI000006419308"))
    );
    assert_eq!(
        next.as_ref().map(|v| v.id()),
        Some(id("LEGIARTI000006419309"))
    );
    assert_eq!(in_gap.reservations(), [Reservation::AnomaliesDetected(1)]);
}

#[test]
fn report_carries_provenance_reservations_and_trace() {
    let corpus = corpus(ARTICLE_16_11);
    let report = skill::resolve_temporally(
        &scenario("LEGIARTI000047569313", "2010-06-15", FactStatus::Alleged),
        &corpus,
    )
    .unwrap();

    let ResolutionOutcome::InForce(version) = report.outcome() else {
        panic!("version en vigueur attendue : {:?}", report.outcome());
    };
    assert_eq!(version.id(), id("LEGIARTI000006419309"));
    assert_eq!(report.corpus().source().sha256(), ARCHIVE_SHA256);
    assert_eq!(report.corpus().source().state_date(), date("2026-11-08"));
    assert!(matches!(report.anomalies(), [TemporalAnomaly::Gap { .. }]));
    assert_eq!(
        report.reservations(),
        [
            Reservation::SituationDateNotEstablished(FactStatus::Alleged),
            Reservation::AnomaliesDetected(1),
        ]
    );
    assert!(!report.limits().is_empty());

    let requests: Vec<_> = report.trace().iter().map(|e| e.request().clone()).collect();
    assert_eq!(
        requests,
        [
            ToolRequest::CorpusState,
            ToolRequest::ResolveProvision {
                provision: id("LEGIARTI000047569313"),
                reference_date: date("2010-06-15"),
            },
            ToolRequest::DetectAnomalies {
                provision: id("LEGIARTI000047569313"),
            },
        ]
    );
    let read = report.trace()[1].outcome().as_ref().unwrap();
    assert_eq!(read.len(), ARTICLE_16_11.len());
}

#[test]
fn identical_inputs_give_identical_reports() {
    let forward = corpus(ARTICLE_16_11);
    let backward = corpus(ARTICLE_16_11.into_iter().rev());
    let scenario = scenario("LEGIARTI000006419308", "2006-01-01", FactStatus::Disputed);

    let first = skill::resolve_temporally(&scenario, &forward).unwrap();
    let again = skill::resolve_temporally(&scenario, &forward).unwrap();
    let other_order = skill::resolve_temporally(&scenario, &backward).unwrap();

    assert_eq!(first, again);
    assert_eq!(first, other_order);
}

#[test]
fn anomaly_tool_distinguishes_absence_from_clean_result() {
    let corpus = corpus(ARTICLE_16_11);
    let mut executor = Executor::new(&corpus, TEMPORAL_RESOLUTION.allowed_tools);

    let absent = executor.detect_anomalies(id(ABSENT_ARTICLE)).unwrap();
    assert_eq!(absent.result(), &AnomalyOutcome::NotInCorpus);
    assert!(absent.provenance().versions_read().is_empty());
}
