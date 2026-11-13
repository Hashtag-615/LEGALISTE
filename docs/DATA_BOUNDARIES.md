# LEGALISTE — Frontières des données

## 1. Objet

Fixer où réside chaque catégorie de données, ce que le code peut
en faire et ce qui peut être publié. Les principes de
confidentialité et de préservation des sources figurent dans
`AGENTS.md` et ne sont pas répétés ici.

Ce dépôt est public : aucun chemin vers des données confidentielles
n'y est consigné, y compris dans ce document.

## 2. Zones

| Zone                    | Contenu | Emplacement | Git | Accès du code |
|-------------------------|---------|-------------|-----|---------------|
| Code                    | Sources Rust, définition canonique du schéma SQLite, documentation, gouvernance. | Dépôt. | Oui, par liste blanche. | Lecture. |
| Corpus public           | Textes officiels acquis, dans leur forme d'origine. | Hors dépôt ; `sources_publiques/` pour les seules pièces dont la publication est vérifiée. | Non, sauf publication vérifiée. | Lecture seule. |
| Base persistante        | Base SQLite opérationnelle et ses sauvegardes. | Répertoire de données hors dépôt (constante `DATA_DIR`). | Jamais. | Écriture par les seules commandes qui la modifient explicitement ; lecture seule pour toute consultation. |
| Données confidentielles | Coffre chiffré, pièces, dossiers, correspondances, données personnelles. | Hors dépôt. | Jamais. | Aucun. |
| Scénarios synthétiques  | Acteurs, faits et pièces fictifs servant à éprouver le raisonnement. | Tests ; mémoire du programme le temps d'une exécution. | Seulement comme fixtures de test fictives. | Lecture ; jamais enregistrés dans la base persistante. |
| Artefacts               | Compilation, bases de test temporaires. | Répertoire cible configuré localement ; répertoire temporaire du système. | Jamais. | Écriture. |

## 3. Base persistante

- Elle ne contient que des contenus normatifs et des métadonnées
  documentaires issus de sources publiques ; aucune donnée
  personnelle ni élément de dossier.
- Toute version enregistrée conserve sa référence de source et sa
  date d'acquisition.
- Elle est entièrement dérivée des sources et peut être reconstruite
  à partir d'elles ; une reconstruction conserve les références de
  source et les dates d'acquisition d'origine.
- Seules `db init`, qui crée la base selon le schéma canonique, et
  les futures commandes d'acquisition y écrivent. L'import de la
  bibliothèque ne crée qu'une base neuve ; il n'est appliqué à la base
  persistante que sur mandat explicite.
- Une consultation l'ouvre en lecture seule : elle ne crée ni ne
  modifie la base, son schéma ou ses données.
- Elle n'est jamais migrée : une base non conforme au schéma
  canonique est refusée, puis reconstruite sur commande explicite,
  sans écrasement automatique de l'existant.
- Ses sauvegardes restent dans le répertoire de données, sous la
  même protection que la base.
- Son chemin n'est résolu que par la CLI. Les fonctions
  d'infrastructure reçoivent le chemin de la base en paramètre et
  n'en connaissent aucun par défaut.

## 4. Données confidentielles

- Aucun module, test, script ou document du dépôt ne lit, n'écrit
  ni ne référence le coffre chiffré ou des données confidentielles.
- Aucune donnée confidentielle n'est importée dans la base SQLite.
- Les dossiers réels n'entrent dans aucun traitement du code tant
  que le stockage séparé prévu par la décision D7
  (`docs/ARCHITECTURE.md`) n'a pas été arrêté.

## 5. Tests

- Bases exclusivement temporaires, dans le répertoire temporaire du
  système, au nom unique, supprimées après usage.
- Aucun test ne référence `DATA_DIR` ni le chemin de la base
  persistante ; les fixtures refusent tout chemin situé hors du
  répertoire temporaire.
- Les tests de la CLI n'exécutent aucune commande qui atteindrait la
  base persistante.
- Données de test fictives ; aucun extrait de pièce réelle non
  publique.
- Extraits LEGI authentiques admis : documents publics reproduits
  octet pour octet dans `tests/common/legi_extracts.rs`, avec chemin
  dans l'archive, empreinte vérifiée et mentions de la Licence
  Ouverte.
- `tests/legi_archive.rs` lit l'archive de référence en lecture seule,
  sans rien écrire sur disque, et vérifie qu'elle reste inchangée ;
  son absence fait échouer ces tests.

## 6. Frontières d'exécution

| Frontière            | Module       | Règle |
|----------------------|--------------|-------|
| Corpus officiel      | `corpus`     | Instantané immuable, construit une fois avec la provenance de sa source ; aucune méthode de modification ; consulté par référence partagée. |
| Espace de simulation | `simulation` | Scénarios synthétiques et éphémères ; jamais enregistrés ; aucun accès direct au corpus. |
| Exécution isolée     | `execution`  | Outils fermés, en lecture seule, sans entrée-sortie, réseau, horloge ni code dynamique ; liste d'autorisation par skill ; provenance et trace de chaque appel. |

- Une simulation n'atteint le corpus qu'au travers de l'exécuteur.
- Une disposition absente du corpus est restituée comme absence
  documentaire, jamais comme absence de norme.
- `tests/boundaries.rs` et `tests/execution.rs` vérifient ces règles.

## 7. Publication Git

- `.gitignore` exclut tout par défaut et n'autorise que des chemins
  nommés. Chaque nouveau type de fichier ou répertoire publié exige
  une entrée explicite.
- Répertoires de code ouverts : `src/corpus/`, `src/domain/`,
  `src/execution/`, `src/infrastructure/`, `tests/common/`
  (fichiers `.rs`, un niveau).
- Jamais dans Git : corpus non vérifié, bases SQLite et
  sauvegardes, données confidentielles, secrets, artefacts de
  compilation, configuration locale (`.cargo/config.toml`), fichiers
  de travail à la racine.
- Avant toute publication : contrôle des fichiers indexés (taille,
  extension, motifs sensibles).
