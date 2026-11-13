//! Base SQLite canonique : import atomique, relecture en lecture seule,
//! refus des bases corrompues ou non conformes. Bases temporaires,
//! bâties sur les extraits authentiques (`tests/common/legi_extracts.rs`).

mod common;

use std::fs;
use std::path::Path;

use common::{CODE_CIVIL, Scratch, extracts_text, id};
use legaliste::domain::anomaly;
use legaliste::infrastructure::ProcessError;
use legaliste::infrastructure::legi_archive::sha256_file;
use legaliste::infrastructure::legi_store::{self, StoreError};
use legaliste::infrastructure::schema::{SchemaError, canonical_schema};
use legaliste::infrastructure::sqlite::{self, Access, SqliteError};

fn imported(scratch: &Scratch, name: &str) -> std::path::PathBuf {
    let database = scratch.file(name);
    legi_store::import(&database, &[extracts_text()]).unwrap();
    database
}

fn load(database: &Path) -> Result<legaliste::corpus::legi_text::TextExtract, StoreError> {
    legi_store::load_text(database, id(CODE_CIVIL))
}

fn write(database: &Path, script: &str) {
    sqlite::execute(database, Access::ReadWrite, script).unwrap();
}

#[test]
fn round_trip_preserves_versions_and_anomalies() {
    let scratch = Scratch::new("aller-retour");
    let database = imported(&scratch, "code_civil.sqlite3");
    let stored = load(&database).unwrap();
    let direct = extracts_text();

    assert_eq!(stored, direct);
    let source_date = stored.corpus().source().state_date();
    for (from_store, from_extract) in stored
        .corpus()
        .provisions()
        .iter()
        .zip(direct.corpus().provisions())
    {
        assert_eq!(
            anomaly::detect(from_store, source_date),
            anomaly::detect(from_extract, source_date)
        );
    }
}

#[test]
fn two_independent_imports_have_identical_logical_content() {
    let scratch = Scratch::new("deux-imports");
    let first = imported(&scratch, "premier.sqlite3");
    let second = imported(&scratch, "second.sqlite3");

    let content = legi_store::logical_content(&first).unwrap();
    assert_eq!(content, legi_store::logical_content(&second).unwrap());
    // Période inversée de l'art. 311-21, conservée telle que publiée.
    assert!(
        content.contains(
            "'LEGIARTI000006424849','311-21','MODIFIE_MORT_NE','2003-09-01','2003-06-19',1"
        )
    );
}

#[test]
fn consultation_refuses_writes_and_leaves_the_file_untouched() {
    let scratch = Scratch::new("lecture-seule");
    let database = imported(&scratch, "code_civil.sqlite3");
    let before = sha256_file(&database).unwrap();

    let write = sqlite::execute(
        &database,
        Access::ReadOnly,
        "INSERT INTO legal_texts (id) VALUES ('LEGITEXT000000000001');",
    );
    let Err(SqliteError::Process(ProcessError::Failed { stderr, .. })) = write else {
        panic!("écriture acceptée : {write:?}");
    };
    assert!(stderr.contains("readonly"), "{stderr}");

    load(&database).unwrap();
    assert_eq!(sha256_file(&database).unwrap(), before);
}

#[test]
fn never_replaces_an_existing_base() {
    let scratch = Scratch::new("existante");
    let database = imported(&scratch, "code_civil.sqlite3");
    let before = sha256_file(&database).unwrap();

    assert!(matches!(
        legi_store::import(&database, &[extracts_text()]),
        Err(StoreError::AlreadyExists(_))
    ));
    assert_eq!(sha256_file(&database).unwrap(), before);
}

#[test]
fn stale_partial_file_blocks_import_without_publishing() {
    let scratch = Scratch::new("provisoire");
    let database = scratch.file("code_civil.sqlite3");
    fs::write(scratch.file("code_civil.sqlite3.partial"), b"").unwrap();

    assert!(matches!(
        legi_store::import(&database, &[extracts_text()]),
        Err(StoreError::StalePartial(_))
    ));
    assert!(!database.exists());
}

