//! Définition canonique du schéma SQLite et contrôle de conformité
//! (`docs/ARCHITECTURE.md`, D6). Aucune table de versions, aucune
//! migration : une base est conforme si ses objets sont exactement ceux
//! que produit la définition canonique, sinon elle est refusée.
//!
//! Les colonnes de dates conservent les valeurs publiées par la source,
//! sentinelles comprises ; aucune contrainte n'ordonne début et fin, de
//! sorte que les périodes inversées ou vides sont conservées. Les
//! contraintes ne sont jamais plus permissives que le domaine : dates
//! civiles réelles, identifiants et états officiels.

use std::error::Error;
use std::fmt;
use std::path::Path;

use crate::domain::legi::{LegiIdKind, LegiStatus};
use crate::infrastructure::sqlite::{self, Access, SqliteError, Value};

/// Objets du schéma, dans l'ordre de `sqlite_schema` trié.
const OBJECTS_QUERY: &str =
    "SELECT type, name, tbl_name, sql FROM sqlite_schema ORDER BY type, name;";

pub fn canonical_schema() -> String {
    let digits = "[0-9]".repeat(12);
    let id = |column: &str, kinds: &[LegiIdKind]| {
        kinds
            .iter()
            .map(|kind| format!("{column} GLOB '{}{digits}'", kind.prefix()))
            .collect::<Vec<_>>()
            .join(" OR ")
    };
    let date = |column: &str| format!("{column} IS date({column}) AND {column} >= '0001-01-01'");
    let statuses: Vec<String> = LegiStatus::ALL
        .iter()
        .map(|status| format!("'{}'", status.as_str()))
        .chain(["''".to_string()])
        .collect();

    let article = id("id", &[LegiIdKind::Article]);
    let text = id("id", &[LegiIdKind::Text, LegiIdKind::JorfText]);
    let (state, acquisition) = (date("state_date"), date("acquisition_date"));
    let (from, until) = (date("valid_from"), date("valid_until"));
    let statuses = statuses.join(", ");

    format!(
        "CREATE TABLE source_records (
    sha256 TEXT NOT NULL PRIMARY KEY CHECK (length(sha256) = 64 AND sha256 NOT GLOB '*[^0-9a-f]*'),
    name TEXT NOT NULL CHECK (name <> ''),
    state_date TEXT NOT NULL CHECK ({state})
) STRICT;
CREATE TABLE legal_texts (
    id TEXT NOT NULL PRIMARY KEY CHECK ({text})
) STRICT;
CREATE TABLE ingestion_events (
    id INTEGER PRIMARY KEY CHECK (id > 0),
    source_sha256 TEXT NOT NULL REFERENCES source_records (sha256),
    text_id TEXT NOT NULL REFERENCES legal_texts (id),
    acquisition_date TEXT NOT NULL CHECK ({acquisition}),
    document_count INTEGER NOT NULL CHECK (document_count > 0),
    UNIQUE (source_sha256, text_id)
) STRICT;
CREATE TABLE provisions (
    ingestion_id INTEGER NOT NULL REFERENCES ingestion_events (id),
    id TEXT NOT NULL CHECK ({article}),
    PRIMARY KEY (ingestion_id, id)
) STRICT;
CREATE TABLE provision_versions (
    ingestion_id INTEGER NOT NULL,
    provision_id TEXT NOT NULL,
    id TEXT NOT NULL CHECK ({article}),
    number TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN ({statuses})),
    valid_from TEXT NOT NULL CHECK ({from}),
    valid_until TEXT NOT NULL CHECK ({until}),
    has_document INTEGER NOT NULL CHECK (has_document IN (0, 1)),
    PRIMARY KEY (ingestion_id, id),
    FOREIGN KEY (ingestion_id, provision_id) REFERENCES provisions (ingestion_id, id)
) STRICT;
"
    )
}

/// Contrôle d'une base, en lecture seule : intégrité, clés étrangères,
/// conformité au schéma canonique. Rien n'est corrigé ni complété.
pub fn check(database: &Path) -> Result<(), SchemaError> {
    let script = format!(
        "SELECT 'section', 'integrity'; PRAGMA integrity_check;
         SELECT 'section', 'foreign_keys'; PRAGMA foreign_key_check;
         SELECT 'section', 'objects'; {OBJECTS_QUERY}"
    );
    let rows = sqlite::query(database, &script)?;
    let [integrity, foreign_keys, objects] = sections(rows)?;

    if integrity != [vec![Value::Text("ok".into())]] {
        let messages = integrity.into_iter().flatten().map(|v| format!("{v:?}"));
        return Err(SchemaError::Integrity(messages.collect()));
    }
    if !foreign_keys.is_empty() {
        return Err(SchemaError::ForeignKeyViolations(foreign_keys.len()));
    }

    let expected = canonical_objects()?;
    if objects != expected {
        return Err(SchemaError::NonConform {
            missing: names(&expected, &objects),
            unexpected: names(&objects, &expected),
        });
    }
    Ok(())
}

