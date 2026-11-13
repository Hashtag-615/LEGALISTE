//! Enregistrement et relecture des textes LEGI dans une base au schéma
//! canonique (`docs/ARCHITECTURE.md`, D6).
//!
//! Import : la base est construite en entier, en une seule transaction,
//! dans un fichier provisoire voisin (`<base>.partial`) ; elle est
//! contrôlée, relue et comparée à l'extrait, puis publiée sous son nom
//! par lien physique, qui échoue si ce nom existe déjà. Une base
//! incomplète n'apparaît donc jamais sous le nom définitif, et aucune
//! base existante n'est remplacée.
//!
//! Relecture : en lecture seule, après contrôle d'intégrité, de clés
//! étrangères et de conformité. SQLite fournit les données publiées ;
//! résolution et anomalies restent au domaine.

// Taille justifiée : import et relecture forment un seul contrat,
// vérifié par aller-retour (l'import relit la base avant de la publier) ;
// erreurs et tests occupent plus du tiers du fichier.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;
use std::fs::{self, File};
use std::io;
use std::path::{Path, PathBuf};

use crate::corpus::legi_text::{ExtractError, TextExtract};
use crate::corpus::{Corpus, CorpusError, SourceRecord};
use crate::domain::legi::{self, LegiError, LegiId, LegiStatus};
use crate::domain::provision::{Provision, ProvisionError, ProvisionVersion};
use crate::infrastructure::schema::{self, SchemaError};
use crate::infrastructure::sqlite::{self, Access, SqliteError, Value, literal};
use crate::temporal::CivilDate;

/// Tables avec leur clé, pour la lecture du contenu logique.
const TABLES: [(&str, &str); 5] = [
    ("source_records", "sha256"),
    ("legal_texts", "id"),
    ("ingestion_events", "id"),
    ("provisions", "ingestion_id, id"),
    ("provision_versions", "ingestion_id, id"),
];

/// Construit une nouvelle base contenant `extracts` ; refuse toute base
/// ou tout fichier provisoire déjà présent.
pub fn import(database: &Path, extracts: &[TextExtract]) -> Result<(), StoreError> {
    let script = import_script(extracts)?;
    publish(database, &script, extracts)
}

fn publish(database: &Path, script: &str, extracts: &[TextExtract]) -> Result<(), StoreError> {
    if fs::symlink_metadata(database).is_ok() {
        return Err(StoreError::AlreadyExists(shown(database)));
    }
    let partial = partial_path(database)?;
    File::create_new(&partial).map_err(|error| match error.kind() {
        io::ErrorKind::AlreadyExists => StoreError::StalePartial(shown(&partial)),
        _ => io_error(&partial, &error),
    })?;

    let built = build(&partial, script, extracts);
    if built.is_err() {
        discard(&partial);
        return built;
    }

    if let Err(error) = fs::hard_link(&partial, database) {
        discard(&partial);
        return Err(match error.kind() {
            io::ErrorKind::AlreadyExists => StoreError::AlreadyExists(shown(database)),
            _ => io_error(database, &error),
        });
    }
    // La base est publiée ; le nom provisoire n'est plus qu'un second
    // lien vers le même fichier complet. S'il ne peut être retiré,
    // l'import suivant le signale (`StalePartial`) : sans conséquence
    // sur l'intégrité.
    let _ = fs::remove_file(&partial);
    Ok(())
}

/// Exécute le script en une transaction, puis exige que la base relue
/// redonne exactement chaque extrait.
fn build(partial: &Path, script: &str, extracts: &[TextExtract]) -> Result<(), StoreError> {
    sqlite::execute(partial, Access::ReadWrite, script)?;
    for extract in extracts {
        if load_text(partial, extract.text())? != *extract {
            return Err(StoreError::RoundTripMismatch(extract.text()));
        }
    }
    Ok(())
}

/// Retire le fichier provisoire créé par cet import, et son journal.
/// Un échec de suppression est sans conséquence sur l'intégrité : ce
/// fichier n'est jamais publié, et l'import suivant le signale.
fn discard(partial: &Path) {
    let mut journal = partial.as_os_str().to_os_string();
    journal.push("-journal");
    for path in [partial.to_path_buf(), PathBuf::from(journal)] {
        let _ = fs::remove_file(path);
    }
}

fn partial_path(database: &Path) -> Result<PathBuf, StoreError> {
    let mut name = database
        .file_name()
        .ok_or_else(|| StoreError::InvalidPath(shown(database)))?
        .to_os_string();
    name.push(".partial");
    Ok(database.with_file_name(name))
}

