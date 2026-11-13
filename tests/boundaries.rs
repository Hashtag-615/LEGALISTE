//! Frontières vérifiées sur les sources : absence d'effet externe,
//! corpus sans méthode de modification, sens des dépendances
//! (`docs/ARCHITECTURE.md`, § 3). Ces contrôles textuels sont des
//! garde-fous, non des preuves : ils signalent toute régression visible.

const TEMPORAL: &str = include_str!("../src/temporal.rs");
const DOMAIN: [&str; 4] = [
    include_str!("../src/domain/mod.rs"),
    include_str!("../src/domain/legi.rs"),
    include_str!("../src/domain/provision.rs"),
    include_str!("../src/domain/anomaly.rs"),
];
const CORPUS: &str = include_str!("../src/corpus/mod.rs");
const SIMULATION: &str = include_str!("../src/simulation.rs");
const EXECUTION: [&str; 3] = [
    include_str!("../src/execution/mod.rs"),
    include_str!("../src/execution/tool.rs"),
    include_str!("../src/execution/skill.rs"),
];

fn library_sources() -> Vec<&'static str> {
    let mut sources = vec![TEMPORAL, CORPUS, SIMULATION];
    sources.extend(DOMAIN);
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
fn library_has_no_external_effect() {
    assert_absent(
        &library_sources(),
        &[
            "std::fs",
            "std::net",
            "std::process",
            "std::env",
            "std::time",
            "std::thread",
            "Command::",
            "File::",
            "SystemTime",
            "Instant::",
        ],
    );
}

#[test]
fn corpus_offers_no_mutation() {
    assert_absent(&[CORPUS], &["&mut self", "pub mut", "Cell<", "Mutex<"]);
    assert_absent(&library_sources(), &["&mut Corpus"]);
}

#[test]
fn execution_runs_no_dynamic_code() {
    assert_absent(&EXECUTION, &["dyn ", "unsafe"]);
}

#[test]
fn dependencies_point_inward() {
    assert_absent(&[TEMPORAL], &["crate::"]);
    assert_absent(
        &DOMAIN,
        &["crate::corpus", "crate::simulation", "crate::execution"],
    );
    assert_absent(&[CORPUS], &["crate::simulation", "crate::execution"]);
    assert_absent(&[SIMULATION], &["crate::corpus", "crate::execution"]);
}
