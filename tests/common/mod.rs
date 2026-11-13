//! Fixtures partagées des tests d'intégration.
//!
//! Article 16-11 du Code civil : métadonnées publiques relevées dans la
//! liste `VERSIONS` de l'archive `Freemium_legi_global_20261108-140000`
//! (consultée le 2026-11-13). Elles éprouvent le moteur et n'attestent
//! pas du droit en vigueur.

// Motif : chaque fichier de `tests/` compile ce module séparément et
// n'en utilise qu'une partie.
#![allow(dead_code)]

pub mod legi_extracts;

use legaliste::corpus::{Corpus, SourceRecord};
use legaliste::domain::legi::{self, LegiId, LegiStatus};
use legaliste::domain::provision::{Provision, ProvisionVersion};
use legaliste::temporal::CivilDate;

pub type Record = (&'static str, &'static str, &'static str, &'static str);

/// `LIEN_ART` : identifiant, état, `debut`, `fin`.
pub const ARTICLE_16_11: [Record; 7] = [
    (
        "LEGIARTI000006419307",
        "MODIFIE",
        "1994-07-30",
        "2004-08-07",
    ),
    (
        "LEGIARTI000006419308",
        "MODIFIE",
        "2004-08-07",
        "2005-07-01",
    ),
    (
        "LEGIARTI000006419309",
        "MODIFIE",
        "2007-07-01",
        "2011-03-16",
    ),
    (
        "LEGIARTI000023711923",
        "MODIFIE",
        "2011-03-16",
        "2016-06-05",
    ),
    (
        "LEGIARTI000032656317",
        "MODIFIE",
        "2016-06-05",
        "2020-01-01",
    ),
    (
        "LEGIARTI000039367698",
        "MODIFIE",
        "2020-01-01",
        "2023-05-21",
    ),
    (
        "LEGIARTI000047569313",
        "VIGUEUR",
        "2023-05-21",
        "2999-01-01",
    ),
];

/// Provenance de l'archive, selon `MANIFESTE_SOURCES.csv`.
pub const ARCHIVE: &str = "Freemium_legi_global_20261108-140000.tar.gz";
pub const ARCHIVE_SHA256: &str = "90c9dc4a0a362943eef055881f00a727ac5d5f08fb9b4ec472e9da6da572bda2";
pub const SOURCE_DATE: &str = "2026-11-08";
pub const ACQUISITION_DATE: &str = "2026-11-13";

/// Dossier du Code civil dans l'archive (CID du texte).
pub const CODE_CIVIL: &str = "LEGITEXT000006070721";

/// Article réel d'un autre texte, absent de tout corpus du Code civil.
pub const FOREIGN_ARTICLE: &str = "LEGIARTI000006850357";

pub fn date(text: &str) -> CivilDate {
    text.parse().unwrap()
}

pub fn id(text: &str) -> LegiId {
    text.parse().unwrap()
}

pub fn provision(records: impl IntoIterator<Item = Record>) -> Provision {
    let versions = records
        .into_iter()
        .map(|(id, status, start, end)| {
            ProvisionVersion::new(
                id.parse().unwrap(),
                "16-11",
                LegiStatus::from_legi(status).unwrap(),
                legi::validity_period(start, end).unwrap(),
            )
            .unwrap()
        })
        .collect();

    Provision::new(versions).unwrap()
}

pub fn legi_source() -> SourceRecord {
    SourceRecord::new(
        ARCHIVE,
        date(SOURCE_DATE),
        date(ACQUISITION_DATE),
        ARCHIVE_SHA256,
    )
    .unwrap()
}

/// Corpus réduit à l'article 16-11, lu dans l'ordre donné.
pub fn corpus(records: impl IntoIterator<Item = Record>) -> Corpus {
    Corpus::new(legi_source(), vec![provision(records)]).unwrap()
}
