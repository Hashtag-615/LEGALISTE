# LEGALISTE — Architecture

## 1. Principes

- Crate unique `legaliste` : une bibliothèque (`src/lib.rs`) et un
  binaire (`src/main.rs`).
- Séparation en plusieurs crates seulement sur besoin technique
  démontré : consommateur distinct, dépendances incompatibles ou
  temps de compilation mesuré.
- Une couche ou un module n'est créé qu'avec une fonctionnalité
  démontrable et ses tests ; aucun module vide.
- Les règles d'écriture figurent dans `docs/CODE_POLICY.md` ; les
  zones de données dans `docs/DATA_BOUNDARIES.md`.

## 2. Couches

| Couche           | Responsabilité | Exclu |
|------------------|----------------|-------|
| `domain`         | Types et invariants juridiques : sources, textes, dispositions, versions, périodes de validité, provenance, consultation à une date. | Entrées-sorties, SQL, processus, variables d'environnement, horloge système. |
| `corpus`         | Corpus officiel en lecture seule : instantané immuable des dispositions et de la provenance de leur source ; lecture des documents LEGI déjà extraits (XML), sans entrée-sortie. | Toute écriture, entrées-sorties, règles juridiques, simulation. |
| `simulation`     | Scénarios synthétiques et éphémères : faits datés et leur statut (D7). | Accès au corpus, persistance, données réelles. |
| `execution`      | Environnement de l'orchestrateur : outils fermés en lecture seule, exécuteur borné par une liste d'autorisation, provenance et trace ; skills déclarés. | Entrées-sorties, horloge, réseau, code dynamique, écriture du corpus. |
| `infrastructure` | Programmes du système et fichiers : lecture de l'archive LEGI par `tar` et `shasum` ; exécution de `sqlite3`, définition canonique du schéma, contrôle de conformité, sauvegardes ; conversion entre lignes SQL et types du domaine. | Règles juridiques, présentation, chemins par défaut, écriture dans le corpus. |
| `cli`            | Lecture des arguments, choix de la commande, résolution des chemins par défaut, formatage des sorties, conversion des erreurs. | SQL, règles juridiques. |
| `main.rs`        | Appel de la CLI, écriture de l'erreur, code de retour. | Toute autre logique. |

La couche `execution` tient le rôle de couche d'application : ses
skills combinent corpus, simulation et domaine. `infrastructure` et
`cli` n'ont pas encore de module. Les commandes d'administration de
la base (`db init`, `db status`) ne mobilisent que le stockage : la
CLI les adressera directement à `infrastructure`.

## 3. Sens des dépendances

Dépendances autorisées :

- `main.rs` → `cli`
- `cli` → `execution`, `infrastructure`, `corpus`, `simulation`,
  `domain`
- `execution` → `corpus`, `simulation`, `domain`
- `infrastructure` → `corpus`, `domain`
- `corpus` → `domain`
- `simulation` → `domain`
- `domain` → bibliothèque standard uniquement

Interdits :

- toute dépendance remontante : `domain` vers une autre couche ;
  `corpus` ou `simulation` vers `execution`, `infrastructure` ou
  `cli` ; `infrastructure` vers `execution` ou `cli` ;
- toute dépendance entre `corpus` et `simulation` : une simulation
  n'atteint le corpus qu'au travers de l'exécuteur ;
- toute dépendance d'`execution` vers `infrastructure` : le corpus
  lui est remis déjà construit, l'exécution reste sans entrée-sortie ;
- tout cycle entre modules ;
- tout usage de `main.rs` par un autre module.

Ces règles sont vérifiables par recherche des chemins `crate::`
importés dans chaque couche ; `tests/boundaries.rs` en contrôle une
partie, avec l'absence d'effet externe de la bibliothèque.

## 4. Carte des modules

Cible indicative. Un fichier n'est créé qu'avec son premier contenu
réel. Les répertoires utilisent `mod.rs`, comme `tests/common/`.

