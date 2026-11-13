# 60_LEGALISTE — Instructions Claude Code

Lire `AGENTS.md` avant toute intervention.
`AGENTS.md` porte les invariants communs ; ne pas les dupliquer ici.

## Périmètre
- Projet autonome ; ne pas explorer les dépôts voisins.
- Lecture seule et aucun effet externe par défaut.
- Ne jamais modifier les pièces juridiques originales.
- Aucune installation, synchronisation, publication ou transmission sans accord.

## Investigation
- Lire seulement les fichiers et passages nécessaires.
- Préférer `rg`, `sed`, `head` et les inspections bornées.
- Préserver le contexte indispensable à l'interprétation des extraits.
- Distinguer date des faits, version du texte et date de consultation.
- Ne jamais présenter une recherche non effectuée comme accomplie.
- Arrêter l'investigation lorsque la question est suffisamment instruite.

## Modifications autorisées
- Délimiter le chantier avant intervention.
- Vérifier les fichiers touchés et les différences produites.
- Ne jamais altérer une source pour la conformer à une conclusion.
- Si Git est présent : contrôler `git status --short` et `git diff --stat`.
- Aucun `git add .`, commit ou push sans autorisation explicite.
- Aucun `Co-Authored-By` dans les commits.

## Restitution
- Exposer les faits retenus, le droit applicable et le raisonnement.
- Citer les références effectivement contrôlées.
- Présenter les objections pertinentes et les incertitudes.
- Ne jamais déclarer une validation réussie sans vérification réelle.
- Privilégier les conclusions motivées aux affirmations péremptoires.
