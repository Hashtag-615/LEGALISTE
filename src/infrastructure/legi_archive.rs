//! Lecture de l'archive LEGI, strictement en lecture seule, par les
//! programmes du système (`docs/ARCHITECTURE.md`, D10) :
//!
//! - `tar` (bsdtar, option `--include`) extrait vers sa sortie standard
//!   (`-O`), lue en mémoire : rien n'est écrit sur disque ;
//! - `shasum -a 256` calcule les empreintes de provenance.
//!
//! L'archive est un flux gzip : toute lecture la parcourt en entier.

use std::error::Error;
use std::ffi::OsString;
use std::fmt;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Output, Stdio};

use crate::corpus::SourceRecord;
use crate::corpus::legi_text::{self, ExtractError, TextExtract};
use crate::domain::legi::LegiId;

/// Vérifie la provenance de l'archive, puis construit le corpus du
/// texte `text` (identifiant de dossier : CID du texte).
pub fn load_text(
    archive: &Path,
    source: SourceRecord,
    text: LegiId,
) -> Result<TextExtract, ArchiveError> {
    verify_provenance(archive, &source)?;
    let stream = read_text_articles(archive, text)?;
    Ok(legi_text::build(source, text, &stream)?)
}

/// L'archive doit porter le nom et l'empreinte enregistrés.
pub fn verify_provenance(archive: &Path, source: &SourceRecord) -> Result<(), ArchiveError> {
    let name = archive
        .file_name()
        .map(|name| name.to_string_lossy().into_owned());
    if name.as_deref() != Some(source.name()) {
        return Err(ArchiveError::NameMismatch {
            expected: source.name().to_string(),
            found: name.unwrap_or_default(),
        });
    }

    let digest = sha256_file(archive)?;
    if digest != source.sha256() {
        return Err(ArchiveError::DigestMismatch {
            expected: source.sha256().to_string(),
            found: digest,
        });
    }
    Ok(())
}

/// Documents d'article du texte, concaténés dans l'ordre de l'archive.
pub fn read_text_articles(archive: &Path, text: LegiId) -> Result<String, ArchiveError> {
    let output = run(Command::new("tar").args(tar_arguments(archive, text)), None)?;
    String::from_utf8(output.stdout).map_err(|_| ArchiveError::NotUtf8)
}

pub fn sha256_file(path: &Path) -> Result<String, ArchiveError> {
    let mut command = Command::new("shasum");
    command.args(["-a", "256", "--"]).arg(path);
    digest(run(&mut command, None)?)
}

pub fn sha256_bytes(bytes: &[u8]) -> Result<String, ArchiveError> {
    digest(run(
        Command::new("shasum").args(["-a", "256"]),
        Some(bytes),
    )?)
}

/// Seul endroit où les arguments de `tar` sont fixés : extraction vers
/// la sortie standard, jamais vers un répertoire.
fn tar_arguments(archive: &Path, text: LegiId) -> Vec<OsString> {
    vec![
        "-x".into(),
        "-z".into(),
        "-O".into(),
        "-f".into(),
        archive.into(),
        "--include".into(),
        format!("*/{text}/article/*.xml").into(),
    ]
}

fn run(command: &mut Command, input: Option<&[u8]>) -> Result<Output, ArchiveError> {
    let program = command.get_program().to_string_lossy().into_owned();
    let spawn_error = |error: std::io::Error| ArchiveError::Spawn {
        program: program.clone(),
        message: error.to_string(),
    };

    let mut child = command
        .stdin(if input.is_some() {
            Stdio::piped()
        } else {
            Stdio::null()
        })
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(spawn_error)?;

    if let (Some(bytes), Some(mut stdin)) = (input, child.stdin.take()) {
        // L'entrée est écrite en entier puis fermée avant toute lecture :
        // `shasum` ne produit sa sortie qu'en fin d'entrée.
        stdin.write_all(bytes).map_err(spawn_error)?;
    }

    let output = child.wait_with_output().map_err(spawn_error)?;
    if !output.status.success() {
        return Err(ArchiveError::Failed {
            program,
            status: output.status.code(),
            stderr: String::from_utf8_lossy(&output.stderr).trim().to_string(),
        });
    }
    Ok(output)
}

fn digest(output: Output) -> Result<String, ArchiveError> {
    let text = String::from_utf8_lossy(&output.stdout);
    let digest = text.split_whitespace().next().unwrap_or_default();
    let hex = |b: u8| b.is_ascii_digit() || (b'a'..=b'f').contains(&b);

    if digest.len() == 64 && digest.bytes().all(hex) {
        Ok(digest.to_string())
    } else {
        Err(ArchiveError::UnexpectedDigest(text.trim().to_string()))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArchiveError {
    Spawn {
        program: String,
        message: String,
    },
    Failed {
        program: String,
        status: Option<i32>,
        stderr: String,
    },
    NotUtf8,
    UnexpectedDigest(String),
    NameMismatch {
        expected: String,
        found: String,
    },
    DigestMismatch {
        expected: String,
        found: String,
    },
    Extract(ExtractError),
}

impl fmt::Display for ArchiveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Spawn { program, message } => {
                write!(f, "Impossible d'exécuter {program} : {message}")
            }
            Self::Failed {
                program,
                status,
                stderr,
            } => write!(f, "{program} a échoué ({status:?}) : {stderr}"),
            Self::NotUtf8 => f.write_str("Documents de l'archive non conformes à UTF-8"),
            Self::UnexpectedDigest(text) => write!(f, "Empreinte illisible : {text}"),
            Self::NameMismatch { expected, found } => {
                write!(f, "Archive « {found} » au lieu de « {expected} »")
            }
            Self::DigestMismatch { expected, found } => {
                write!(f, "Empreinte {found} au lieu de {expected}")
            }
            Self::Extract(error) => error.fmt(f),
        }
    }
}

impl Error for ArchiveError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Extract(error) => Some(error),
            _ => None,
        }
    }
}

impl From<ExtractError> for ArchiveError {
    fn from(error: ExtractError) -> Self {
        Self::Extract(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tar_extracts_to_standard_output_only() {
        let text = "LEGITEXT000006070721".parse().unwrap();
        let arguments = tar_arguments(Path::new("archive.tar.gz"), text);

        assert_eq!(arguments[..4], ["-x", "-z", "-O", "-f"]);
        assert!(!arguments.iter().any(|a| a == "-C" || a == "--directory"));
        assert_eq!(
            arguments.last().unwrap(),
            "*/LEGITEXT000006070721/article/*.xml"
        );
    }

    #[test]
    fn computes_reference_digest() {
        assert_eq!(
            sha256_bytes(b"abc").unwrap(),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn missing_archive_fails_with_tar_message() {
        let text = "LEGITEXT000006070721".parse().unwrap();
        let error = read_text_articles(Path::new("absente.tar.gz"), text).unwrap_err();

        assert!(matches!(error, ArchiveError::Failed { ref program, .. } if program == "tar"));
    }
}
