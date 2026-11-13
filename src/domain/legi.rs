//! Conventions de la base LEGI (DILA) : identifiants officiels, états
//! officiels (`ETAT`) et bornes des périodes de vigueur.

use std::error::Error;
use std::fmt;
use std::str::FromStr;

use crate::temporal::{CivilDate, TemporalError, ValidityPeriod};

/// Date de fin conventionnelle de LEGI : aucune fin de vigueur connue.
pub const OPEN_END: &str = "2999-01-01";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LegiIdKind {
    /// `LEGIARTI` : version d'article.
    Article,
    /// `LEGISCTA` : section.
    Section,
    /// `LEGITEXT` : texte consolidé.
    Text,
    /// `JORFTEXT` : texte publié au Journal officiel.
    JorfText,
}

impl LegiIdKind {
    const ALL: [Self; 4] = [Self::Article, Self::Section, Self::Text, Self::JorfText];

    pub fn prefix(self) -> &'static str {
        match self {
            Self::Article => "LEGIARTI",
            Self::Section => "LEGISCTA",
            Self::Text => "LEGITEXT",
            Self::JorfText => "JORFTEXT",
        }
    }
}

/// Identifiant officiel : préfixe de huit lettres suivi de douze
/// chiffres. La forme textuelle est restituée à l'identique.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LegiId {
    kind: LegiIdKind,
    number: u64,
}

impl LegiId {
    pub fn kind(self) -> LegiIdKind {
        self.kind
    }
}

impl FromStr for LegiId {
    type Err = LegiError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let invalid = || LegiError::InvalidId(text.to_string());

        // Le contrôle ASCII précède la découpe, qui suppose un octet par
        // caractère.
        if !text.is_ascii() || text.len() != 20 {
            return Err(invalid());
        }

        let (prefix, digits) = text.split_at(8);
        let kind = LegiIdKind::ALL
            .into_iter()
            .find(|kind| kind.prefix() == prefix)
            .ok_or_else(invalid)?;

        if !digits.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(invalid());
        }

        let number = digits.parse().map_err(|_| invalid())?;

        Ok(Self { kind, number })
    }
}

impl fmt::Display for LegiId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{:012}", self.kind.prefix(), self.number)
    }
}

/// État officiel d'une version d'article, valable à la date de l'état
/// de la source. Les variantes reprennent le vocabulaire de LEGI sans
/// le traduire, pour n'en altérer ni n'en requalifier le sens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LegiStatus {
    Vigueur,
    VigueurDiff,
    Modifie,
    ModifieMortNe,
    Abroge,
    AbrogeDiff,
    Transfere,
    Perime,
    Annule,
    Disjoint,
    Deplace,
}

impl LegiStatus {
    const ALL: [Self; 11] = [
        Self::Vigueur,
        Self::VigueurDiff,
        Self::Modifie,
        Self::ModifieMortNe,
        Self::Abroge,
        Self::AbrogeDiff,
        Self::Transfere,
        Self::Perime,
        Self::Annule,
        Self::Disjoint,
        Self::Deplace,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Vigueur => "VIGUEUR",
            Self::VigueurDiff => "VIGUEUR_DIFF",
            Self::Modifie => "MODIFIE",
            Self::ModifieMortNe => "MODIFIE_MORT_NE",
            Self::Abroge => "ABROGE",
            Self::AbrogeDiff => "ABROGE_DIFF",
            Self::Transfere => "TRANSFERE",
            Self::Perime => "PERIME",
            Self::Annule => "ANNULE",
            Self::Disjoint => "DISJOINT",
            Self::Deplace => "DEPLACE",
        }
    }

    /// Lit le contenu d'un élément `ETAT`. Un élément vide, constaté
    /// dans l'archive, donne `None` : l'absence est conservée, non
    /// comblée.
    pub fn from_legi(text: &str) -> Result<Option<Self>, LegiError> {
        if text.is_empty() {
            return Ok(None);
        }

        text.parse().map(Some)
    }
}

impl FromStr for LegiStatus {
    type Err = LegiError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .into_iter()
            .find(|status| status.as_str() == text)
            .ok_or_else(|| LegiError::UnknownStatus(text.to_string()))
    }
}

