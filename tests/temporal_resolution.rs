//! Résolution temporelle d'un article sur un cas réel : l'article
//! 16-11 du Code civil (fixture décrite dans `tests/common/mod.rs`).

mod common;

use common::{ARTICLE_16_11, SOURCE_DATE, date, id, provision};
use legaliste::domain::anomaly::{self, TemporalAnomaly};
use legaliste::domain::legi::LegiId;
use legaliste::domain::provision::{Provision, Resolution};

fn applicable_id(provision: &Provision, reference_date: &str) -> LegiId {
    match provision.resolve(date(reference_date)) {
        Resolution::Applicable(version) => version.id(),
        other => panic!("version applicable attendue au {reference_date} : {other:?}"),
    }
}

#[test]
fn resolves_version_in_force_at_reference_date() {
    let article = provision(ARTICLE_16_11);

    assert_eq!(
        applicable_id(&article, "2010-06-15"),
        id("LEGIARTI000006419309")
    );
    assert_eq!(
        applicable_id(&article, "2023-05-20"),
        id("LEGIARTI000039367698")
    );
    assert_eq!(
        applicable_id(&article, "2023-05-21"),
        id("LEGIARTI000047569313")
    );
    assert_eq!(
        applicable_id(&article, "2026-11-13"),
        id("LEGIARTI000047569313")
    );
}

#[test]
fn reports_no_version_inside_official_gap() {
    let article = provision(ARTICLE_16_11);

    let Resolution::NoneApplicable { previous, next } = article.resolve(date("2006-01-01")) else {
        panic!("aucune version ne devrait être applicable");
    };
    assert_eq!(previous.map(|v| v.id()), Some(id("LEGIARTI000006419308")));
    assert_eq!(next.map(|v| v.id()), Some(id("LEGIARTI000006419309")));
}

#[test]
fn reports_no_version_before_first_entry_into_force() {
    let article = provision(ARTICLE_16_11);

    let Resolution::NoneApplicable { previous, next } = article.resolve(date("1994-07-29")) else {
        panic!("aucune version ne devrait être applicable");
    };
    assert_eq!(previous, None);
    assert_eq!(next.map(|v| v.id()), Some(id("LEGIARTI000006419307")));
}

#[test]
fn detects_gap_and_nothing_else() {
    let article = provision(ARTICLE_16_11);

    assert_eq!(
        anomaly::detect(&article, date(SOURCE_DATE)),
        [TemporalAnomaly::Gap {
            from: date("2005-07-01"),
            until: date("2007-07-01"),
            after: id("LEGIARTI000006419308"),
            before: id("LEGIARTI000006419309"),
        }]
    );
}

#[test]
fn same_source_and_date_give_same_result_whatever_the_reading_order() {
    let forward = provision(ARTICLE_16_11);
    let backward = provision(ARTICLE_16_11.into_iter().rev());

    assert_eq!(forward, backward);
    for reference_date in ["1990-01-01", "2006-01-01", "2010-06-15", "2026-11-13"] {
        assert_eq!(
            forward.resolve(date(reference_date)),
            backward.resolve(date(reference_date))
        );
    }
    assert_eq!(
        anomaly::detect(&forward, date(SOURCE_DATE)),
        anomaly::detect(&backward, date(SOURCE_DATE))
    );
}
