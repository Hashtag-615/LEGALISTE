#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidityPeriod {
    pub start: String,
    pub end: Option<String>,
}

impl ValidityPeriod {
    pub fn new(start: &str, end: Option<&str>) -> Result<Self, String> {
        if !valid_date(start) {
            return Err("Date de début invalide".into());
        }

        if let Some(end_date) = end {
            if !valid_date(end_date) {
                return Err("Date de fin invalide".into());
            }

            if end_date <= start {
                return Err("La fin doit être postérieure au début".into());
            }
        }

        Ok(Self {
            start: start.to_string(),
            end: end.map(str::to_string),
        })
    }

    pub fn contains(&self, date: &str) -> Result<bool, String> {
        if !valid_date(date) {
            return Err("Date recherchée invalide".into());
        }

        Ok(date >= self.start.as_str() && self.end.as_deref().is_none_or(|end| date < end))
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        let first_before_second_end = other
            .end
            .as_deref()
            .is_none_or(|end| self.start.as_str() < end);

        let second_before_first_end = self
            .end
            .as_deref()
            .is_none_or(|end| other.start.as_str() < end);

        first_before_second_end && second_before_first_end
    }
}

fn valid_date(date: &str) -> bool {
    let bytes = date.as_bytes();

    if bytes.len() != 10 || bytes[4] != b'-' || bytes[7] != b'-' {
        return false;
    }

    if !bytes
        .iter()
        .enumerate()
        .all(|(i, b)| i == 4 || i == 7 || b.is_ascii_digit())
    {
        return false;
    }

    let year: u32 = match date[0..4].parse() {
        Ok(value) => value,
        Err(_) => return false,
    };

    let month: u32 = match date[5..7].parse() {
        Ok(value) => value,
        Err(_) => return false,
    };

    let day: u32 = match date[8..10].parse() {
        Ok(value) => value,
        Err(_) => return false,
    };

    if year == 0 || !(1..=12).contains(&month) {
        return false;
    }

    let leap = year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400));

    let days = match month {
        2 if leap => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    };

    (1..=days).contains(&day)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_dates() {
        assert!(valid_date("2024-02-29"));
        assert!(valid_date("2000-02-29"));
        assert!(valid_date("2025-11-24"));
    }

    #[test]
    fn rejects_invalid_dates() {
        assert!(!valid_date("2025-02-29"));
        assert!(!valid_date("1900-02-29"));
        assert!(!valid_date("2025-13-01"));
        assert!(!valid_date("2025-04-31"));
        assert!(!valid_date("2025-1-01"));
    }

    #[test]
    fn period_contains_start_but_not_end() {
        let period = ValidityPeriod::new("2020-01-01", Some("2024-01-01")).unwrap();

        assert!(period.contains("2020-01-01").unwrap());
        assert!(period.contains("2023-12-31").unwrap());
        assert!(!period.contains("2024-01-01").unwrap());
    }

    #[test]
    fn adjacent_periods_do_not_overlap() {
        let first = ValidityPeriod::new("2020-01-01", Some("2024-01-01")).unwrap();

        let second = ValidityPeriod::new("2024-01-01", None).unwrap();

        assert!(!first.overlaps(&second));
    }

    #[test]
    fn overlapping_periods_are_detected() {
        let first = ValidityPeriod::new("2020-01-01", Some("2025-01-01")).unwrap();

        let second = ValidityPeriod::new("2024-01-01", None).unwrap();

        assert!(first.overlaps(&second));
    }

    #[test]
    fn rejects_reversed_period() {
        assert!(ValidityPeriod::new("2025-01-01", Some("2024-01-01")).is_err());
    }

    #[test]
    fn rejects_invalid_query_date() {
        let period = ValidityPeriod::new("2020-01-01", None).unwrap();

        assert!(period.contains("2025-02-29").is_err());
    }
}
