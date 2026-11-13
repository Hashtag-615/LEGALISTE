use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const DATA_DIR: &str = "/Volumes/STORAGE NVME/60_LEGALISTE_DATA";
const DB_NAME: &str = "legaliste.sqlite3";

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

fn db_status() -> Result<(), String> {
    let db = database_path();

    if !db.is_file() {
        return Err(format!("Base absente : {}", db.display()));
    }

    let result = sqlite(&db, "PRAGMA integrity_check; PRAGMA foreign_key_check;")?;

    println!("Base : {}", db.display());
    println!("{result}");

    if result.lines().next() != Some("ok") {
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
