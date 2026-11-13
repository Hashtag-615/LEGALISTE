//! Dates civiles et périodes de validité.
//!
//! Une période est un intervalle semi-ouvert : la date de début est
//! incluse, la date de fin exclue (`docs/ARCHITECTURE.md`, § 5).

use std::error::Error;
use std::fmt;
use std::str::FromStr;

/// Date du calendrier grégorien, écrite `AAAA-MM-JJ`. L'ordre des
/// champs donne l'ordre chronologique.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CivilDate {
    year: u16,
    month: u8,
    day: u8,
}

impl FromStr for CivilDate {
    type Err = TemporalError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let invalid = || TemporalError::InvalidDate(text.to_string());
        let bytes = text.as_bytes();

        if bytes.len() != 10 || bytes[4] != b'-' || bytes[7] != b'-' {
            return Err(invalid());
        }

        // Après ce contrôle, le texte est entièrement ASCII : les
        // découpes ci-dessous tombent sur des limites de caractère.
        if !bytes
            .iter()
            .enumerate()
            .all(|(i, b)| i == 4 || i == 7 || b.is_ascii_digit())
        {
            return Err(invalid());
        }

        let year: u16 = text[0..4].parse().map_err(|_| invalid())?;
        let month: u8 = text[5..7].parse().map_err(|_| invalid())?;
        let day: u8 = text[8..10].parse().map_err(|_| invalid())?;

        if year == 0
            || !(1..=12).contains(&month)
            || !(1..=days_in_month(year, month)).contains(&day)
        {
            return Err(invalid());
        }

        Ok(Self { year, month, day })
    }
}

impl fmt::Display for CivilDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

fn days_in_month(year: u16, month: u8) -> u8 {
    let leap = year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400));

    match month {
        2 if leap => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

/// Période de validité. Une fin absente signifie qu'aucune fin n'est
/// connue. Une période dont la fin n'excède pas le début est vide : elle
/// ne contient aucune date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ValidityPeriod {
    start: CivilDate,
    end: Option<CivilDate>,
}

impl ValidityPeriod {
    /// Période saisie : une fin antérieure au début est une erreur.
    pub fn new(start: CivilDate, end: Option<CivilDate>) -> Result<Self, TemporalError> {
        match end {
            Some(end) if end < start => Err(TemporalError::EndBeforeStart { start, end }),
            _ => Ok(Self { start, end }),
        }
    }

    /// Période telle qu'une source officielle la publie, conservée sans
    /// correction. Une fin antérieure au début y est un fait de la
    /// source : la période est inversée, donc vide, et ses deux dates
    /// restent restituables.
    pub fn as_published(start: CivilDate, end: Option<CivilDate>) -> Self {
        Self { start, end }
    }

    pub fn start(&self) -> CivilDate {
        self.start
    }

    pub fn end(&self) -> Option<CivilDate> {
        self.end
    }

    pub fn is_empty(&self) -> bool {
        self.end.is_some_and(|end| end <= self.start)
    }

    pub fn is_inverted(&self) -> bool {
        self.end.is_some_and(|end| end < self.start)
    }

    pub fn contains(&self, date: CivilDate) -> bool {
        date >= self.start && self.end.is_none_or(|end| date < end)
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        !self.is_empty()
            && !other.is_empty()
            && other.end.is_none_or(|end| self.start < end)
            && self.end.is_none_or(|end| other.start < end)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemporalError {
    InvalidDate(String),
    EndBeforeStart { start: CivilDate, end: CivilDate },
}

impl fmt::Display for TemporalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDate(text) => {
                write!(f, "Date invalide : « {text} » (AAAA-MM-JJ attendu)")
            }
            Self::EndBeforeStart { start, end } => {
                write!(f, "Fin de période ({end}) antérieure au début ({start})")
            }
        }
    }
}

impl Error for TemporalError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(text: &str) -> CivilDate {
        text.parse().unwrap()
    }

    fn period(start: &str, end: Option<&str>) -> ValidityPeriod {
        ValidityPeriod::new(date(start), end.map(date)).unwrap()
    }

    #[test]
    fn accepts_valid_dates() {
        for text in ["2024-02-29", "2000-02-29", "2025-11-24", "0001-01-01"] {
            assert_eq!(date(text).to_string(), text);
        }
    }

    #[test]
    fn rejects_invalid_dates() {
        for text in [
            "2025-02-29",
            "1900-02-29",
            "2025-13-01",
            "2025-00-10",
            "2025-04-31",
            "2025-01-00",
            "0000-01-01",
            "2025-1-01",
            "2025/01/01",
            "2025-01-01 ",
            "２０２５-01",
        ] {
            assert_eq!(
                text.parse::<CivilDate>(),
                Err(TemporalError::InvalidDate(text.to_string()))
            );
        }
    }

    #[test]
    fn dates_order_chronologically() {
        assert!(date("1999-12-31") < date("2000-01-01"));
        assert!(date("2000-01-31") < date("2000-02-01"));
    }

    #[test]
    fn period_contains_start_but_not_end() {
        let period = period("2020-01-01", Some("2024-01-01"));

        assert!(period.contains(date("2020-01-01")));
        assert!(period.contains(date("2023-12-31")));
        assert!(!period.contains(date("2024-01-01")));
        assert!(!period.contains(date("2019-12-31")));
    }

    #[test]
    fn open_period_has_no_end() {
        assert!(period("2020-01-01", None).contains(date("9999-12-31")));
    }

    #[test]
    fn adjacent_periods_do_not_overlap() {
        let first = period("2020-01-01", Some("2024-01-01"));
        let second = period("2024-01-01", None);

        assert!(!first.overlaps(&second));
        assert!(!second.overlaps(&first));
    }

    #[test]
    fn overlapping_periods_are_detected() {
        let first = period("2020-01-01", Some("2025-01-01"));
        let second = period("2024-01-01", None);

        assert!(first.overlaps(&second));
        assert!(second.overlaps(&first));
    }

    #[test]
    fn empty_period_contains_nothing_and_overlaps_nothing() {
        let empty = period("2022-01-01", Some("2022-01-01"));
        let around = period("2020-01-01", None);

        assert!(empty.is_empty());
        assert!(!empty.contains(date("2022-01-01")));
        assert!(!empty.overlaps(&around));
        assert!(!around.overlaps(&empty));
    }

    #[test]
    fn rejects_reversed_period() {
        assert_eq!(
            ValidityPeriod::new(date("2025-01-01"), Some(date("2024-01-01"))),
            Err(TemporalError::EndBeforeStart {
                start: date("2025-01-01"),
                end: date("2024-01-01"),
            })
        );
    }

    #[test]
    fn published_inverted_period_is_kept_and_empty() {
        let around = period("1990-01-01", None);
        let inverted = ValidityPeriod::as_published(date("1994-03-01"), Some(date("1993-09-02")));

        assert_eq!(inverted.start(), date("1994-03-01"));
        assert_eq!(inverted.end(), Some(date("1993-09-02")));
        assert!(inverted.is_inverted() && inverted.is_empty());
        assert!(!inverted.contains(date("1993-12-01")));
        assert!(!inverted.overlaps(&around));
    }
}
