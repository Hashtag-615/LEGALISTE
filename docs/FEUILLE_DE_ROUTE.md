# LEGALISTE — Feuille de route

Arrêtée le 2026-11-13. Les décisions d'architecture correspondantes
(D6 à D9) figurent dans `docs/ARCHITECTURE.md`, § 6.

## 1. Cap

LEGALISTE est un moteur d'examen juridique fondé sur des règles
vérifiables. Il établit ce qui peut l'être, montre ce qui reste
discutable et signale ce qui ne peut être résolu. Il ne se substitue
pas à une juridiction.

Les étapes sont successives : chacune repose sur la fiabilité de la
précédente.

## 2. Étapes

### Étape 1 — Socle documentaire et temporel (en cours)

Question à laquelle répondre : quel texte était juridiquement
applicable à une situation survenue à une date déterminée, et sur
quelles sources repose cette identification ?

Critère de réussite : à corpus identique et date identique, le moteur
restitue exactement les mêmes dispositions, références et anomalies.

| Incrément | Contenu | État |
|-----------|---------|------|
| 1.1 | Noyau temporel du domaine : dates civiles, identifiants et statuts LEGI, versions de dispositions, résolution à une date, détection des lacunes, chevauchements, périodes inversées et incohérences de statut. | Réalisé |
| 1.2 | Définition canonique du schéma SQLite, contrôle de conformité, `db init`, consultation en lecture seule. | Réalisé (D11) ; `db status` contrôle intégrité, clés étrangères et conformité en lecture seule ; `db init` non exposé |
| 1.3 | Lecture de l'archive LEGI en flux, sans extraction sur disque ; provenance vérifiée (nom, date d'état, SHA-256). | Réalisé pour un texte à la fois (Code civil éprouvé, D10) ; registre des sources à lire depuis le manifeste |
| 1.4 | Import reproductible : base temporaire d'abord ; base persistante sur mandat explicite ; traçabilité des acquisitions. | Réalisé pour le Code civil, en bases temporaires ; base persistante non créée, en attente de mandat |
| 1.5 | Consultation : dispositions applicables à une date, selon l'état de la source, avec références et anomalies ; test de reproductibilité. | Réalisé dans la bibliothèque (skill sur la base SQLite, résultats identiques à l'archive) ; aucune commande CLI |

### Étape 2 — Qualification juridique

Introduction des faits, chacun établi, allégué, contesté ou
hypothétique. Confrontation aux conditions d'application des normes
pertinentes, en distinguant conditions nécessaires, exceptions et
dérogations, éléments de preuve, qualifications concurrentes et
incertitudes empêchant de conclure (D8).

### Étape 3 — Raisonnement contradictoire

Pour chaque dossier, plusieurs raisonnements juridiquement
défendables, une argumentation favorable à chaque partie, sans
préférence artificielle. Chaque argument est relié aux faits, aux
dispositions applicables et, lorsqu'elle est disponible, à la
jurisprudence.

### Étape 4 — Orchestrateur

Procédure d'examen reproductible (D9) :
Dossier → Faits → Qualification → Normes applicables → Exceptions →
Jurisprudence → Contradictoire → Conclusion motivée.

Socle posé le 2026-11-13 : corpus en lecture seule, espace de
simulation, exécuteur à outils fermés avec provenance et trace,
format des skills et premier skill (`docs/SKILLS.md`). Ce socle ne
traite encore qu'un corpus construit en mémoire.

### Étape 5 — Extension de l'univers juridique

LEGI est un point de départ, non l'intégralité du droit applicable.
Jurisprudence, droit de l'Union européenne, conventions
internationales et normes réglementaires complémentaires s'ajoutent
progressivement, chacun avec ses propres mécanismes d'identification,
de provenance, de temporalité et de contrôle. Jusque-là, l'absence
d'une norme dans le corpus ne prouve pas son absence en droit.

## 3. Décisions ouvertes

- Commandes d'import et de consultation : leur forme CLI reste à
  arrêter ; d'ici là, la base n'est construite que par la
  bibliothèque.
- Plusieurs textes par base : un import accepte plusieurs extraits,
  mais chaque lecture de l'archive ne livre qu'un texte.
- Identité durable d'une disposition LEGI : aucun identifiant commun
  aux versions d'un article n'a été constaté ; la liste `VERSIONS`
  de chaque article en tient lieu.
- Dates de fin `2222-02-22` (versions `ABROGE_DIFF`) et débuts
  `2999-01-01` : valeurs conventionnelles supposées, sens non
  déterminé ; dates conservées littéralement.
- Versions d'articles de textes non codifiés sans `ETAT` : à étudier
  avant leur import.
- Stockage séparé des dossiers réels (D7).
- Exposition des outils et skills à l'orchestrateur : aucune commande
  CLI n'est créée tant que ce mode n'est pas arrêté.
