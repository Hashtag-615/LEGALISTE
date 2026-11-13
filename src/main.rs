mod migrations;
mod temporal;

use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const DATA_DIR: &str = "/Volumes/STORAGE NVME/60_LEGALISTE_DATA";
const DB_NAME: &str = "legaliste.sqlite3";
const MIGRATION: &str = include_str!("../migrations/001_initial.sql");

fn database_path() -> PathBuf {
    Path::new(DATA_DIR).join(DB_NAME)
}

fn sqlite(db: &Path, sql: &str) -> Result<String, String> {
    let output = Command::new("sqlite3")
        .arg("-batch")
        .arg("-bail")
        .arg("-cmd")
        .arg("PRAGMA foreign_keys=ON;")
        .arg(db)
        .arg(sql)
        .stdin(Stdio::null())
        .output()
        .map_err(|e| format!("Impossible d'exécuter sqlite3 : {e}"))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn db_init() -> Result<(), String> {
    let directory = Path::new(DATA_DIR);

    if !directory.is_dir() {
        return Err(format!(
            "Répertoire de données absent : {}",
            directory.display()
        ));
    }

    let db = database_path();

    if db.exists() {
        return Err(format!(
            "Base déjà présente : {}. Initialisation refusée.",
            db.display()
        ));
    }

    let sql = format!(
        "BEGIN IMMEDIATE;\n{}\n\
         CREATE TABLE schema_migrations (\
         version INTEGER PRIMARY KEY,\
         applied_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP\
         );\n\
         INSERT INTO schema_migrations(version) VALUES (1);\n\
         COMMIT;",
        MIGRATION
    );

    match sqlite(&db, &sql) {
        Ok(_) => {
            println!("Base initialisée : {}", db.display());
            println!("Migration appliquée : 001_initial");
            Ok(())
        }
        Err(error) => Err(format!(
            "Initialisation échouée : {error}. Vérifier le fichier SQLite avant toute nouvelle tentative."
        )),
    }
}

fn db_status() -> Result<(), String> {
    let db = database_path();

    if !db.is_file() {
        return Err(format!("Base absente : {}", db.display()));
    }

    let sql = r#"
        PRAGMA integrity_check;
        PRAGMA foreign_key_check;
        SELECT 'schema_version=' || MAX(version) FROM schema_migrations;
        SELECT 'sources=' || COUNT(*) FROM legal_sources;
        SELECT 'textes=' || COUNT(*) FROM legal_texts;
        SELECT 'dispositions=' || COUNT(*) FROM provisions;
        SELECT 'versions=' || COUNT(*) FROM provision_versions;
    "#;

    let result = sqlite(&db, sql)?;
    println!("Base : {}", db.display());
    println!("{result}");

    if !result.lines().any(|line| line == "ok") {
        return Err("Contrôle d'intégrité non concluant.".into());
    }

    Ok(())
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().skip(1).collect();

    match args.as_slice() {
        [command] if command == "version" => {
            println!("LEGALISTE {}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
        [command] if command == "status" => {
            println!("LEGALISTE — moteur opérationnel");
            db_status()
        }
        [command, action] if command == "db" && action == "init" => db_init(),
        [command, action] if command == "db" && action == "status" => db_status(),
        _ => Err("Usage: legaliste <version|status|db init|db status>".into()),
    }
}

fn main() {
    if let Err(error) = run() {
        eprintln!("ERREUR : {error}");
        std::process::exit(1);
    }
}
