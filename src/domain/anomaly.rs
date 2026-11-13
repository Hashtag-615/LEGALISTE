//! Contrôles de cohérence temporelle des versions d'une disposition :
//! lacunes, chevauchements, états officiels absents ou contradictoires.
//!
//! Ces contrôles signalent ; ils ne corrigent rien et ne modifient
//! jamais l'applicabilité (`docs/ARCHITECTURE.md`, § 5).

use crate::domain::legi::{LegiId, LegiStatus};
use crate::domain::provision::Provision;
use crate::temporal::{CivilDate, ValidityPeriod};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemporalAnomaly {
    /// Aucune version en vigueur entre `from` (inclus) et `until` (exclu).
    Gap {
        from: CivilDate,
        until: CivilDate,
        after: LegiId,
        before: LegiId,
    },
    /// Deux versions en vigueur en même temps.
    Overlap { first: LegiId, second: LegiId },
    /// Fin de vigueur antérieure à l'entrée en vigueur dans la source :
    /// période conservée telle quelle, jamais applicable.
    InvertedPeriod(LegiId),
    /// Version dépourvue d'état officiel dans la source.
    MissingStatus(LegiId),
    /// État officiel contraire à la position de la version à la date
    /// de l'état de la source.
    StatusConflict {
        version: LegiId,
        status: LegiStatus,
        standing: Standing,
    },
}

/// Position d'une version par rapport à une date.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Standing {
    NotYetInForce,
    InForce,
    Ended,
}

impl Standing {
    fn of(period: ValidityPeriod, date: CivilDate) -> Self {
        if date < period.start() {
            Self::NotYetInForce
        } else if period.contains(date) {
            Self::InForce
        } else {
            Self::Ended
        }
    }
}

/// Anomalies d'une disposition, dans un ordre stable : lacunes,
/// chevauchements, puis, version par version, période inversée et
/// contrôles d'état. `source_date` est la date de l'état de la source,
/// seule date à laquelle l'état officiel est valable.
pub fn detect(provision: &Provision, source_date: CivilDate) -> Vec<TemporalAnomaly> {
    let versions = provision.versions();
    let mut anomalies = gaps(provision);

    for (i, first) in versions.iter().enumerate() {
        for second in &versions[i + 1..] {
            if first.period().overlaps(&second.period()) {
                anomalies.push(TemporalAnomaly::Overlap {
                    first: first.id(),
                    second: second.id(),
                });
            }
        }
    }

    for version in versions {
        if version.period().is_inverted() {
            anomalies.push(TemporalAnomaly::InvertedPeriod(version.id()));
        }

        let standing = Standing::of(version.period(), source_date);
        match version.status() {
            None => anomalies.push(TemporalAnomaly::MissingStatus(version.id())),
            Some(status) if contradicts(status, standing) => {
                anomalies.push(TemporalAnomaly::StatusConflict {
                    version: version.id(),
                    status,
                    standing,
                });
            }
            Some(_) => {}
        }
    }

    anomalies
}

/// Intervalles non couverts entre la première entrée en vigueur et la
/// dernière fin connue. Les périodes vides ne couvrent rien.
fn gaps(provision: &Provision) -> Vec<TemporalAnomaly> {
    let mut gaps = Vec::new();
    let mut versions = provision
        .versions()
        .iter()
        .filter(|v| !v.period().is_empty());
    let Some(first) = versions.next() else {
        return gaps;
    };

    // Fin la plus tardive atteinte jusqu'ici, et version qui l'atteint.
    let (mut reach, mut reached_by) = (first.period().end(), first.id());

    for version in versions {
        // Une vigueur sans fin couvre tout ce qui suit.
        let Some(end) = reach else {
            break;
        };

        let start = version.period().start();
        if start > end {
            gaps.push(TemporalAnomaly::Gap {
                from: end,
                until: start,
                after: reached_by,
                before: version.id(),
            });
        }

        if version.period().end().is_none_or(|later| later > end) {
            reach = version.period().end();
            reached_by = version.id();
        }
    }

    gaps
}