impl fmt::Display for LegiStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Construit la période de vigueur d'une version à partir de
/// `DATE_DEBUT` et `DATE_FIN`, telle que publiée. La fin `2999-01-01`
/// se lit « sans fin » lorsque l'entrée en vigueur la précède. Toute
/// autre fin est conservée, même lointaine (`2222-02-22`) ou antérieure
/// au début : de telles périodes inversées figurent dans l'archive.
pub fn validity_period(start: &str, end: &str) -> Result<ValidityPeriod, LegiError> {
    let start: CivilDate = start.parse()?;
    let end: CivilDate = end.parse()?;
    let open = end.to_string() == OPEN_END && start < end;

    Ok(ValidityPeriod::as_published(
        start,
        if open { None } else { Some(end) },
    ))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LegiError {
    InvalidId(String),
    UnknownStatus(String),
    Temporal(TemporalError),
}

impl fmt::Display for LegiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidId(text) => write!(f, "Identifiant LEGI invalide : « {text} »"),
            Self::UnknownStatus(text) => write!(f, "État LEGI inconnu : « {text} »"),
            Self::Temporal(error) => write!(f, "Période LEGI invalide : {error}"),
        }
    }
}

impl Error for LegiError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Temporal(error) => Some(error),
            _ => None,
        }
    }
}

impl From<TemporalError> for LegiError {
    fn from(error: TemporalError) -> Self {
        Self::Temporal(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifier_round_trips_with_leading_zeros() {
        for text in [
            "LEGIARTI000006419307",
            "LEGISCTA000006109057",
            "LEGITEXT000006070721",
            "JORFTEXT000000546652",
        ] {
            let id: LegiId = text.parse().unwrap();
            assert_eq!(id.to_string(), text);
        }
    }

    #[test]
    fn identifier_kind_follows_prefix() {
        let id: LegiId = "LEGIARTI000047569313".parse().unwrap();
        assert_eq!(id.kind(), LegiIdKind::Article);
    }

    #[test]
    fn rejects_malformed_identifiers() {
        for text in [
            "",
            "LEGIARTI00000641930",
            "LEGIARTI0000064193070",
            "KALIARTI000006419307",
            "legiarti000006419307",
            "LEGIARTI00000641930X",
            "LEGIARTI00000641930é",
            "LEGIARTé00000641930",
        ] {
            assert_eq!(
                text.parse::<LegiId>(),
                Err(LegiError::InvalidId(text.to_string()))
            );
        }
    }

    #[test]
    fn status_round_trips_official_vocabulary() {
        for status in LegiStatus::ALL {
            assert_eq!(status.as_str().parse::<LegiStatus>(), Ok(status));
        }
    }

    #[test]
    fn empty_status_is_kept_as_absent() {
        assert_eq!(LegiStatus::from_legi(""), Ok(None));
        assert_eq!(
            LegiStatus::from_legi("VIGUEUR"),
            Ok(Some(LegiStatus::Vigueur))
        );
    }

    #[test]
    fn rejects_unknown_status() {
        assert_eq!(
            LegiStatus::from_legi("vigueur"),
            Err(LegiError::UnknownStatus("vigueur".to_string()))
        );
    }

    #[test]
    fn conventional_end_means_open_period() {
        let period = validity_period("2023-05-21", OPEN_END).unwrap();
        assert_eq!(period.end(), None);
    }

    #[test]
    fn distant_end_other_than_convention_is_kept() {
        let period = validity_period("2020-01-01", "2222-02-22").unwrap();
        assert_eq!(period.end(), Some("2222-02-22".parse().unwrap()));
    }

    #[test]
    fn rejects_impossible_legi_date() {
        assert!(matches!(
            validity_period("2025-02-29", OPEN_END),
            Err(LegiError::Temporal(TemporalError::InvalidDate(_)))
        ));
    }

    // Dates relevées sur LEGIARTI000006434422 (`MODIFIE_MORT_NE`).
    #[test]
    fn inverted_official_period_is_kept_as_published() {
        let period = validity_period("1994-03-01", "1993-09-02").unwrap();

        assert_eq!(period.start().to_string(), "1994-03-01");
        assert_eq!(
            period.end().map(|end| end.to_string()).as_deref(),
            Some("1993-09-02")
        );
        assert!(period.is_inverted());
    }

    #[test]
    fn conventional_end_is_literal_when_start_is_conventional_too() {
        let period = validity_period(OPEN_END, OPEN_END).unwrap();

        assert_eq!(
            period.end().map(|end| end.to_string()).as_deref(),
            Some(OPEN_END)
        );
        assert!(period.is_empty());
    }
}