/// Script d'import complet : schéma canonique et données, encadrés par
/// une seule transaction. Les textes sont rangés par identifiant ; dates,
/// états et numéros sont écrits tels que publiés.
pub fn import_script(extracts: &[TextExtract]) -> Result<String, StoreError> {
    let mut ordered: Vec<&TextExtract> = extracts.iter().collect();
    ordered.sort_by_key(|extract| extract.text());
    if ordered.is_empty() {
        return Err(StoreError::NothingToImport);
    }
    if let Some(pair) = ordered.windows(2).find(|p| p[0].text() == p[1].text()) {
        return Err(StoreError::DuplicateText(pair[0].text()));
    }

    let mut sources: BTreeMap<&str, &SourceRecord> = BTreeMap::new();
    for source in ordered.iter().map(|extract| extract.corpus().source()) {
        let known = sources.insert(source.sha256(), source);
        if known.is_some_and(|k| (k.name(), k.state_date()) != (source.name(), source.state_date()))
        {
            return Err(StoreError::ConflictingSources(source.sha256().to_string()));
        }
    }

    let mut sql = format!("BEGIN IMMEDIATE;\n{}", schema::canonical_schema());
    for source in sources.values() {
        sql += &format!(
            "INSERT INTO source_records (sha256, name, state_date) VALUES ({}, {}, {});\n",
            literal(source.sha256())?,
            literal(source.name())?,
            literal(&source.state_date().to_string())?
        );
    }
    for (rank, extract) in ordered.iter().enumerate() {
        insert_text(&mut sql, rank + 1, extract)?;
    }
    sql += "COMMIT;\n";
    Ok(sql)
}

fn insert_text(sql: &mut String, event: usize, extract: &TextExtract) -> Result<(), StoreError> {
    let text = literal(&extract.text().to_string())?;
    let source = extract.corpus().source();
    *sql += &format!("INSERT INTO legal_texts (id) VALUES ({text});\n");
    *sql += &format!(
        "INSERT INTO ingestion_events (id, source_sha256, text_id, acquisition_date, \
         document_count) VALUES ({event}, {}, {text}, {}, {});\n",
        literal(source.sha256())?,
        literal(&source.acquisition_date().to_string())?,
        extract.document_count()
    );

    let without: BTreeSet<LegiId> = extract
        .versions_without_document()
        .iter()
        .copied()
        .collect();
    for provision in extract.corpus().provisions() {
        let key = provision.versions().iter().map(|v| v.id()).min();
        let key = literal(&key.ok_or(StoreError::EmptyProvision)?.to_string())?;
        *sql += &format!("INSERT INTO provisions (ingestion_id, id) VALUES ({event}, {key});\n");

        for version in provision.versions() {
            let (from, until) = legi::published_bounds(version.period());
            *sql += &format!(
                "INSERT INTO provision_versions (ingestion_id, provision_id, id, number, status, \
                 valid_from, valid_until, has_document) VALUES ({event}, {key}, {}, {}, {}, {}, \
                 {}, {});\n",
                literal(&version.id().to_string())?,
                literal(version.number())?,
                literal(version.status().map_or("", LegiStatus::as_str))?,
                literal(&from)?,
                literal(&until)?,
                u8::from(!without.contains(&version.id()))
            );
        }
    }
    Ok(())
}

/// Relit un texte, en lecture seule, après contrôle de la base.
pub fn load_text(database: &Path, text: LegiId) -> Result<TextExtract, StoreError> {
    schema::check(database)?;
    let events = sqlite::query(
        database,
        &format!(
            "SELECT e.id, s.name, s.state_date, e.acquisition_date, s.sha256, e.document_count, \
             (SELECT count(*) FROM provisions p WHERE p.ingestion_id = e.id) \
             FROM ingestion_events e JOIN source_records s ON s.sha256 = e.source_sha256 \
             WHERE e.text_id = {};",
            literal(&text.to_string())?
        ),
    )?;
    let event = match events.as_slice() {
        [event] => event,
        [] => return Err(StoreError::TextNotStored(text)),
        _ => return Err(StoreError::SeveralSources(text)),
    };
    let [
        event_id,
        name,
        state,
        acquired,
        sha256,
        documents,
        provision_count,
    ] = event.as_slice()
    else {
        return Err(unexpected(event));
    };
    let source = SourceRecord::new(
        text_of(name)?,
        date_of(state)?,
        date_of(acquired)?,
        text_of(sha256)?,
    )?;

    let rows = sqlite::query(
        database,
        &format!(
            "SELECT provision_id, id, number, status, valid_from, valid_until, has_document \
             FROM provision_versions WHERE ingestion_id = {} ORDER BY provision_id, id;",
            integer_of(event_id)?
        ),
    )?;
    let (provisions, without) = provisions(&rows)?;
    if provisions.len() != count_of(provision_count)? {
        return Err(StoreError::InconsistentStore(text));
    }

    let corpus = Corpus::new(source, provisions)?;
    Ok(TextExtract::from_parts(
        corpus,
        text,
        count_of(documents)?,
        without,
    )?)
}