```text
src/main.rs                        point d'entrée
src/lib.rs                         déclaration des modules
src/cli.rs                         interface opérateur
src/temporal.rs                    dates civiles et périodes de validité
src/domain/mod.rs                  types juridiques
src/domain/legi.rs                 conventions LEGI : identifiants,
                                   états officiels, bornes de vigueur
src/domain/provision.rs            versions, résolution à une date
src/domain/anomaly.rs              anomalies temporelles
src/corpus/mod.rs                  corpus en lecture seule, provenance
src/corpus/legi_xml.rs             lecteur XML dédié aux articles LEGI
src/corpus/legi_text.rs            corpus d'un texte à partir de ses
                                   documents d'article
src/simulation.rs                  scénarios synthétiques, faits datés
src/execution/mod.rs
src/execution/tool.rs              contrat des outils, exécuteur, trace
src/execution/skill.rs             format des skills, premier skill
src/infrastructure/mod.rs
src/infrastructure/legi_archive.rs lecture de l'archive LEGI (tar),
                                   empreintes (shasum), provenance
src/infrastructure/sqlite.rs       point unique d'exécution de sqlite3
src/infrastructure/schema.rs       définition canonique du schéma,
                                   contrôle de conformité
src/infrastructure/legi_store.rs   import atomique et relecture d'un
                                   texte LEGI
src/infrastructure/backup.rs       sauvegardes cohérentes
tests/common/mod.rs                fixtures partagées des tests
tests/common/legi_extracts.rs      extraits LEGI authentiques
tests/<capacité>.rs                tests d'intégration
```

`src/temporal.rs` relève de la couche `domain` : mêmes
responsabilités, mêmes exclusions, mêmes dépendances.

Seuls `src/corpus/`, `src/domain/`, `src/execution/`,
`src/infrastructure/` et `tests/common/` sont ouverts dans
`.gitignore`, sur un niveau et pour les fichiers `.rs`.
Tout autre répertoire exige une entrée dans `.gitignore` et la mise
à jour de cette section.

## 5. Glossaire et temporalité

Les noms Rust non encore introduits sont réservés à ces notions.

| Notion                    | Rust               | SQL                         |
|---------------------------|--------------------|-----------------------------|
| Source officielle capturée| `SourceRecord`     | `source_records` ; date d'acquisition dans `ingestion_events` |
| Texte                     | `LegalText`        | `legal_texts`               |
| Disposition               | `Provision`        | `provisions` (clé : plus petit identifiant de version) |
| Version d'une disposition | `ProvisionVersion` | `provision_versions`        |
| Période de validité       | `ValidityPeriod`   | `valid_from`, `valid_until`, dates telles que publiées |
| Événement d'acquisition   | `IngestionEvent`   | `ingestion_events`          |
| Date civile               | `CivilDate`        | `TEXT` `AAAA-MM-JJ`, contrôlée |
| Identifiant officiel LEGI | `LegiId`           | `TEXT`, préfixe et douze chiffres contrôlés |
| État officiel LEGI        | `LegiStatus`       | `status`, onze valeurs ou vide |
| Résolution à une date     | `Resolution`       | aucun : calculée            |
| Anomalie temporelle       | `TemporalAnomaly`  | aucun : calculée            |
| Corpus officiel           | `Corpus`           | aucun : en mémoire          |
| Scénario                  | `Scenario`         | aucun : éphémère            |
| Fait daté et son statut   | `DatedFact`, `FactStatus` | aucun : éphémère     |
| Outil, exécuteur          | `ToolKind`, `Executor` | aucun                   |
| Skill juridique           | `Skill`            | aucun                       |
| Extrait d'un texte LEGI   | `TextExtract`      | aucun : en mémoire          |

Le moteur distingue :

