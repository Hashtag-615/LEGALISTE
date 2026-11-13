//! Accès au système : programmes externes et fichiers. Seule couche
//! autorisée à lancer un processus (`docs/ARCHITECTURE.md`, D2 et D10).

pub mod legi_archive;
pub mod legi_store;
pub mod schema;
pub mod sqlite;

use std::error::Error;
use std::fmt;
use std::io::{self, Write};
use std::process::{Command, Output, Stdio};
use std::thread;

/// Échec d'un programme du système ; son message est conservé tel quel.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcessError {
    Spawn {
        program: String,
        message: String,
    },
    Failed {
        program: String,
        status: Option<i32>,
        stderr: String,
    },
}

impl fmt::Display for ProcessError {
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
        }
    }
}

impl Error for ProcessError {}

/// Lance `command`, lui transmet `input` puis ferme son entrée. L'entrée
/// est écrite par un fil distinct : un programme qui écrit avant d'avoir
/// tout lu ne peut pas bloquer l'échange.
pub(crate) fn run(command: &mut Command, input: Option<&[u8]>) -> Result<Output, ProcessError> {
    let program = command.get_program().to_string_lossy().into_owned();
    let spawn_error = |error: io::Error| ProcessError::Spawn {
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
    let stdin = child.stdin.take();

    let (written, output) = thread::scope(|scope| {
        let writer = scope.spawn(move || match (input, stdin) {
            (Some(bytes), Some(mut stdin)) => stdin.write_all(bytes),
            _ => Ok(()),
        });
        let output = child.wait_with_output();
        let written = writer
            .join()
            .unwrap_or_else(|_| Err(io::Error::other("écriture de l'entrée interrompue")));
        (written, output)
    });

    let output = output.map_err(spawn_error)?;
    if !output.status.success() {
        return Err(ProcessError::Failed {
            program,
            status: output.status.code(),
            stderr: String::from_utf8_lossy(&output.stderr).trim().to_string(),
        });
    }
    written.map_err(spawn_error)?;
    Ok(output)
}
