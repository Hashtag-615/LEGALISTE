//! Frontières vérifiées sur les sources : absence d'effet externe hors
//! d'`infrastructure`, corpus sans méthode de modification, sens des
//! dépendances (`docs/ARCHITECTURE.md`, § 3). Ces contrôles textuels sont
//! des garde-fous, non des preuves : ils signalent toute régression
//! visible.

const TEMPORAL: &str = include_str!("../src/temporal.rs");
const DOMAIN: [&str; 4] = [
    include_str!("../src/domain/mod.rs"),
    include_str!("../src/domain/legi.rs"),
    include_str!("../src/domain/provision.rs"),
    include_str!("../src/domain/anomaly.rs"),
];
/// Le premier fichier définit le type `Corpus`.
const CORPUS: [&str; 3] = [
    include_str!("../src/corpus/mod.rs"),
    include_str!("../src/corpus/legi_xml.rs"),
    include_str!("../src/corpus/legi_text.rs"),
];
const SIMULATION: &str = include_str!("../src/simulation.rs");
const EXECUTION: [&str; 3] = [
    include_str!("../src/execution/mod.rs"),
    include_str!("../src/execution/tool.rs"),
    include_str!("../src/execution/skill.rs"),
];
/// Ordre : processus, archive, sqlite3, schéma, import.
const INFRASTRUCTURE: [&str; 5] = [
    include_str!("../src/infrastructure/mod.rs"),
    include_str!("../src/infrastructure/legi_archive.rs"),
    include_str!("../src/infrastructure/sqlite.rs"),
    include_str!("../src/infrastructure/schema.rs"),
    include_str!("../src/infrastructure/legi_store.rs"),
];
const MAIN: &str = include_str!("../src/main.rs");

/// Sources sans entrée-sortie : tout sauf `infrastructure`.
fn pure_sources() -> Vec<&'static str> {
    let mut sources = vec![TEMPORAL, SIMULATION];
    sources.extend(DOMAIN);
    sources.extend(CORPUS);
    sources.extend(EXECUTION);
    sources
}

fn assert_absent(sources: &[&str], forbidden: &[&str]) {
    for source in sources {
        for token in forbidden {
            assert!(!source.contains(token), "« {token} » interdit ici");
        }
    }
}

#[test]
fn only_infrastructure_has_external_effects() {
    assert_absent(
        &pure_sources(),
        &[
            "std::fs",
            "std::net",
            "std::process",
            "std::env",
            "std::time",
            "std::thread",
            "std::io",
            "Command::",
            "File::",
            "SystemTime",
            "Instant::",
        ],
    );
    assert_absent(&INFRASTRUCTURE, &["std::net"]);
}

/// Seul l'import écrit sur disque, et uniquement dans un fichier neuf ;
/// l'archive n'est jamais ouverte en écriture.
#[test]
fn only_the_import_writes_files() {
    let writes = [
        "File::create",
        "OpenOptions",
        "fs::write",
        "hard_link",
        "remove_file",
    ];
    assert_absent(&INFRASTRUCTURE[..4], &writes);
    assert_absent(&[MAIN], &writes);
    assert!(INFRASTRUCTURE[4].contains("File::create_new"));
}

/// Un seul point d'exécution de `sqlite3` dans le crate, aucun SQL ni
/// processus dans `main.rs` (CODE_POLICY, § 2 et § 7).
#[test]
fn single_sqlite3_execution_point() {
    let mut sources = pure_sources();
    sources.extend(INFRASTRUCTURE);
    sources.push(MAIN);
    let launches = |program: &str| {
        let call = format!("Command::new(\"{program}\")");
        sources
            .iter()
            .map(|s| s.matches(&call).count())
            .sum::<usize>()
    };

    assert_eq!(launches("sqlite3"), 1);
    assert!(INFRASTRUCTURE[2].contains("Command::new(\"sqlite3\")"));
    assert_absent(
        &[MAIN],
        &["Command", "PRAGMA", "SELECT", "std::process::Command"],
    );
}

#[test]
fn corpus_offers_no_mutation() {
    assert_absent(&CORPUS[..1], &["&mut self", "pub mut", "Cell<", "Mutex<"]);
    let mut sources = pure_sources();
    sources.extend(INFRASTRUCTURE);
    assert_absent(&sources, &["&mut Corpus"]);
}

#[test]
fn execution_runs_no_dynamic_code() {
    assert_absent(&EXECUTION, &["dyn ", "unsafe"]);
}

#[test]
fn dependencies_point_inward() {
    let outer = [
        "crate::simulation",
        "crate::execution",
        "crate::infrastructure",
    ];

    assert_absent(&[TEMPORAL], &["crate::"]);
    assert_absent(&DOMAIN, &[&["crate::corpus"], &outer[..]].concat());
    assert_absent(&CORPUS, &outer);
    assert_absent(
        &[SIMULATION],
        &["crate::corpus", "crate::execution", "crate::infrastructure"],
    );
    assert_absent(&EXECUTION, &["crate::infrastructure"]);
    assert_absent(&INFRASTRUCTURE, &["crate::simulation", "crate::execution"]);
}
