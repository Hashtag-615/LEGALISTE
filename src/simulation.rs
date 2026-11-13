//! Espace de simulation : scénarios synthétiques et éphémères
//! (`docs/ARCHITECTURE.md`, D7). Un scénario formule une question sur
//! le corpus ; il n'est jamais enregistré et ne contient aucune donnée
//! réelle de dossier.

use std::error::Error;
use std::fmt;

use crate::domain::legi::{LegiId, LegiIdKind};
use crate::temporal::CivilDate;

/// Statut d'un fait (`AGENTS.md`, doctrine juridique).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FactStatus {
    Established,
    Alleged,
    Disputed,
    Hypothetical,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatedFact {
    description: String,
    date: CivilDate,
    status: FactStatus,
}

impl DatedFact {
    pub fn new(
        description: &str,
        date: CivilDate,
        status: FactStatus,
    ) -> Result<Self, ScenarioError> {
        if description.trim().is_empty() {
            return Err(ScenarioError::MissingDescription);
        }

        Ok(Self {
            description: description.to_string(),
            date,
            status,
        })
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn date(&self) -> CivilDate {
        self.date
    }

    pub fn status(&self) -> FactStatus {
        self.status
    }
}

/// Question : quelle version de la disposition régissait la situation ?
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scenario {
    label: String,
    provision: LegiId,
    situation: DatedFact,
}

impl Scenario {
    /// La disposition est désignée par l'un de ses identifiants de
    /// version (`LEGIARTI`).
    pub fn new(
        label: &str,
        provision: LegiId,
        situation: DatedFact,
    ) -> Result<Self, ScenarioError> {
        if label.trim().is_empty() {
            return Err(ScenarioError::MissingLabel);
        }

        if provision.kind() != LegiIdKind::Article {
            return Err(ScenarioError::NotAnArticle(provision));
        }

        Ok(Self {
            label: label.to_string(),
            provision,
            situation,
        })
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn provision(&self) -> LegiId {
        self.provision
    }

    pub fn situation(&self) -> &DatedFact {
        &self.situation
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScenarioError {
    MissingLabel,
    MissingDescription,
    NotAnArticle(LegiId),
}

impl fmt::Display for ScenarioError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingLabel => f.write_str("Scénario sans intitulé"),
            Self::MissingDescription => f.write_str("Fait sans description"),
            Self::NotAnArticle(id) => write!(f, "{id} ne désigne pas une version d'article"),
        }
    }
}

impl Error for ScenarioError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(text: &str) -> CivilDate {
        text.parse().unwrap()
    }

    #[test]
    fn rejects_incomplete_scenario() {
        let article = "LEGIARTI000006419307".parse().unwrap();
        let text = "LEGITEXT000006070721".parse().unwrap();
        let fact = DatedFact::new(
            "Signature du contrat",
            date("2010-06-15"),
            FactStatus::Alleged,
        );

        assert_eq!(
            DatedFact::new(" ", date("2010-06-15"), FactStatus::Alleged),
            Err(ScenarioError::MissingDescription)
        );
        assert_eq!(
            Scenario::new("", article, fact.clone().unwrap()),
            Err(ScenarioError::MissingLabel)
        );
        assert_eq!(
            Scenario::new("Essai", text, fact.unwrap()),
            Err(ScenarioError::NotAnArticle(text))
        );
    }
}
