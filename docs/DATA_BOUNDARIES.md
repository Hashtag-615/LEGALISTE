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
| Code                    | Sources Rust, migrations SQL, documentation, gouvernance. | Dépôt. | Oui, par liste blanche. | Lecture. |
| Corpus public           | Textes officiels acquis, dans leur forme d'origine. | Hors dépôt ; `sources_publiques/` pour les seules pièces dont la publication est vérifiée. | Non, sauf publication vérifiée. | Lecture seule. |
| Base persistante        | Base SQLite opérationnelle et ses sauvegardes. | Répertoire de données hors dépôt (constante `DATA_DIR`). | Jamais. | Écriture par les seules commandes qui la modifient explicitement. |
| Données confidentielles | Coffre chiffré, pièces, dossiers, correspondances, données personnelles. | Hors dépôt. | Jamais. | Aucun. |
| Artefacts               | Compilation, bases de test temporaires. | Répertoire cible configuré localement ; répertoire temporaire du système. | Jamais. | Écriture. |

## 3. Base persistante

- Elle ne contient que des contenus normatifs et des métadonnées
  documentaires issus de sources publiques ; aucune donnée
  personnelle ni élément de dossier.
- Toute version enregistrée conserve sa référence de source et sa
  date d'acquisition.
- Seules `db init`, `db migrate` et les futures commandes
  d'acquisition y écrivent ; une consultation n'y écrit jamais.
- Ses sauvegardes restent dans le répertoire de données, sous la
  même protection que la base.
- Son chemin n'est résolu que par la CLI. Les fonctions
  d'infrastructure reçoivent le chemin de la base en paramètre et
  n'en connaissent aucun par défaut.

## 4. Données confidentielles

- Aucun module, test, script ou document du dépôt ne lit, n'écrit
  ni ne référence le coffre chiffré ou des données confidentielles.
- Aucune donnée confidentielle n'est importée dans la base SQLite.

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

## 6. Publication Git

- `.gitignore` exclut tout par défaut et n'autorise que des chemins
  nommés. Chaque nouveau type de fichier ou répertoire publié exige
  une entrée explicite.
- Répertoires de code ouverts : `src/domain/`,
  `src/infrastructure/`, `tests/common/` (fichiers `.rs`, un niveau).
- Jamais dans Git : corpus non vérifié, bases SQLite et
  sauvegardes, données confidentielles, secrets, artefacts de
  compilation, configuration locale (`.cargo/config.toml`), fichiers
  de travail à la racine.
- Avant toute publication : contrôle des fichiers indexés (taille,
  extension, motifs sensibles).
