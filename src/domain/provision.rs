//! Versions d'une disposition et résolution de la version applicable à
//! une date de référence.
//!
//! L'applicabilité se détermine par les seules périodes de vigueur ;
//! l'état officiel n'intervient pas (`docs/ARCHITECTURE.md`, § 5).

use std::error::Error;
use std::fmt;

use crate::domain::legi::{LegiId, LegiIdKind, LegiStatus};
use crate::temporal::{CivilDate, ValidityPeriod};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProvisionVersion {
    id: LegiId,
    number: String,
    status: Option<LegiStatus>,
    period: ValidityPeriod,
}

impl ProvisionVersion {
    /// `number` est le numéro officiel (`NUM`), conservé tel quel ;
    /// `status` vaut `None` lorsque la source ne donne aucun état.
    pub fn new(
        id: LegiId,
        number: &str,
        status: Option<LegiStatus>,
        period: ValidityPeriod,
    ) -> Result<Self, ProvisionError> {
        if id.kind() != LegiIdKind::Article {
            return Err(ProvisionError::NotAnArticle(id));
        }

        Ok(Self {
            id,
            number: number.to_string(),
            status,
            period,
        })
    }

    pub fn id(&self) -> LegiId {
        self.id
    }

    pub fn number(&self) -> &str {
        &self.number
    }

    pub fn status(&self) -> Option<LegiStatus> {
        self.status
    }

    pub fn period(&self) -> ValidityPeriod {
        self.period
    }
}

/// Disposition connue par l'ensemble de ses versions. Les versions sont
/// rangées par entrée en vigueur puis par identifiant : aucun résultat
/// ne dépend de l'ordre de lecture des sources.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Provision {
    versions: Vec<ProvisionVersion>,
}

impl Provision {
    pub fn new(mut versions: Vec<ProvisionVersion>) -> Result<Self, ProvisionError> {
        if versions.is_empty() {
            return Err(ProvisionError::NoVersion);
        }

        // Tri par identifiant d'abord, pour signaler toujours le même
        // doublon quel que soit l'ordre d'entrée.
        versions.sort_by_key(|version| version.id);
        if let Some(pair) = versions.windows(2).find(|pair| pair[0].id == pair[1].id) {
            return Err(ProvisionError::DuplicateVersion(pair[0].id));
        }

        versions.sort_by_key(|version| (version.period.start(), version.id));
        Ok(Self { versions })
    }

    /// Versions dans l'ordre d'entrée en vigueur, puis d'identifiant.
    pub fn versions(&self) -> &[ProvisionVersion] {
        &self.versions
    }

    /// Une période vide ne s'applique jamais et n'encadre rien.
    pub fn resolve(&self, reference_date: CivilDate) -> Resolution<'_> {
        let in_force: Vec<&ProvisionVersion> = self
            .versions
            .iter()
            .filter(|version| version.period.contains(reference_date))
            .collect();

        match in_force.len() {
            0 => Resolution::NoneApplicable {
                previous: self
                    .versions
                    .iter()
                    .filter(|v| !v.period.is_empty())
                    .filter(|v| v.period.end().is_some_and(|end| end <= reference_date))
                    .max_by_key(|v| (v.period.end(), v.period.start(), v.id)),
                next: self
                    .versions
                    .iter()
                    .filter(|v| !v.period.is_empty() && v.period.start() > reference_date)
                    .min_by_key(|v| (v.period.start(), v.id)),
            },
            1 => Resolution::Applicable(in_force[0]),
            _ => Resolution::Ambiguous(in_force),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Resolution<'a> {
    /// Une seule version en vigueur à la date de référence.
    Applicable(&'a ProvisionVersion),
    /// Aucune version en vigueur ; versions encadrantes, s'il en existe.
    NoneApplicable {
        previous: Option<&'a ProvisionVersion>,
        next: Option<&'a ProvisionVersion>,
    },
    /// Plusieurs versions en vigueur : anomalie des données ; aucune
    /// n'est choisie.
    Ambiguous(Vec<&'a ProvisionVersion>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProvisionError {
    NotAnArticle(LegiId),
    NoVersion,
    DuplicateVersion(LegiId),
}

impl fmt::Display for ProvisionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotAnArticle(id) => write!(f, "{id} n'identifie pas une version d'article"),
            Self::NoVersion => f.write_str("Disposition sans aucune version"),
            Self::DuplicateVersion(id) => write!(f, "Version {id} présente plusieurs fois"),
        }
    }
}

impl Error for ProvisionError {}

/// Constructeurs de test partagés avec `anomaly`. Identifiants fictifs :
/// ces tests n'attestent d'aucun texte réel.
#[cfg(test)]
pub(crate) mod fixtures {
    use super::*;

    pub(crate) fn id(n: u8) -> LegiId {
        format!("LEGIARTI{n:012}").parse().unwrap()
    }

    pub(crate) fn date(text: &str) -> CivilDate {
        text.parse().unwrap()
    }

    pub(crate) fn version(
        n: u8,
        status: Option<LegiStatus>,
        start: &str,
        end: Option<&str>,
    ) -> ProvisionVersion {
        let period = ValidityPeriod::new(date(start), end.map(date)).unwrap();
        ProvisionVersion::new(id(n), "1", status, period).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::fixtures::{date, id, version};
    use super::*;

    const MODIFIED: Option<LegiStatus> = Some(LegiStatus::Modifie);

    #[test]
    fn rejects_identifier_that_is_not_an_article() {
        let section = "LEGISCTA000006109057".parse().unwrap();
        let period = ValidityPeriod::new(date("2020-01-01"), None).unwrap();

        assert_eq!(
            ProvisionVersion::new(section, "1", None, period),
            Err(ProvisionError::NotAnArticle(section))
        );
    }

    #[test]
    fn rejects_empty_provision_and_duplicates() {
        assert_eq!(Provision::new(vec![]), Err(ProvisionError::NoVersion));

        let twice = vec![
            version(1, MODIFIED, "2020-01-01", None),
            version(1, MODIFIED, "2021-01-01", None),
        ];
        assert_eq!(
            Provision::new(twice),
            Err(ProvisionError::DuplicateVersion(id(1)))
        );
    }

    #[test]
    fn overlapping_versions_make_resolution_ambiguous() {
        let provision = Provision::new(vec![
            version(1, MODIFIED, "2020-01-01", Some("2023-01-01")),
            version(2, MODIFIED, "2022-01-01", None),
        ])
        .unwrap();

        let Resolution::Ambiguous(versions) = provision.resolve(date("2022-06-01")) else {
            panic!("résolution ambiguë attendue");
        };
        let ids: Vec<LegiId> = versions.iter().map(|v| v.id()).collect();
        assert_eq!(ids, [id(1), id(2)]);
    }

    #[test]
    fn empty_period_never_applies_nor_frames_a_date() {
        let provision = Provision::new(vec![
            version(1, MODIFIED, "2020-01-01", Some("2021-01-01")),
            version(
                2,
                Some(LegiStatus::ModifieMortNe),
                "2021-06-01",
                Some("2021-06-01"),
            ),
            version(3, MODIFIED, "2022-01-01", None),
        ])
        .unwrap();

        assert_eq!(
            provision.resolve(date("2021-06-01")),
            Resolution::NoneApplicable {
                previous: Some(&provision.versions()[0]),
                next: Some(&provision.versions()[2]),
            }
        );
    }
}