/// Dispositions recomposées par les constructeurs du domaine, et
/// versions sans document, par identifiant croissant.
fn provisions(rows: &[Vec<Value>]) -> Result<(Vec<Provision>, Vec<LegiId>), StoreError> {
    let mut groups: BTreeMap<&str, Vec<ProvisionVersion>> = BTreeMap::new();
    let mut without = Vec::new();

    for row in rows {
        let [key, id, number, status, from, until, has_document] = row.as_slice() else {
            return Err(unexpected(row));
        };
        let id: LegiId = text_of(id)?.parse()?;
        let version = ProvisionVersion::new(
            id,
            text_of(number)?,
            LegiStatus::from_legi(text_of(status)?)?,
            legi::validity_period(text_of(from)?, text_of(until)?)?,
        )?;
        match integer_of(has_document)? {
            0 => without.push(id),
            1 => {}
            _ => return Err(unexpected(row)),
        }
        groups.entry(text_of(key)?).or_default().push(version);
    }

    let mut provisions = Vec::with_capacity(groups.len());
    for (key, versions) in groups {
        let provision = Provision::new(versions)?;
        let smallest = provision.versions().iter().map(|v| v.id()).min();
        if smallest.map(|id| id.to_string()).as_deref() != Some(key) {
            return Err(StoreError::InconsistentKey(key.to_string()));
        }
        provisions.push(provision);
    }
    without.sort();
    Ok((provisions, without))
}

/// Contenu logique : chaque table triée par sa clé, au format `quote`.
/// Deux bases au même contenu donnent le même texte, quelle que soit la
/// disposition physique de leurs pages.
pub fn logical_content(database: &Path) -> Result<String, StoreError> {
    schema::check(database)?;
    let script: String = TABLES
        .iter()
        .map(|(table, key)| format!("SELECT '{table}'; SELECT * FROM {table} ORDER BY {key};\n"))
        .collect();
    Ok(sqlite::execute(database, Access::ReadOnly, &script)?)
}

fn text_of(value: &Value) -> Result<&str, StoreError> {
    match value {
        Value::Text(text) => Ok(text),
        other => Err(StoreError::UnexpectedValue(format!("{other:?}"))),
    }
}

fn integer_of(value: &Value) -> Result<i64, StoreError> {
    match value {
        Value::Integer(integer) => Ok(*integer),
        other => Err(StoreError::UnexpectedValue(format!("{other:?}"))),
    }
}

fn count_of(value: &Value) -> Result<usize, StoreError> {
    usize::try_from(integer_of(value)?)
        .map_err(|_| StoreError::UnexpectedValue(format!("{value:?}")))
}

fn date_of(value: &Value) -> Result<CivilDate, StoreError> {
    Ok(text_of(value)?
        .parse::<CivilDate>()
        .map_err(LegiError::from)?)
}

fn unexpected(row: &[Value]) -> StoreError {
    StoreError::UnexpectedValue(format!("{row:?}"))
}

fn shown(path: &Path) -> String {
    path.display().to_string()
}

fn io_error(path: &Path, error: &io::Error) -> StoreError {
    StoreError::Io {
        path: shown(path),
        message: error.to_string(),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StoreError {
    NothingToImport,
    DuplicateText(LegiId),
    ConflictingSources(String),
    EmptyProvision,
    InvalidPath(String),
    AlreadyExists(String),
    StalePartial(String),
    Io { path: String, message: String },
    RoundTripMismatch(LegiId),
    TextNotStored(LegiId),
    SeveralSources(LegiId),
    InconsistentStore(LegiId),
    InconsistentKey(String),
    UnexpectedValue(String),
    Sqlite(SqliteError),
    Schema(SchemaError),
    Legi(LegiError),
    Provision(ProvisionError),
    Corpus(CorpusError),
    Extract(ExtractError),
}

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NothingToImport => f.write_str("Aucun texte à importer"),
            Self::DuplicateText(id) => write!(f, "Texte {id} fourni plusieurs fois"),
            Self::ConflictingSources(sha) => write!(f, "Source {sha} décrite de deux façons"),
            Self::EmptyProvision => f.write_str("Disposition sans version"),
            Self::InvalidPath(path) => write!(f, "Chemin de base invalide : {path}"),
            Self::AlreadyExists(path) => {
                write!(f, "Base déjà présente, jamais remplacée : {path}")
            }
            Self::StalePartial(path) => write!(
                f,
                "Fichier provisoire d'un import antérieur présent : {path} ; à examiner avant \
                 tout nouvel import"
            ),
            Self::Io { path, message } => write!(f, "{path} : {message}"),
            Self::RoundTripMismatch(id) => write!(f, "{id} : la base relue diffère de l'extrait"),
            Self::TextNotStored(id) => write!(f, "Texte {id} absent de la base"),
            Self::SeveralSources(id) => write!(f, "Texte {id} présent pour plusieurs sources"),
            Self::InconsistentStore(id) => write!(f, "{id} : dispositions sans version"),
            Self::InconsistentKey(key) => write!(f, "Clé de disposition incohérente : {key}"),
            Self::UnexpectedValue(value) => write!(f, "Valeur inattendue : {value}"),
            Self::Sqlite(error) => error.fmt(f),
            Self::Schema(error) => error.fmt(f),
            Self::Legi(error) => error.fmt(f),
            Self::Provision(error) => error.fmt(f),
            Self::Corpus(error) => error.fmt(f),
            Self::Extract(error) => error.fmt(f),
        }
    }
}