- la date de référence : date de la situation examinée ;
- l'entrée en vigueur et la fin de vigueur d'une version
  (`DATE_DEBUT`, `DATE_FIN` dans LEGI ; `2999-01-01` en fin signifie
  « sans fin connue ») ;
- la cause de la fin de vigueur, qui ne se lit pas dans la date mais
  dans l'état officiel : modification (`MODIFIE`), abrogation
  (`ABROGE`), abrogation différée (`ABROGE_DIFF`), transfert
  (`TRANSFERE`)… ;
- l'application différée : version dont l'entrée en vigueur est
  postérieure à la date de l'état de la source (`VIGUEUR_DIFF`) ;
- la date de l'état de la source : date de l'archive LEGI, seule
  date à laquelle l'état officiel est valable ;
- la date d'acquisition : capture de l'archive par l'opérateur ;
- la date de consultation : moment où la question est posée.

L'applicabilité à la date de référence se détermine par les seules
périodes de vigueur. L'état officiel sert de contrôle de cohérence,
apprécié à la date de l'état de la source ; il ne modifie jamais
l'applicabilité. Une résolution ne dépend que des versions connues
et de la date de référence : à source identique et date identique,
résultat identique.

Constats sur les 526 864 versions d'articles des 108 codes de
l'archive `Freemium_legi_global_20261108-140000`, mesurés en flux le
2026-11-13 :

- identifiants tous de la forme `LEGIARTI` suivie de douze chiffres ;
  états limités aux onze valeurs de `LegiStatus`, plus un état vide ;
  dates et états de `META_ARTICLE` identiques à ceux de `LIEN_ART` ;
- 167 909 fins `2999-01-01`, aucune autre fin au-delà de 2900 ;
  9 débuts `2999-01-01`, dont la période est conservée littéralement ;
- 1 709 périodes vides (début égal à la fin) et 5 707 périodes
  inversées (fin antérieure au début), dont 5 594 `MODIFIE_MORT_NE` :
  toutes conservées telles que publiées, jamais applicables ; les
  périodes inversées sont signalées comme anomalies ;
- sur 263 996 listes `VERSIONS` distinctes, 7 638 présentent une
  lacune et 1 562 un chevauchement.

