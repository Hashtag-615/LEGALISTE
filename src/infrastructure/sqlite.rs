//! Point unique d'exécution de `sqlite3` (CODE_POLICY, § 7, et D2).
//!
//! Chaque appel passe en mode `-safe`, s'arrête à la première erreur
//! (`-bail`) et active les clés étrangères avant toute transaction. Une
//! consultation ouvre la base en `-readonly` : toute écriture y échoue.
//! Les résultats sont lus au format `quote` (littéraux SQL), le seul qui
//! ne laisse aucune ambiguïté sur les séparateurs ; toute autre forme de
//! valeur est refusée.

use std::error::Error;
use std::fmt;
use std::path::Path;
use std::process::Command;

use crate::infrastructure::ProcessError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Access {
    ReadOnly,
    ReadWrite,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Null,
    Integer(i64),
    Text(String),
}

/// Exécute `script`, transmis par l'entrée standard, et rend la sortie
/// brute. En lecture seule, la base doit exister : `sqlite3` ne la crée
/// jamais.
pub fn execute(database: &Path, access: Access, script: &str) -> Result<String, SqliteError> {
    if access == Access::ReadOnly && !database.is_file() {
        return Err(SqliteError::Missing(database.display().to_string()));
    }

    let mut command = Command::new("sqlite3");
    command.args(["-batch", "-bail", "-safe", "-quote"]);
    if access == Access::ReadOnly {
        command.arg("-readonly");
    }
    command
        .args(["-cmd", "PRAGMA foreign_keys = ON;"])
        .arg(database);

    let output = super::run(&mut command, Some(script.as_bytes()))?;
    String::from_utf8(output.stdout).map_err(|_| SqliteError::NotUtf8)
}

/// Consultation en lecture seule ; lignes lues au format `quote`.
pub fn query(database: &Path, script: &str) -> Result<Vec<Vec<Value>>, SqliteError> {
    parse_rows(&execute(database, Access::ReadOnly, script)?)
}

/// Littéral SQL d'un texte : apostrophes doublées, caractères de
/// contrôle refusés, pour que la relecture au format `quote` reste
/// exacte.
pub fn literal(text: &str) -> Result<String, SqliteError> {
    if text.chars().any(char::is_control) {
        return Err(SqliteError::UnsupportedText(text.to_string()));
    }
    Ok(format!("'{}'", text.replace('\'', "''")))
}

pub(crate) fn parse_rows(output: &str) -> Result<Vec<Vec<Value>>, SqliteError> {
    let mut rows = Vec::new();
    let mut row = Vec::new();
    let mut rest = output;

    while !rest.is_empty() {
        let (value, after) = parse_value(rest)?;
        row.push(value);
        rest = match after.as_bytes().first() {
            Some(b',') => &after[1..],
            Some(b'\n') => {
                rows.push(std::mem::take(&mut row));
                &after[1..]
            }
            None => {
                rows.push(std::mem::take(&mut row));
                after
            }
            Some(_) => return Err(unparsable(after)),
        };
    }
    Ok(rows)
}

fn parse_value(text: &str) -> Result<(Value, &str), SqliteError> {
    if let Some(rest) = text.strip_prefix("NULL") {
        return Ok((Value::Null, rest));
    }

    if let Some(mut rest) = text.strip_prefix('\'') {
        let mut value = String::new();
        loop {
            let quote = rest.find('\'').ok_or_else(|| unparsable(text))?;
            value.push_str(&rest[..quote]);
            rest = &rest[quote + 1..];
            match rest.strip_prefix('\'') {
                Some(after) => {
                    value.push('\'');
                    rest = after;
                }
                None => return Ok((Value::Text(value), rest)),
            }
        }
    }

    let end = text
        .char_indices()
        .find(|&(i, c)| !(c.is_ascii_digit() || (i == 0 && c == '-')))
        .map_or(text.len(), |(i, _)| i);
    let integer = text[..end].parse().map_err(|_| unparsable(text))?;
    Ok((Value::Integer(integer), &text[end..]))
}

fn unparsable(text: &str) -> SqliteError {
    SqliteError::Unparsable(text.chars().take(60).collect())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SqliteError {
    Missing(String),
    Process(ProcessError),
    NotUtf8,
    Unparsable(String),
    UnsupportedText(String),
}

impl fmt::Display for SqliteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Missing(path) => write!(f, "Base absente : {path}"),
            Self::Process(error) => error.fmt(f),
            Self::NotUtf8 => f.write_str("Sortie de sqlite3 non conforme à UTF-8"),
            Self::Unparsable(text) => write!(f, "Valeur SQLite illisible : {text}"),
            Self::UnsupportedText(text) => {
                write!(f, "Texte à caractère de contrôle refusé : {text:?}")
            }
        }
    }
}

impl Error for SqliteError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Process(error) => Some(error),
            _ => None,
        }
    }
}

impl From<ProcessError> for SqliteError {
    fn from(error: ProcessError) -> Self {
        Self::Process(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_quoted_rows() {
        let output = "'a''b','x\ny',NULL,-12\n'',7\n";

        assert_eq!(
            parse_rows(output).unwrap(),
            [
                vec![
                    Value::Text("a'b".into()),
                    Value::Text("x\ny".into()),
                    Value::Null,
                    Value::Integer(-12),
                ],
                vec![Value::Text(String::new()), Value::Integer(7)],
            ]
        );
    }

    #[test]
    fn rejects_values_it_cannot_read_exactly() {
        for output in ["unistr('x\\u001fy')\n", "1.5\n", "X'00'\n", "'ouvert\n"] {
            assert!(parse_rows(output).is_err(), "{output:?} accepté");
        }
    }

    #[test]
    fn literal_escapes_quotes_and_refuses_control_characters() {
        assert_eq!(literal("l'article").unwrap(), "'l''article'");
        assert!(literal("a\u{1f}b").is_err());
    }

    #[test]
    fn in_memory_round_trip_through_sqlite3() {
        let script = format!("SELECT {}, NULL, 42;", literal("d'où").unwrap());
        let output = execute(Path::new(":memory:"), Access::ReadWrite, &script).unwrap();

        assert_eq!(
            parse_rows(&output).unwrap(),
            [vec![
                Value::Text("d'où".into()),
                Value::Null,
                Value::Integer(42)
            ]]
        );
    }

    #[test]
    fn read_only_access_never_creates_a_database() {
        assert_eq!(
            query(Path::new("absente.sqlite3"), "SELECT 1;"),
            Err(SqliteError::Missing("absente.sqlite3".into()))
        );
        assert!(!Path::new("absente.sqlite3").exists());
    }
}
