# LEGALISTE — Architecture

## 1. Principes

- Crate unique `legaliste` : une bibliothèque (`src/lib.rs`) et un
  binaire (`src/main.rs`).
- Séparation en plusieurs crates seulement sur besoin technique
  démontré : consommateur distinct, dépendances incompatibles ou
  temps de compilation mesuré.
- Une couche n'a de module qu'à partir de son premier contenu réel.
- Les règles d'écriture figurent dans `docs/CODE_POLICY.md` ; les
  zones de données dans `docs/DATA_BOUNDARIES.md`.

## 2. Couches

| Couche           | Responsabilité | Exclu |
|------------------|----------------|-------|
| `domain`         | Types et invariants juridiques : sources, textes, dispositions, versions, périodes de validité, provenance, consultation à une date. | Entrées-sorties, SQL, processus, variables d'environnement, horloge système. |
| `infrastructure` | Exécution de `sqlite3`, schéma, migrations, sauvegardes, fichiers ; conversion entre lignes SQL et types du domaine. | Règles juridiques, présentation, chemins par défaut. |
| `application`    | Cas d'utilisation combinant domaine et stockage. | SQL, présentation. |
| `cli`            | Lecture des arguments, choix de la commande, résolution des chemins par défaut, formatage des sorties, conversion des erreurs. | SQL, règles juridiques. |
| `main.rs`        | Appel de la CLI, écriture de l'erreur, code de retour. | Toute autre logique. |

La couche `application` n'existe pas encore. Les commandes
d'administration de la base (`db init`, `db status`, `db migrate`)
ne mobilisent que le stockage : la CLI les adresse directement à
`infrastructure`. La couche `application` sera créée avec le premier
cas d'utilisation combinant domaine et stockage, par exemple la
consultation d'une disposition à une date.

## 3. Sens des dépendances

Dépendances autorisées :

- `main.rs` → `cli`
- `cli` → `application`, `infrastructure`, `domain`
- `application` → `infrastructure`, `domain`
- `infrastructure` → `domain`
- `domain` → bibliothèque standard uniquement

Interdits :

- toute dépendance remontante : `domain` vers une autre couche,
  `infrastructure` vers `application` ou `cli`, `application`
  vers `cli` ;
- tout cycle entre modules ;
- tout usage de `main.rs` par un autre module.

Ces règles sont vérifiables par recherche des chemins `crate::`
importés dans chaque couche.

## 4. Carte des modules

Cible indicative. Un fichier n'est créé qu'avec son premier contenu
réel. Les répertoires utilisent `mod.rs`, comme `tests/common/`.

```text
src/main.rs                        point d'entrée
src/lib.rs                         déclaration des modules
src/cli.rs                         interface opérateur
src/domain/mod.rs                  types juridiques
src/domain/temporal.rs             périodes de validité
src/infrastructure/mod.rs
src/infrastructure/sqlite.rs       point unique d'exécution de sqlite3
src/infrastructure/schema.rs       initialisation et état de la base
src/infrastructure/migrations.rs   contrôle et application des migrations
src/infrastructure/backup.rs       sauvegardes cohérentes
migrations/NNN_description.sql     scripts de schéma versionnés
tests/common/mod.rs                fixtures SQLite temporaires
tests/<capacité>.rs                tests d'intégration
```

Seuls `src/domain/`, `src/infrastructure/` et `tests/common/` sont
ouverts dans `.gitignore`, sur un niveau et pour les fichiers `.rs`.
Tout autre répertoire exige une entrée dans `.gitignore` et la mise
à jour de cette section.

## 5. Glossaire

Les noms Rust non encore introduits sont réservés à ces notions.

| Notion                    | Rust               | SQL                         |
|---------------------------|--------------------|-----------------------------|
| Source juridique          | `LegalSource`      | `legal_sources`             |
| Texte                     | `LegalText`        | `legal_texts`               |
| Disposition               | `Provision`        | `provisions`                |
| Version d'une disposition | `ProvisionVersion` | `provision_versions`        |
| Période de validité       | `ValidityPeriod`   | `valid_from`, `valid_until` |
| Événement d'acquisition   | `IngestionEvent`   | `ingestion_events`          |

Une période de validité est un intervalle semi-ouvert : la date de
début est incluse, la date de fin exclue. `temporal.rs` et les
déclencheurs de la migration 002 appliquent la même convention.

Dans le schéma actuel, `retrieved_at` tient lieu de date
d'acquisition, mais vaut par défaut l'horodatage d'enregistrement,
qui peut différer de l'acquisition réelle. Les dates de publication
et de rectification n'y figurent pas encore.

## 6. Décisions

- **D1 — Crate unique.** Voir § 1.
- **D2 — SQLite par le programme `sqlite3` du système.** Aucune
  dépendance Cargo. Conséquences : erreurs SQLite reçues sous forme
  de texte, sorties à analyser, version dépendante du poste
  (3.51.0 constatée le 2026-11-13). Tout réexamen est soumis à
  autorisation.
- **D3 — Pas de framework CLI.** Les arguments sont lus par filtrage
  sur une tranche.
- **D4 — Dates civiles `AAAA-MM-JJ`.** Validées par le domaine, sans
  bibliothèque de dates.
- **D5 — Compilation hors dépôt.** Le répertoire cible est fixé
  localement par `.cargo/config.toml`, non versionné.

## 7. Écarts constatés le 2026-11-13

À résorber au fil des chantiers ; chaque écart corrigé est retiré
de cette liste.

- `main.rs` contient la lecture des commandes, l'exécution de
  `sqlite3` et le SQL d'initialisation.
- Deux fonctions exécutent `sqlite3` avec des comportements
  différents (passage du SQL, activation des clés étrangères,
  traitement de la sortie).
- `schema_migrations` est créée hors des migrations versionnées,
  ce qui fait échouer les deux tests SQLite qui ne rejouent que
  `001_initial.sql`.
- Les tests d'intégration SQLite résident dans `src/migrations.rs`.
- `ValidityPeriod` expose ses champs, ce qui permet de contourner
  sa validation.
- La contrainte SQL `GLOB` accepte des dates impossibles
  (`2025-02-30`) que le domaine refuse.
- `db status` réussit malgré une violation de clé étrangère.
- La migration 002 ne contrôle pas les chevauchements antérieurs
  à l'installation de ses déclencheurs.
