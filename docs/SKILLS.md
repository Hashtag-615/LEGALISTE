# LEGALISTE — Skills juridiques

## 1. Définition

Un skill est une procédure d'examen déclarée, exécutée par
l'orchestrateur au moyen des seuls outils qu'elle autorise
(`docs/ARCHITECTURE.md`, D9). Il ne contient aucun code arbitraire :
sa déclaration est une donnée Rust (`Skill`, `src/execution/skill.rs`)
et sa procédure une fonction Rust relue et testée.

## 2. Format minimal

| Champ                | Rust                 | Contenu |
|----------------------|----------------------|---------|
| Identité             | `id`                 | Nom stable, en minuscules. |
| Objectif             | `objective`          | Ce que le skill établit, et dans quelles limites. |
| Préconditions        | `preconditions`      | Corpus, provenance et scénario requis. |
| Outils autorisés     | `allowed_tools`      | Liste fermée, appliquée par l'exécuteur ; tout autre outil est refusé et tracé. |
| Procédure            | `procedure`          | Étapes, dans l'ordre de la fonction qui les exécute. |
| Preuves requises     | `required_evidence`  | Éléments que le résultat doit citer : identifiants, périodes, source, trace. |
| Sorties              | `outputs`            | Résultat documentaire, réserves, limites. |
| Conditions d'échec   | `failure_conditions` | Cas où le skill s'arrête sans résultat. |

## 3. Règles

- Un skill ne repose que sur des outils réellement implémentés.
- Il distingue le résultat documentaire, ses réserves et ses limites
  permanentes ; il ne présente jamais comme certaine une conclusion
  qui dépend d'une interprétation ou d'un fait non établi.
- Une disposition absente du corpus donne une absence documentaire,
  jamais une absence de norme.
- À corpus, scénario et appels identiques, rapport et trace
  identiques : aucune heure ni aucun effet externe n'y entre.

## 4. Skills disponibles

### `resolution-normative-temporelle`

Déclaration : `TEMPORAL_RESOLUTION` ; exécution :
`resolve_temporally` (`src/execution/skill.rs`) ; tests :
`tests/execution.rs`, `tests/legi_extracts.rs` (documents
authentiques) et `tests/legi_archive.rs` (Code civil lu dans
l'archive de référence, puis relu dans une base SQLite canonique : mêmes
rapports).

Il identifie, selon la source consultée, la version d'une disposition
en vigueur à la date d'une situation. Outils : état du corpus,
résolution d'une disposition, détection des anomalies. Le rapport
contient la provenance de la source, le résultat documentaire, les
anomalies, les réserves (date non établie, absence documentaire,
versions concurrentes, anomalies) et la trace des appels.

Il ne détermine pas la norme applicable : droit transitoire,
hiérarchie des normes et sources hors corpus restent à examiner.
