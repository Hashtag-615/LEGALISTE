use std::env;
use std::path::{Path, PathBuf};

use legaliste::infrastructure::schema;

const DATA_DIR: &str = "/Volumes/STORAGE NVME/60_LEGALISTE_DATA";
const DB_NAME: &str = "legaliste.sqlite3";

fn database_path() -> PathBuf {
    Path::new(DATA_DIR).join(DB_NAME)
}

/// Contrôle en lecture seule : intégrité, clés étrangères, conformité
/// au schéma canonique. Toute anomalie fait échouer la commande.
fn db_status() -> Result<(), String> {
    let db = database_path();
    println!("Base : {}", db.display());
    schema::check(&db).map_err(|error| error.to_string())?;
    println!("Intégrité : ok ; clés étrangères : aucune violation ; schéma : conforme");
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
            db_status()?;
            println!("LEGALISTE — base accessible et intègre");
            Ok(())
        }
        [command, action] if command == "db" && action == "status" => db_status(),
        _ => Err("Usage: legaliste <version|status|db status>".into()),
    }
}

fn main() {
    if let Err(error) = run() {
        eprintln!("ERREUR : {error}");
        std::process::exit(1);
    }
}
