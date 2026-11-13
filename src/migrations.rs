use std::process::{Command, Stdio};

const MIGRATION_002: &str = include_str!("../migrations/002_temporal_guards.sql");

pub fn migration_002_script() -> String {
    format!(
        "PRAGMA foreign_keys = ON;\n\
         BEGIN IMMEDIATE;\n\
         {MIGRATION_002}\n\
         INSERT INTO schema_migrations(version) VALUES (2);\n\
         COMMIT;\n"
    )
}

pub fn sqlite_script(database: &std::path::Path, script: &str) -> Result<String, String> {
    use std::io::Write;

    let mut child = Command::new("sqlite3")
        .arg("-batch")
        .arg("-bail")
        .arg(database)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Impossible de lancer sqlite3 : {e}"))?;

    child
        .stdin
        .take()
        .ok_or("Entrée standard SQLite indisponible.")?
        .write_all(script.as_bytes())
        .map_err(|e| format!("Écriture du script impossible : {e}"))?;

    let output = child
        .wait_with_output()
        .map_err(|e| format!("Exécution SQLite impossible : {e}"))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migration_contains_transaction() {
        let script = migration_002_script();

        assert!(script.contains("BEGIN IMMEDIATE;"));
        assert!(script.contains("COMMIT;"));
        assert!(script.contains("INSERT INTO schema_migrations(version) VALUES (2)"));
    }

    #[test]
    fn migration_contains_both_triggers() {
        let script = migration_002_script();

        assert!(script.contains("prevent_version_overlap_insert"));
        assert!(script.contains("prevent_version_overlap_update"));
    }

    #[test]
    fn migration_executes_on_temporary_database() {
        use std::fs;
        use std::time::{SystemTime, UNIX_EPOCH};

        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let database = std::env::temp_dir().join(format!(
            "legaliste_migration_{}_{}.sqlite3",
            std::process::id(),
            stamp
        ));

        let initial = include_str!("../migrations/001_initial.sql");

        let setup = format!(
            "PRAGMA foreign_keys = ON;\n\
             BEGIN IMMEDIATE;\n\
             {initial}\n\
             COMMIT;\n"
        );

        let result = (|| -> Result<(), String> {
            sqlite_script(&database, &setup)?;

            let before = sqlite_script(
                &database,
                "SELECT version FROM schema_migrations ORDER BY version;",
            )?;

            if before.trim() != "1" {
                return Err(format!("Version initiale inattendue : {before:?}"));
            }

            sqlite_script(&database, &migration_002_script())?;

            let after = sqlite_script(
                &database,
                "SELECT version FROM schema_migrations ORDER BY version;",
            )?;

            if after.trim() != "1\n2" {
                return Err(format!("Versions inattendues : {after:?}"));
            }

            let triggers = sqlite_script(
                &database,
                "SELECT name FROM sqlite_master \
                 WHERE type = 'trigger' ORDER BY name;",
            )?;

            if !triggers.contains("prevent_version_overlap_insert")
                || !triggers.contains("prevent_version_overlap_update")
            {
                return Err(format!("Déclencheurs manquants : {triggers:?}"));
            }

            let integrity = sqlite_script(&database, "PRAGMA integrity_check;")?;

            if integrity.trim() != "ok" {
                return Err(format!("Intégrité invalide : {integrity:?}"));
            }

            Ok(())
        })();

        let _ = fs::remove_file(&database);

        if let Err(error) = result {
            panic!("{error}");
        }
    }

    #[test]
    fn migration_failure_rolls_back() {
        use std::fs;
        use std::time::{SystemTime, UNIX_EPOCH};

        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let database = std::env::temp_dir().join(format!(
            "legaliste_rollback_{}_{}.sqlite3",
            std::process::id(),
            stamp
        ));

        let initial = include_str!("../migrations/001_initial.sql");

        let setup = format!(
            "PRAGMA foreign_keys = ON;\n\
             BEGIN IMMEDIATE;\n\
             {initial}\n\
             COMMIT;\n"
        );

        let result = (|| -> Result<(), String> {
            sqlite_script(&database, &setup)?;

            let faulty = format!(
                "PRAGMA foreign_keys = ON;\n\
                 BEGIN IMMEDIATE;\n\
                 {}\n\
                 INSERT INTO schema_migrations(version) VALUES (2);\n\
                 INSERT INTO nonexistent_table(value) VALUES (1);\n\
                 COMMIT;\n",
                include_str!("../migrations/002_temporal_guards.sql")
            );

            if sqlite_script(&database, &faulty).is_ok() {
                return Err("Une migration volontairement invalide a réussi.".into());
            }

            let versions = sqlite_script(
                &database,
                "SELECT version FROM schema_migrations ORDER BY version;",
            )?;

            if versions.trim() != "1" {
                return Err(format!("Rollback incomplet : {versions:?}"));
            }

            let triggers = sqlite_script(
                &database,
                "SELECT COUNT(*) FROM sqlite_master WHERE type = 'trigger';",
            )?;

            if triggers.trim() != "0" {
                return Err(format!("Déclencheurs persistants : {triggers:?}"));
            }

            Ok(())
        })();

        let _ = fs::remove_file(&database);

        if let Err(error) = result {
            panic!("{error}");
        }
    }
}
