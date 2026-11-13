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
const INFRASTRUCTURE: [&str; 2] = [
    include_str!("../src/infrastructure/mod.rs"),
    include_str!("../src/infrastructure/legi_archive.rs"),
];

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
    assert_absent(
        &INFRASTRUCTURE,
        &["std::net", "File::create", "OpenOptions"],
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