/// Contrôle de cohérence, non règle juridique : seules les
/// contradictions manifestes entre le nom de l'état et la position de
/// la version sont signalées. `MODIFIE_MORT_NE`, dont le régime n'est
/// pas établi, n'est pas contrôlé.
fn contradicts(status: LegiStatus, standing: Standing) -> bool {
    match status {
        LegiStatus::Vigueur | LegiStatus::AbrogeDiff => standing != Standing::InForce,
        LegiStatus::VigueurDiff => standing != Standing::NotYetInForce,
        LegiStatus::Modifie
        | LegiStatus::Abroge
        | LegiStatus::Transfere
        | LegiStatus::Perime
        | LegiStatus::Annule
        | LegiStatus::Disjoint
        | LegiStatus::Deplace => standing == Standing::InForce,
        LegiStatus::ModifieMortNe => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::provision::ProvisionVersion;
    use crate::domain::provision::fixtures::{date, id, version};

    const MODIFIED: Option<LegiStatus> = Some(LegiStatus::Modifie);

    #[test]
    fn detects_overlap_once_per_pair() {
        let provision = Provision::new(vec![
            version(1, MODIFIED, "2020-01-01", Some("2023-01-01")),
            version(2, MODIFIED, "2022-01-01", None),
        ])
        .unwrap();

        assert_eq!(
            detect(&provision, date("2019-01-01")),
            [TemporalAnomaly::Overlap {
                first: id(1),
                second: id(2),
            }]
        );
    }

    #[test]
    fn empty_period_does_not_bridge_a_gap() {
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
            detect(&provision, date("2019-01-01")),
            [TemporalAnomaly::Gap {
                from: date("2021-01-01"),
                until: date("2022-01-01"),
                after: id(1),
                before: id(3),
            }]
        );
    }

    #[test]
    fn open_version_covers_everything_after_it() {
        let provision = Provision::new(vec![
            version(1, MODIFIED, "2020-01-01", None),
            version(2, MODIFIED, "2030-01-01", Some("2031-01-01")),
        ])
        .unwrap();

        assert_eq!(
            detect(&provision, date("2019-01-01")),
            [TemporalAnomaly::Overlap {
                first: id(1),
                second: id(2),
            }]
        );
    }

    #[test]
    fn status_is_checked_against_source_date_only() {
        let provision = Provision::new(vec![
            version(
                1,
                Some(LegiStatus::Vigueur),
                "2020-01-01",
                Some("2021-01-01"),
            ),
            version(
                2,
                Some(LegiStatus::Abroge),
                "2021-01-01",
                Some("2030-01-01"),
            ),
            version(
                3,
                Some(LegiStatus::VigueurDiff),
                "2030-01-01",
                Some("2040-01-01"),
            ),
            version(4, None, "2040-01-01", None),
        ])
        .unwrap();

        assert_eq!(
            detect(&provision, date("2026-11-08")),
            [
                TemporalAnomaly::StatusConflict {
                    version: id(1),
                    status: LegiStatus::Vigueur,
                    standing: Standing::Ended,
                },
                TemporalAnomaly::StatusConflict {
                    version: id(2),
                    status: LegiStatus::Abroge,
                    standing: Standing::InForce,
                },
                TemporalAnomaly::MissingStatus(id(4)),
            ]
        );
    }

    #[test]
    fn coherent_statuses_raise_no_conflict() {
        let provision = Provision::new(vec![
            version(1, MODIFIED, "2020-01-01", Some("2024-01-01")),
            version(
                2,
                Some(LegiStatus::AbrogeDiff),
                "2024-01-01",
                Some("2029-01-01"),
            ),
            version(3, Some(LegiStatus::VigueurDiff), "2029-01-01", None),
            version(
                4,
                Some(LegiStatus::ModifieMortNe),
                "2025-01-01",
                Some("2025-01-01"),
            ),
        ])
        .unwrap();

        assert!(detect(&provision, date("2026-11-08")).is_empty());
    }

    #[test]
    fn inverted_period_is_reported_and_never_overlaps() {
        let inverted = ValidityPeriod::as_published(date("1994-03-01"), Some(date("1993-09-02")));
        let stillborn =
            ProvisionVersion::new(id(2), "1", Some(LegiStatus::ModifieMortNe), inverted);
        let provision = Provision::new(vec![
            version(1, MODIFIED, "1990-01-01", Some("2000-01-01")),
            stillborn.unwrap(),
        ])
        .unwrap();

        assert_eq!(
            detect(&provision, date("2026-11-08")),
            [TemporalAnomaly::InvertedPeriod(id(2))]
        );
    }
}
