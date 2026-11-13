# LEGALISTE — Politique d'écriture du code Rust

## 1. Objectif

Préserver un moteur juridique Rust lisible, vérifiable, modulaire
et maintenable, sans refactorisations structurelles répétées.

Documents associés, dont les règles ne sont pas répétées ici :

- `AGENTS.md` : doctrine juridique, confidentialité, discipline des agents ;
- `CLAUDE.md` : discipline opérationnelle de Claude Code, y compris Git ;
- `docs/ARCHITECTURE.md` : couches, sens des dépendances, carte des modules ;
- `docs/DATA_BOUNDARIES.md` : zones de données, frontières
  d'exécution et publication ;
- `docs/SKILLS.md` : format et inventaire des skills juridiques ;
- `docs/FEUILLE_DE_ROUTE.md` : étapes et décisions ouvertes.

## 2. Points d'entrée

`main.rs` se limite à transmettre les arguments à la CLI et à
convertir son résultat en sortie d'erreur et code de retour.

Aucune requête SQL, définition de schéma ni règle juridique
ne doit être placée dans `main.rs`.

`lib.rs` ne contient que des déclarations de modules.

## 3. Nommage

- Conventions Rust : `snake_case` pour modules, fonctions et
  variables ; `CamelCase` pour types et traits ;
  `SCREAMING_SNAKE_CASE` pour constantes.
- Identifiants en anglais ; messages opérateur, commentaires et
  documentation en français. Les valeurs d'un vocabulaire officiel
  (par exemple les états LEGI) gardent leur forme d'origine, sans
  traduction qui en requalifierait le sens.
- Le vocabulaire juridique suit le glossaire de
  `docs/ARCHITECTURE.md` ; une notion porte le même nom en Rust
  et en SQL.
- Chaque date est nommée d'après sa nature : publication, entrée en
  vigueur, fin de vigueur, état de la source, acquisition,
  rectification, référence (date de la situation examinée). Aucun
  champ générique `date`.
- Noms de module proscrits : `utils`, `helpers`, `misc`, `manager`,
  `common` (sauf `tests/common/`).
- Noms de fichier proscrits : ceux contenant `token`, `secret` ou
  `credentials`, silencieusement exclus de Git par `.gitignore`.
- Tests : le nom énonce le comportement vérifié
  (`rejects_reversed_period`, `rolls_back_on_sql_error`).

## 4. Taille et cohésion

Mesure : lignes physiques, tests unitaires inclus ; pour une
fonction, de la signature à l'accolade fermante.

| Élément  | Objectif | Revue obligatoire              | Justification écrite |
|----------|----------|--------------------------------|----------------------|
| Fichier  | ≤ 300    | avant extension au-delà de 250 | au-delà de 400       |
| Fonction | —        | au-delà de 50                  | au-delà de 80        |

Une justification prend la forme d'un commentaire
`// Taille justifiée : <motif>` placé en tête du fichier ou
au-dessus de la fonction concernée.

La cohésion prime sur le respect mécanique des seuils : un fichier
n'est jamais fractionné dans le seul but de passer sous un seuil.

## 5. Visibilité

- Tout élément est privé par défaut.
- `pub(crate)` pour un partage entre modules du crate.
- `pub` uniquement pour la surface utilisée par `main.rs` ou par
  `tests/` ; la bibliothèque n'est pas publiée.
- Un type porteur d'invariant garde ses champs privés et ne se
  construit que par un constructeur qui vérifie l'invariant.
- Aucune réexportation globale (`pub use module::*`).
- Un test n'élargit pas la visibilité d'un élément interne : il
  passe par la surface publique ou devient un test unitaire du module.

## 6. Erreurs

- Toute opération faillible retourne `Result`.
- Une erreur que l'appelant doit distinguer devient un variant d'une
  `enum` propre au module, implémentant `Display` et
  `std::error::Error`. Le texte libre (`String`) n'est admis que
  lorsque aucun appelant ne distingue les cas.
- Chaque couche ajoute son contexte sans perdre la cause d'origine ;
  le message d'erreur de `sqlite3` est conservé tel quel.
- Seule la CLI convertit une erreur en message opérateur :
  `ERREUR : <message>` sur la sortie d'erreur, code de retour 1.
  Les résultats vont sur la sortie standard.
