//! Format des skills juridiques et premier skill : la résolution
//! normative temporelle (`docs/SKILLS.md`).
//!
//! Un skill est une procédure déclarée, sans code arbitraire : seuls
//! les outils qu'il autorise lui sont accessibles, par l'exécuteur.

use crate::corpus::{Corpus, CorpusState};
use crate::domain::anomaly::TemporalAnomaly;
use crate::execution::tool::{
    AnomalyOutcome, Executor, ResolutionOutcome, ToolError, ToolKind, TraceEntry,
};
use crate::simulation::{DatedFact, FactStatus, Scenario};

/// Descripteur d'un skill. Les textes documentent la procédure ;
/// `allowed_tools` est appliqué par l'exécuteur.
#[derive(Debug)]
pub struct Skill {
    pub id: &'static str,
    pub objective: &'static str,
    pub preconditions: &'static [&'static str],
    pub allowed_tools: &'static [ToolKind],
    pub procedure: &'static [&'static str],
    pub required_evidence: &'static [&'static str],
    pub outputs: &'static [&'static str],
    pub failure_conditions: &'static [&'static str],
}

pub const TEMPORAL_RESOLUTION: Skill = Skill {
    id: "resolution-normative-temporelle",
    objective: "Identifier, selon la source consultée, la version d'une disposition \
                en vigueur à la date d'une situation, avec sa provenance et les \
                anomalies qui en affectent la fiabilité.",
    preconditions: &[
        "Corpus officiel doté de sa provenance : nom, date d'état, date d'acquisition, SHA-256.",
        "Scénario synthétique désignant la disposition par l'un de ses identifiants \
         LEGIARTI et la date de la situation, avec le statut de ce fait.",
    ],
    allowed_tools: &[
        ToolKind::CorpusState,
        ToolKind::ResolveProvision,
        ToolKind::DetectAnomalies,
    ],
    procedure: &[
        "Consulter l'état du corpus pour fixer la provenance.",
        "Résoudre la disposition à la date de la situation.",
        "Détecter les anomalies de la disposition.",
        "Assembler le rapport : résultat documentaire, réserves, limites, trace.",
    ],
    required_evidence: &[
        "Identifiant et période de chaque version retenue ou encadrante.",
        "Source : nom, date d'état, date d'acquisition, SHA-256.",
        "Trace des appels d'outils.",
    ],
    outputs: &[
        "Résultat documentaire : version en vigueur, aucune version en vigueur, \
         résultat ambigu ou disposition hors corpus.",
        "Réserves : date non établie, absence documentaire, versions concurrentes, anomalies.",
        "Limites permanentes du skill.",
    ],
    failure_conditions: &[
        "Outil non autorisé par le skill.",
        "Identifiant ne désignant pas une version d'article.",
    ],
};

/// Limites valables pour tout résultat de ce skill.
pub const TEMPORAL_RESOLUTION_LIMITS: &[&str] = &[
    "Le résultat indique la version que la source tient pour en vigueur à la date ; \
     il ne détermine pas à lui seul la norme applicable (droit transitoire, \
     hiérarchie des normes, sources hors corpus).",
    "L'absence d'une disposition dans le corpus est une absence documentaire, non \
     une absence de norme.",
    "La source reflète le droit à sa date d'état, sans les modifications ultérieures.",
];

/// Ce qui empêche de tenir le résultat documentaire pour acquis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reservation {
    /// Le résultat vaut sous la condition que la date soit établie.
    SituationDateNotEstablished(FactStatus),
    DocumentaryAbsence,
    ConcurrentVersions,
    AnomaliesDetected(usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemporalResolutionReport {
    situation: DatedFact,
    corpus: CorpusState,
    outcome: ResolutionOutcome,
    anomalies: Vec<TemporalAnomaly>,
    reservations: Vec<Reservation>,
    trace: Vec<TraceEntry>,
}

impl TemporalResolutionReport {
    pub fn situation(&self) -> &DatedFact {
        &self.situation
    }

    pub fn corpus(&self) -> &CorpusState {
        &self.corpus
    }

    pub fn outcome(&self) -> &ResolutionOutcome {
        &self.outcome
    }

    pub fn anomalies(&self) -> &[TemporalAnomaly] {
        &self.anomalies
    }

    pub fn reservations(&self) -> &[Reservation] {
        &self.reservations
    }

    pub fn limits(&self) -> &'static [&'static str] {
        TEMPORAL_RESOLUTION_LIMITS
    }

    pub fn trace(&self) -> &[TraceEntry] {
        &self.trace
    }
}

/// Exécute `TEMPORAL_RESOLUTION` ; le corpus n'est lu qu'au travers de
/// l'exécuteur, borné aux outils du skill.
pub fn resolve_temporally(
    scenario: &Scenario,
    corpus: &Corpus,
) -> Result<TemporalResolutionReport, ToolError> {
    let mut executor = Executor::new(corpus, TEMPORAL_RESOLUTION.allowed_tools);
    let situation = scenario.situation();

    let state = executor.corpus_state()?.into_result();
    let outcome = executor
        .resolve_provision(scenario.provision(), situation.date())?
        .into_result();
    let anomalies = match executor
        .detect_anomalies(scenario.provision())?
        .into_result()
    {
        AnomalyOutcome::Detected(anomalies) => anomalies,
        AnomalyOutcome::NotInCorpus => Vec::new(),
    };

    let mut reservations = Vec::new();
    if situation.status() != FactStatus::Established {
        reservations.push(Reservation::SituationDateNotEstablished(situation.status()));
    }
    match outcome {
        ResolutionOutcome::NotInCorpus => reservations.push(Reservation::DocumentaryAbsence),
        ResolutionOutcome::Ambiguous(_) => reservations.push(Reservation::ConcurrentVersions),
        ResolutionOutcome::InForce(_) | ResolutionOutcome::NoVersionInForce { .. } => {}
    }
    if !anomalies.is_empty() {
        reservations.push(Reservation::AnomaliesDetected(anomalies.len()));
    }

    Ok(TemporalResolutionReport {
        situation: situation.clone(),
        corpus: state,
        outcome,
        anomalies,
        reservations,
        trace: executor.into_trace(),
    })
}