Code civil (`LEGITEXT000006070721`), même archive, lu par le moteur le
2026-11-13 : 6 395 documents d'article, 3 540 dispositions, chaque
version citée dotée de son document ; XML homogène (UTF-8, une
déclaration par document, ni DOCTYPE, ni CDATA, ni commentaire ; seule
entité rencontrée : `&#13;`) ; 37 périodes inversées, 1 période vide
marquée `VIGUEUR` (conflit d'état), 22 dispositions à chevauchement,
116 lacunes, aucun état absent.

Une période de validité est un intervalle semi-ouvert : la date de
début est incluse, la date de fin exclue. `temporal.rs` et les
contraintes du schéma canonique appliquent la même convention. Ces
contraintes ne sont jamais plus permissives que la validation du
domaine : une date impossible (`2025-02-30`) est refusée des deux
côtés.

Le schéma canonique distingue la date d'acquisition réelle de
l'horodatage d'enregistrement et ne substitue jamais l'un à l'autre
par défaut. Les dates de publication et de rectification, lorsqu'elles
sont enregistrées, ont chacune leur champ, nommé d'après leur nature
(`docs/CODE_POLICY.md`, § 3). Une reconstruction de la base (D6)
conserve la date d'acquisition de chaque source ; elle ne lui
substitue jamais la date de reconstruction.

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
- **D6 — Schéma canonique, sans migration.** Arrêtée le 2026-11-13.
  La base persistante est une production dérivée des sources
  officielles. Son schéma a une seule définition canonique (§ 4) ;
  il n'existe ni script de migration, ni table de versions du
  schéma, ni mise à niveau sur place. Modifier le schéma, c'est
  modifier cette définition ; une base qui ne s'y conforme pas est
  refusée, puis reconstruite depuis les sources sur commande
  explicite ; une reconstruction ne détruit jamais silencieusement
  la base existante. Aucune consultation ne modifie la base.
  Conséquence : la base ne contient rien qui ne puisse être
  reconstruit depuis les sources (`docs/DATA_BOUNDARIES.md`, § 3).
- **D7 — Dossiers distincts du corpus.** Arrêtée le 2026-11-13.
  Les dossiers (acteurs, faits, pièces, positions) forment un
  domaine distinct du corpus juridique et n'entrent jamais dans la
  base juridique publique. Un stockage local séparé, doté de ses
  propres garanties de confidentialité, relèvera d'une décision
  ultérieure ; d'ici là, seuls des scénarios synthétiques et
  éphémères sont admis.
- **D8 — Règles formalisées, interprétations dérivées.** Arrêtée le
  2026-11-13. Une règle formalisée est une production dérivée : elle
  cite les versions de dispositions qu'elle interprète, n'est jamais
  présentée comme leur texte et ne se substitue jamais à la source
  officielle. Aucune disposition n'est réduite artificiellement à un
  algorithme binaire : ce qui ne peut être établi reste indéterminé.
- **D9 — Orchestrateur sans invention.** Arrêtée le 2026-11-13.
  L'orchestrateur coordonne une procédure d'examen reproductible ;
  il n'invente ni fait, ni norme, ni jurisprudence. Chaque étape
  produit un résultat contrôlable ; les contradictions non résolues
  restent visibles dans le résultat final ; aucune hypothèse ne
  devient certitude par accumulation de raisonnements. Il ne se
  substitue pas à une juridiction.
- **D10 — Archive LEGI lue par `tar` et `shasum` du système.**
  Arrêtée le 2026-11-13. Dans la logique de D2, aucune dépendance
  Cargo : `tar` (bsdtar 3.5.3 constaté, option `--include`) extrait
  vers sa sortie standard, sans écriture sur disque ; `shasum -a 256`
  vérifie l'empreinte enregistrée avant toute lecture ; un lecteur XML
  dédié, sans entrée-sortie, ne couvre que les constructions constatées
  dans l'archive et refuse les autres. Conséquences : chaque lecture
  parcourt l'archive entière (environ une minute) ; GNU tar, qui
  ignore `--include`, n'est pas pris en charge ; tout document
  divergent arrête la lecture.
- **D11 — Base SQLite construite en entier, publiée par lien.**
  Arrêtée le 2026-11-13, en application de D6. Un import crée une
  base neuve : schéma canonique et données en une transaction, dans
  `<base>.partial` ; la base est contrôlée (intégrité, clés étrangères,
  conformité), relue et comparée à l'extrait, puis publiée par lien
  physique, qui échoue si le nom existe. Aucune base existante n'est
  remplacée ; aucune base incomplète n'apparaît sous son nom. Les
  dates sont conservées telles que publiées ; aucune contrainte
  n'ordonne début et fin. La base ne porte aucun horodatage
  d'exécution : deux imports d'une même source ont le même contenu
  logique. Toute consultation passe par `sqlite3 -readonly -safe`,
  après contrôle ; résolution et anomalies restent au domaine.

## 7. Écarts constatés le 2026-11-13

À résorber au fil des chantiers ; chaque écart corrigé est retiré
de cette liste.

- `main.rs` réunit encore la lecture des commandes et le chemin par
  défaut de la base ; aucun module `cli` n'existe.
- L'import et `db init` ne sont exposés par aucune commande : la
  création d'une base n'est possible que depuis la bibliothèque.
- La CLI elle-même n'est couverte par aucun test ; le contrôle
  qu'elle appelle (`schema::check`) l'est.
- La base persistante désignée par `DATA_DIR` est absente ;
  `DATA/legaliste.sqlite3`, hors de ce chemin, porte un schéma
  antérieur à D6 (`schema_migrations`) et serait refusée comme non
  conforme.