/// Objets que produit la définition canonique, appliquée en mémoire.
fn canonical_objects() -> Result<Vec<Vec<Value>>, SchemaError> {
    let script = format!("{}{OBJECTS_QUERY}", canonical_schema());
    let output = sqlite::execute(Path::new(":memory:"), Access::ReadWrite, &script)?;
    Ok(sqlite::parse_rows(&output)?)
}

/// Découpe les résultats selon les lignes repères `'section', nom`.
fn sections(rows: Vec<Vec<Value>>) -> Result<[Vec<Vec<Value>>; 3], SchemaError> {
    let mut sections: Vec<Vec<Vec<Value>>> = Vec::new();
    for row in rows {
        if matches!(row.first(), Some(Value::Text(t)) if t == "section") && row.len() == 2 {
            sections.push(Vec::new());
        } else if let Some(current) = sections.last_mut() {
            current.push(row);
        }
    }
    sections
        .try_into()
        .map_err(|_| SchemaError::UnexpectedOutput)
}

/// Noms des objets de `left` absents ou différents dans `right`.
fn names(left: &[Vec<Value>], right: &[Vec<Value>]) -> Vec<String> {
    left.iter()
        .filter(|object| !right.contains(object))
        .filter_map(|object| match object.get(1) {
            Some(Value::Text(name)) => Some(name.clone()),
            _ => None,
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchemaError {
    Sqlite(SqliteError),
    UnexpectedOutput,
    Integrity(Vec<String>),
    ForeignKeyViolations(usize),
    /// Objets canoniques absents ou altérés, et objets étrangers.
    NonConform {
        missing: Vec<String>,
        unexpected: Vec<String>,
    },
}

impl fmt::Display for SchemaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sqlite(error) => error.fmt(f),
            Self::UnexpectedOutput => f.write_str("Sortie de contrôle inattendue"),
            Self::Integrity(messages) => {
                write!(f, "Intégrité compromise : {}", messages.join(" ; "))
            }
            Self::ForeignKeyViolations(count) => {
                write!(f, "{count} violation(s) de clé étrangère")
            }
            Self::NonConform {
                missing,
                unexpected,
            } => write!(
                f,
                "Schéma non conforme (D6 : base à reconstruire) ; absents ou altérés : [{}] ; \
                 étrangers : [{}]",
                missing.join(", "),
                unexpected.join(", ")
            ),
        }
    }
}

impl Error for SchemaError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Sqlite(error) => Some(error),
            _ => None,
        }
    }
}

impl From<SqliteError> for SchemaError {
    fn from(error: SqliteError) -> Self {
        Self::Sqlite(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::temporal::CivilDate;

    /// Les contraintes de date acceptent exactement ce qu'accepte le
    /// domaine (`docs/ARCHITECTURE.md`, § 5).
    #[test]
    fn date_constraint_matches_domain_validation() {
        let dates = [
            "2024-02-29",
            "2025-02-29",
            "2025-02-30",
            "1900-02-29",
            "2000-02-29",
            "0000-01-01",
            "0001-01-01",
            "2025-1-01",
            "2025-13-01",
            "2025-01-01 ",
            "2460000",
            "2999-01-01",
            "9999-12-31",
        ];
        let selects: Vec<String> = dates
            .iter()
            .map(|d| format!("SELECT ({d:?} IS date({d:?}) AND {d:?} >= '0001-01-01');"))
            .collect();
        let script = selects.join("\n").replace('"', "'");
        let output = sqlite::execute(Path::new(":memory:"), Access::ReadWrite, &script).unwrap();
        let accepted: Vec<bool> = sqlite::parse_rows(&output)
            .unwrap()
            .iter()
            .map(|row| row == &[Value::Integer(1)])
            .collect();

        let domain: Vec<bool> = dates
            .iter()
            .map(|d| d.parse::<CivilDate>().is_ok())
            .collect();
        assert_eq!(accepted, domain);
    }

    #[test]
    fn canonical_schema_applies_and_lists_five_tables() {
        let tables = canonical_objects()
            .unwrap()
            .into_iter()
            .filter(|object| object[0] == Value::Text("table".into()))
            .count();
        assert_eq!(tables, 5);
    }
}