impl Error for StoreError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Sqlite(error) => Some(error),
            Self::Schema(error) => Some(error),
            Self::Legi(error) => Some(error),
            Self::Provision(error) => Some(error),
            Self::Corpus(error) => Some(error),
            Self::Extract(error) => Some(error),
            _ => None,
        }
    }
}

macro_rules! from_error {
    ($($error:ident => $variant:ident),* $(,)?) => {
        $(impl From<$error> for StoreError {
            fn from(error: $error) -> Self {
                Self::$variant(error)
            }
        })*
    };
}

from_error! {
    SqliteError => Sqlite,
    SchemaError => Schema,
    LegiError => Legi,
    ProvisionError => Provision,
    CorpusError => Corpus,
    ExtractError => Extract,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::corpus::legi_text;

    const SHA: &str = "90c9dc4a0a362943eef055881f00a727ac5d5f08fb9b4ec472e9da6da572bda2";

    /// Document synthétique minimal ; identifiants fictifs.
    fn extract() -> TextExtract {
        let document = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<ARTICLE><META>\
            <META_COMMUN><ID>LEGIARTI000000000001</ID>\
            <URL>article/LEGIARTI000000000001.xml</URL></META_COMMUN><META_SPEC>\
            <META_ARTICLE><NUM>1</NUM><ETAT>VIGUEUR</ETAT><DATE_DEBUT>2020-01-01</DATE_DEBUT>\
            <DATE_FIN>2999-01-01</DATE_FIN></META_ARTICLE></META_SPEC></META><CONTEXTE>\
            <TEXTE cid=\"LEGITEXT000000000009\"/></CONTEXTE><VERSIONS><VERSION etat=\"VIGUEUR\">\
            <LIEN_ART debut=\"2020-01-01\" etat=\"VIGUEUR\" fin=\"2999-01-01\" \
            id=\"LEGIARTI000000000001\" num=\"1\" origine=\"LEGI\"/></VERSION></VERSIONS>\
            </ARTICLE>\n";
        let date = |text: &str| text.parse().unwrap();
        let source =
            SourceRecord::new("essai", date("2026-11-08"), date("2026-11-13"), SHA).unwrap();
        legi_text::build(source, "LEGITEXT000000000009".parse().unwrap(), document).unwrap()
    }

    fn scratch(name: &str) -> PathBuf {
        let directory =
            std::env::temp_dir().join(format!("legaliste-store-{}-{name}", std::process::id()));
        fs::create_dir_all(&directory).unwrap();
        directory
    }

    #[test]
    fn failed_import_leaves_neither_base_nor_partial_file() {
        let directory = scratch("echec");
        let database = directory.join("base.sqlite3");
        let extract = extract();
        let script = import_script(std::slice::from_ref(&extract)).unwrap();
        let failing = script.replace(
            "COMMIT;\n",
            "INSERT INTO legal_texts (id) VALUES ('invalide');\nCOMMIT;\n",
        );

        let result = publish(&database, &failing, &[extract]);

        assert!(matches!(result, Err(StoreError::Sqlite(_))));
        assert!(!database.exists());
        assert!(!partial_path(&database).unwrap().exists());
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn import_then_load_gives_back_the_extract() {
        let directory = scratch("aller-retour");
        let database = directory.join("base.sqlite3");
        let extract = extract();

        import(&database, std::slice::from_ref(&extract)).unwrap();

        assert_eq!(load_text(&database, extract.text()).unwrap(), extract);
        fs::remove_dir_all(directory).unwrap();
    }
}