- Ni `unwrap` ni `expect` dans le code de production, sauf invariant
  établi localement et expliqué en commentaire. Les tests peuvent
  les utiliser.
- Aucun `Result` ignoré (`let _ =`, `.ok()`) en production sans
  commentaire établissant que l'échec est sans conséquence.
- Aucun `panic!` pour un échec prévisible : saisie, fichier, SQL.

## 7. SQLite et transactions

- Un seul point d'exécution de `sqlite3` dans le crate. Il active
  les clés étrangères avant toute transaction, car
  `PRAGMA foreign_keys` est sans effet à l'intérieur d'une
  transaction, et interrompt le script à la première erreur (`-bail`).
- Le schéma a une seule définition canonique, partagée par
  `db init` et par les fixtures de test. Aucune autre instruction du
  crate ne crée ni ne modifie d'objet de schéma.
- Aucune migration historique : ni script versionné, ni table de
  versions du schéma, ni mise à niveau sur place. Un changement de
  schéma modifie la définition canonique ; les bases existantes sont
  reconstruites depuis les sources (`docs/ARCHITECTURE.md`, D6).
- Toute ouverture d'une base contrôle la conformité de ses objets à
  la définition canonique. Une base non conforme est refusée, jamais
  corrigée ni complétée.
- Une consultation ouvre la base en lecture seule
  (`sqlite3 -readonly`) : elle ne crée ni base, ni objet, ni donnée,
  et n'ajuste jamais le schéma.
- L'initialisation, comme toute écriture, forme une seule
  transaction (`BEGIN IMMEDIATE` … `COMMIT`), exécutée en un seul
  appel à `sqlite3` : toute erreur laisse la base dans son état
  antérieur.
- Avant modification d'une base persistante :
  1. contrôle d'intégrité (`PRAGMA integrity_check`) ;
  2. contrôle des clés étrangères (`PRAGMA foreign_key_check`) ;
  3. contrôle de conformité au schéma canonique ;
  4. contrôle des données que l'écriture va contraindre ;
  5. sauvegarde cohérente produite par SQLite, et non par copie de
     fichier, puis vérifiée ; tout échec interrompt l'écriture.
- Après modification : mêmes contrôles et présence des objets
  attendus.
- Une commande d'écriture relancée sans donnée nouvelle n'écrit rien.
- Aucun écrasement automatique de données existantes, d'une base
  ou d'une sauvegarde.

## 8. Tests

- Tests unitaires dans le module, au plus près des règles pures.
- Tests SQLite et CLI dans `tests/`, fixtures partagées dans
  `tests/common/mod.rs` ; aucune duplication de la préparation
  des bases.
- Chaque test utilise sa propre base temporaire, au nom unique,
  supprimée même en cas d'échec. Les règles d'accès aux données
  figurent dans `docs/DATA_BOUNDARIES.md`.
- Tout chemin d'échec et tout rollback ont leur test. Un test
  d'échec vérifie la cause de l'échec, pas seulement sa survenance.
- Aucun test n'est désactivé (`#[ignore]`) ni affaibli pour masquer
  une défaillance.
- L'absence de `sqlite3` fait échouer les tests concernés ; elle ne
  les fait jamais passer silencieusement.

## 9. Qualité Rust

- `cargo fmt` obligatoire.
- `cargo clippy --all-targets` sans avertissement avant publication.
- Aucun `#![allow(...)]` au niveau du crate ; un `#[allow(...)]`
  local est accompagné de son motif.
- Aucune dépendance externe sans justification et autorisation.
- Aucun module fourre-tout ; aucune duplication de logique métier.
- Aucune abstraction (trait, couche, généricité) sans usage réel.

## 10. Commentaires

Les commentaires expliquent les invariants, les risques et les
décisions, sans paraphraser le code.

Toute modification des frontières architecturales est documentée
dans `docs/ARCHITECTURE.md` avant sa généralisation.

## 11. Principe directeur

Ajouter une fonctionnalité ne doit pas obliger à modifier
plusieurs couches pour une raison qui relève d'une seule.

La simplicité structurelle prime sur l'abstraction prématurée.
