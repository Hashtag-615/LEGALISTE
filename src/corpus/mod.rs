//! Corpus juridique officiel : instantané immuable d'une source, avec
//! sa provenance. Aucune méthode ne le modifie après construction ;
//! outils et simulations n'en reçoivent qu'une référence partagée.

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;

use crate::domain::legi::LegiId;
use crate::domain::provision::Provision;
use crate::temporal::CivilDate;

/// Source officielle dont un corpus est tiré.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceRecord {
    name: String,
    state_date: CivilDate,
    acquisition_date: CivilDate,
    sha256: String,
}

impl SourceRecord {
    /// `state_date` : date de l'état de la source ; `acquisition_date` :
    /// date de sa capture ; `sha256` : empreinte calculée localement.
    pub fn new(
        name: &str,
        state_date: CivilDate,
        acquisition_date: CivilDate,
        sha256: &str,
    ) -> Result<Self, CorpusError> {
        if name.is_empty() {
            return Err(CorpusError::MissingSourceName);
        }

        let hex = |byte: u8| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte);
        if sha256.len() != 64 || !sha256.bytes().all(hex) {
            return Err(CorpusError::InvalidDigest(sha256.to_string()));
        }

        // Une source ne peut être capturée avant la date de son état.
        if acquisition_date < state_date {
            return Err(CorpusError::AcquiredBeforeState {
                state_date,
                acquisition_date,
            });
        }

        Ok(Self {
            name: name.to_string(),
            state_date,
            acquisition_date,
            sha256: sha256.to_string(),
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn state_date(&self) -> CivilDate {
        self.state_date
    }

    pub fn acquisition_date(&self) -> CivilDate {
        self.acquisition_date
    }

    pub fn sha256(&self) -> &str {
        &self.sha256
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Corpus {
    source: SourceRecord,
    provisions: Vec<Provision>,
    /// Identifiant de version vers rang de sa disposition.
    index: BTreeMap<LegiId, usize>,
}

impl Corpus {
    /// Les dispositions sont rangées par plus petit identifiant de
    /// version : deux lectures d'une même source donnent le même corpus.
    pub fn new(source: SourceRecord, mut provisions: Vec<Provision>) -> Result<Self, CorpusError> {
        provisions.sort_by_key(|provision| provision.versions().iter().map(|v| v.id()).min());

        let mut index = BTreeMap::new();
        for (rank, provision) in provisions.iter().enumerate() {
            for version in provision.versions() {
                if index.insert(version.id(), rank).is_some() {
                    return Err(CorpusError::VersionInSeveralProvisions(version.id()));
                }
            }
        }

        Ok(Self {
            source,
            provisions,
            index,
        })
    }

    pub fn source(&self) -> &SourceRecord {
        &self.source
    }

    /// Disposition désignée par l'un quelconque de ses identifiants de
    /// version. `None` signifie une absence documentaire, non l'absence
    /// d'une norme.
    pub fn provision(&self, version_id: LegiId) -> Option<&Provision> {
        self.index
            .get(&version_id)
            .map(|&rank| &self.provisions[rank])
    }

    pub fn state(&self) -> CorpusState {
        CorpusState {
            source: self.source.clone(),
            provision_count: self.provisions.len(),
            version_count: self.index.len(),
        }
    }
}

/// État d'un corpus : provenance et volume.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CorpusState {
    source: SourceRecord,
    provision_count: usize,
    version_count: usize,
}

impl CorpusState {
    pub fn source(&self) -> &SourceRecord {
        &self.source
    }

    pub fn provision_count(&self) -> usize {
        self.provision_count
    }

    pub fn version_count(&self) -> usize {
        self.version_count
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CorpusError {
    MissingSourceName,
    InvalidDigest(String),
    AcquiredBeforeState {
        state_date: CivilDate,
        acquisition_date: CivilDate,
    },
    VersionInSeveralProvisions(LegiId),
}

impl fmt::Display for CorpusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingSourceName => f.write_str("Source sans nom"),
            Self::InvalidDigest(text) => {
                write!(f, "Empreinte SHA-256 invalide : « {text} »")
            }
            Self::AcquiredBeforeState {
                state_date,
                acquisition_date,
            } => write!(
                f,
                "Acquisition ({acquisition_date}) antérieure à l'état de la source ({state_date})"
            ),
            Self::VersionInSeveralProvisions(id) => {
                write!(f, "Version {id} rattachée à plusieurs dispositions")
            }
        }
    }
}

impl Error for CorpusError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::provision::fixtures::{date, id, version};

    const DIGEST: &str = "90c9dc4a0a362943eef055881f00a727ac5d5f08fb9b4ec472e9da6da572bda2";

    fn source() -> SourceRecord {
        SourceRecord::new("archive", date("2026-11-08"), date("2026-11-13"), DIGEST).unwrap()
    }

    #[test]
    fn rejects_invalid_provenance() {
        let (state, acquired) = (date("2026-11-08"), date("2026-11-13"));

        assert_eq!(
            SourceRecord::new("", state, acquired, DIGEST),
            Err(CorpusError::MissingSourceName)
        );
        assert_eq!(
            SourceRecord::new("archive", state, acquired, &DIGEST.to_uppercase()),
            Err(CorpusError::InvalidDigest(DIGEST.to_uppercase()))
        );
        assert_eq!(
            SourceRecord::new("archive", acquired, state, DIGEST),
            Err(CorpusError::AcquiredBeforeState {
                state_date: acquired,
                acquisition_date: state,
            })
        );
    }

    #[test]
    fn finds_provision_by_any_version_and_counts_volume() {
        let provision = Provision::new(vec![
            version(1, None, "2020-01-01", Some("2021-01-01")),
            version(2, None, "2021-01-01", None),
        ])
        .unwrap();
        let corpus = Corpus::new(source(), vec![provision.clone()]).unwrap();

        assert_eq!(corpus.provision(id(1)), Some(&provision));
        assert_eq!(corpus.provision(id(2)), Some(&provision));
        assert_eq!(corpus.provision(id(3)), None);
        assert_eq!(corpus.state().provision_count(), 1);
        assert_eq!(corpus.state().version_count(), 2);
    }

    #[test]
    fn rejects_version_shared_by_two_provisions() {
        let first = Provision::new(vec![version(1, None, "2020-01-01", None)]).unwrap();
        let second = Provision::new(vec![version(1, None, "2021-01-01", None)]).unwrap();

        assert_eq!(
            Corpus::new(source(), vec![first, second]),
            Err(CorpusError::VersionInSeveralProvisions(id(1)))
        );
    }
}
