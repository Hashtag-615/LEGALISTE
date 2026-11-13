//! Contrat des outils de l'orchestrateur : ensemble fermé d'opérations
//! en lecture seule sur le corpus, entrées et sorties typées, erreurs
//! explicites, provenance et trace d'exécution.
//!
//! L'exécuteur ne détient qu'une référence partagée au corpus et
//! n'accède ni au réseau, ni aux fichiers, ni à l'horloge, ni à un
//! processus : à corpus et appels identiques, sorties et trace
//! identiques.

use std::error::Error;
use std::fmt;

use crate::corpus::{Corpus, CorpusState, SourceRecord};
use crate::domain::anomaly::{self, TemporalAnomaly};
use crate::domain::legi::{LegiId, LegiIdKind};
use crate::domain::provision::{Provision, ProvisionVersion, Resolution};
use crate::temporal::CivilDate;

/// Ensemble fermé : aucun outil ne s'ajoute à l'exécution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ToolKind {
    CorpusState,
    ResolveProvision,
    DetectAnomalies,
}

/// Appel tel qu'enregistré dans la trace.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolRequest {
    CorpusState,
    ResolveProvision {
        provision: LegiId,
        reference_date: CivilDate,
    },
    DetectAnomalies {
        provision: LegiId,
    },
}

impl ToolRequest {
    pub fn kind(&self) -> ToolKind {
        match self {
            Self::CorpusState => ToolKind::CorpusState,
            Self::ResolveProvision { .. } => ToolKind::ResolveProvision,
            Self::DetectAnomalies { .. } => ToolKind::DetectAnomalies,
        }
    }
}

/// Résolution détachée du corpus.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolutionOutcome {
    /// Identifiant absent du corpus : absence documentaire. Rien n'en
    /// résulte quant à l'existence d'une norme.
    NotInCorpus,
    InForce(ProvisionVersion),
    /// Disposition connue, sans version en vigueur à la date selon la
    /// source. Cela n'établit pas qu'aucune autre norme ne s'applique.
    NoVersionInForce {
        previous: Option<ProvisionVersion>,
        next: Option<ProvisionVersion>,
    },
    /// Plusieurs versions en vigueur : aucune n'est choisie.
    Ambiguous(Vec<ProvisionVersion>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnomalyOutcome {
    NotInCorpus,
    Detected(Vec<TemporalAnomaly>),
}

/// Source consultée et versions effectivement lues.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Provenance {
    source: SourceRecord,
    versions_read: Vec<LegiId>,
}

impl Provenance {
    pub fn source(&self) -> &SourceRecord {
        &self.source
    }

    pub fn versions_read(&self) -> &[LegiId] {
        &self.versions_read
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolOutput<T> {
    result: T,
    provenance: Provenance,
}

impl<T> ToolOutput<T> {
    pub fn result(&self) -> &T {
        &self.result
    }

    pub fn provenance(&self) -> &Provenance {
        &self.provenance
    }

    pub fn into_result(self) -> T {
        self.result
    }
}

/// Entrée de trace : rang de l'appel, requête, versions lues ou erreur.
/// Aucune heure n'y figure, pour que la trace soit reproductible.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceEntry {
    step: usize,
    request: ToolRequest,
    outcome: Result<Vec<LegiId>, ToolError>,
}

impl TraceEntry {
    pub fn step(&self) -> usize {
        self.step
    }

    pub fn request(&self) -> &ToolRequest {
        &self.request
    }

    pub fn outcome(&self) -> &Result<Vec<LegiId>, ToolError> {
        &self.outcome
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolError {
    NotAuthorized(ToolKind),
    NotAnArticle(LegiId),
}

impl fmt::Display for ToolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotAuthorized(kind) => write!(f, "Outil non autorisé : {kind:?}"),
            Self::NotAnArticle(id) => write!(f, "{id} ne désigne pas une version d'article"),
        }
    }
}

impl Error for ToolError {}

pub struct Executor<'c> {
    corpus: &'c Corpus,
    allowed: &'static [ToolKind],
    trace: Vec<TraceEntry>,
}

impl<'c> Executor<'c> {
    /// `allowed` : outils autorisés, en pratique ceux d'un skill.
    pub fn new(corpus: &'c Corpus, allowed: &'static [ToolKind]) -> Self {
        Self {
            corpus,
            allowed,
            trace: Vec::new(),
        }
    }

    pub fn trace(&self) -> &[TraceEntry] {
        &self.trace
    }

    pub fn into_trace(self) -> Vec<TraceEntry> {
        self.trace
    }

    pub fn corpus_state(&mut self) -> Result<ToolOutput<CorpusState>, ToolError> {
        self.invoke(ToolRequest::CorpusState, |corpus| {
            Ok((corpus.state(), Vec::new()))
        })
    }

    pub fn resolve_provision(
        &mut self,
        provision: LegiId,
        reference_date: CivilDate,
    ) -> Result<ToolOutput<ResolutionOutcome>, ToolError> {
        let request = ToolRequest::ResolveProvision {
            provision,
            reference_date,
        };
        self.invoke(request, |corpus| {
            let Some(found) = find_article(corpus, provision)? else {
                return Ok((ResolutionOutcome::NotInCorpus, Vec::new()));
            };
            let outcome = match found.resolve(reference_date) {
                Resolution::Applicable(version) => ResolutionOutcome::InForce(version.clone()),
                Resolution::NoneApplicable { previous, next } => {
                    ResolutionOutcome::NoVersionInForce {
                        previous: previous.cloned(),
                        next: next.cloned(),
                    }
                }
                Resolution::Ambiguous(versions) => {
                    ResolutionOutcome::Ambiguous(versions.into_iter().cloned().collect())
                }
            };
            Ok((outcome, version_ids(found)))
        })
    }

    /// Les états officiels sont contrôlés à la date de l'état de la
    /// source du corpus.
    pub fn detect_anomalies(
        &mut self,
        provision: LegiId,
    ) -> Result<ToolOutput<AnomalyOutcome>, ToolError> {
        self.invoke(ToolRequest::DetectAnomalies { provision }, |corpus| {
            let Some(found) = find_article(corpus, provision)? else {
                return Ok((AnomalyOutcome::NotInCorpus, Vec::new()));
            };
            let detected = anomaly::detect(found, corpus.source().state_date());
            Ok((AnomalyOutcome::Detected(detected), version_ids(found)))
        })
    }

    /// Point unique d'autorisation, d'exécution et de traçage.
    fn invoke<T>(
        &mut self,
        request: ToolRequest,
        run: impl FnOnce(&Corpus) -> Result<(T, Vec<LegiId>), ToolError>,
    ) -> Result<ToolOutput<T>, ToolError> {
        let kind = request.kind();
        let result = if self.allowed.contains(&kind) {
            run(self.corpus)
        } else {
            Err(ToolError::NotAuthorized(kind))
        };

        self.trace.push(TraceEntry {
            step: self.trace.len() + 1,
            request,
            outcome: result
                .as_ref()
                .map(|(_, read)| read.clone())
                .map_err(|e| *e),
        });

        result.map(|(result, versions_read)| ToolOutput {
            result,
            provenance: Provenance {
                source: self.corpus.source().clone(),
                versions_read,
            },
        })
    }
}

fn find_article(corpus: &Corpus, id: LegiId) -> Result<Option<&Provision>, ToolError> {
    if id.kind() != LegiIdKind::Article {
        return Err(ToolError::NotAnArticle(id));
    }
    Ok(corpus.provision(id))
}

fn version_ids(provision: &Provision) -> Vec<LegiId> {
    provision.versions().iter().map(|v| v.id()).collect()
}