/// Un import interrompu avant `COMMIT` ne laisse aucune donnée, et la
/// base qui en résulte est refusée comme non conforme.
#[test]
fn interrupted_import_is_rejected() {
    let scratch = Scratch::new("interruption");
    let database = scratch.file("interrompue.sqlite3");
    let script = legi_store::import_script(&[extracts_text()]).unwrap();
    let lines: Vec<&str> = script.lines().collect();
    let truncated = lines[..lines.len() * 2 / 3].join("\n");
    assert!(!truncated.contains("COMMIT"));

    write(&database, &truncated);

    assert!(matches!(
        load(&database),
        Err(StoreError::Schema(SchemaError::NonConform { .. }))
    ));
}

#[test]
fn corrupted_base_is_rejected() {
    let scratch = Scratch::new("corruption");
    let database = imported(&scratch, "code_civil.sqlite3");
    let mut bytes = fs::read(&database).unwrap();
    assert!(bytes.len() > 8192);

    let page = scratch.file("page.sqlite3");
    let mut damaged = bytes.clone();
    damaged[4096..4112].fill(0xFF);
    fs::write(&page, &damaged).unwrap();
    assert!(matches!(
        load(&page),
        Err(StoreError::Schema(
            SchemaError::Integrity(_) | SchemaError::Sqlite(_)
        ))
    ));

    bytes[..16].fill(0);
    let header = scratch.file("entete.sqlite3");
    fs::write(&header, &bytes).unwrap();
    assert!(matches!(
        load(&header),
        Err(StoreError::Schema(SchemaError::Sqlite(_)))
    ));
}

#[test]
fn non_conforming_schemas_are_rejected() {
    let scratch = Scratch::new("non-conforme");

    let former = scratch.file("ancienne.sqlite3");
    write(
        &former,
        "CREATE TABLE schema_migrations (version INTEGER PRIMARY KEY);",
    );
    let extra = scratch.file("ajout.sqlite3");
    write(
        &extra,
        &format!("{}CREATE TABLE notes (texte TEXT);", canonical_schema()),
    );
    let altered = scratch.file("alteree.sqlite3");
    write(&altered, &canonical_schema().replace(" STRICT;", ";"));

    for database in [former, extra, altered] {
        assert!(
            matches!(
                load(&database),
                Err(StoreError::Schema(SchemaError::NonConform { .. }))
            ),
            "{}",
            database.display()
        );
    }
}

#[test]
fn foreign_key_violation_is_rejected() {
    let scratch = Scratch::new("cles");
    let database = scratch.file("orpheline.sqlite3");
    write(
        &database,
        &format!(
            "{}PRAGMA foreign_keys = OFF;
             INSERT INTO provisions (ingestion_id, id) VALUES (7, 'LEGIARTI000006419307');",
            canonical_schema()
        ),
    );

    assert_eq!(
        load(&database),
        Err(StoreError::Schema(SchemaError::ForeignKeyViolations(1)))
    );
}

#[test]
fn empty_conforming_base_holds_no_text() {
    let scratch = Scratch::new("vide");
    let database = scratch.file("vide.sqlite3");
    write(&database, &canonical_schema());

    assert_eq!(
        load(&database),
        Err(StoreError::TextNotStored(id(CODE_CIVIL)))
    );
}

#[test]
fn schema_refuses_values_the_domain_refuses() {
    let scratch = Scratch::new("contraintes");
    let database = scratch.file("contraintes.sqlite3");
    write(&database, &canonical_schema());

    for insert in [
        "INSERT INTO source_records VALUES ('00', 'a', '2026-11-08');",
        "INSERT INTO legal_texts VALUES ('LEGIARTI000006419307');",
        "INSERT INTO source_records VALUES ('90c9dc4a0a362943eef055881f00a727ac5d5f08fb9b4ec472e9da6da572bda2', 'a', '2025-02-30');",
    ] {
        let result = sqlite::execute(&database, Access::ReadWrite, insert);
        assert!(result.is_err(), "accepté : {insert}");
    }
}
