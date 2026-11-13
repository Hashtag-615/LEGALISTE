//! Extraits authentiques de l'archive LEGI de référence.
//!
//! Source : DILA, base LEGI, archive
//! `Freemium_legi_global_20261108-140000.tar.gz` (fichier du
//! 2026-11-08, capturé le 2026-11-13, SHA-256 dans `ARCHIVE_SHA256`),
//! <https://echanges.dila.gouv.fr/OPENDATA/LEGI/Freemium_legi_global_20261108-140000.tar.gz>.
//! Licence Ouverte v2.0 (Etalab) ; paternité : DILA.
//!
//! Chaque document est reproduit octet pour octet, sans aucune
//! modification ; son empreinte SHA-256, relevée à l'extraction, est
//! revérifiée par `tests/legi_extracts.rs`. Ces documents sont intégrés
//! en Rust parce que le contrôle de l'index Git n'admet que des `.rs`
//! sous `tests/`. Ils éprouvent le moteur et n'attestent pas du droit
//! en vigueur.

// Taille justifiée : documents officiels reproduits intégralement, pour
// que leur empreinte reste vérifiable.

pub struct Extract {
    /// Chemin du membre dans l'archive.
    pub member: &'static str,
    pub sha256: &'static str,
    pub xml: &'static str,
}

pub const EXTRACTS: [Extract; 10] = [
    // art. 16-11, version 1994-07-30 → 2004-08-07
    Extract {
        member: "legi/global/code_et_TNC_en_vigueur/code_en_vigueur/LEGI/TEXT/00/00/06/07/07/LEGITEXT000006070721/article/LEGI/ARTI/00/00/06/41/93/LEGIARTI000006419307.xml",
        sha256: "0f111754bf1be7097f5aeb94f460d925bcf33991c85f2313d48a7b0875bedf36",
        xml: r#"<?xml version="1.0" encoding="UTF-8"?>
<ARTICLE>
<META>
<META_COMMUN>
<ID>LEGIARTI000006419307</ID>
<ANCIEN_ID>ACAXXXXXXXX5X00016LAXXAA</ANCIEN_ID>
<ORIGINE>LEGI</ORIGINE>
<URL>article/LEGI/ARTI/00/00/06/41/93/LEGIARTI000006419307.xml</URL>
<NATURE>Article</NATURE>
</META_COMMUN>
<META_SPEC>
<META_ARTICLE>

<NUM>16-11</NUM>

<ETAT>MODIFIE</ETAT>

<DATE_DEBUT>1994-07-30</DATE_DEBUT>
<DATE_FIN>2004-08-07</DATE_FIN>



<TYPE>AUTONOME</TYPE>
</META_ARTICLE>
</META_SPEC>
</META>
<CONTEXTE>
<TEXTE autorite="" cid="LEGITEXT000006070721" date_publi="2999-01-01" date_signature="2999-01-01" ministere="" nature="CODE" nor="" num="" num_parution_jo="">
<TITRE_TXT c_titre_court="Code civil" debut="1804-03-21" fin="2999-01-01" id_txt="LEGITEXT000006070721">Code civil</TITRE_TXT>
<TM>
<TITRE_TM debut="1804-03-21" fin="2999-01-01" id="LEGISCTA000006089697">Livre Ier : Des personnes</TITRE_TM>
<TM>
<TITRE_TM debut="1994-07-30" fin="2999-01-01" id="LEGISCTA000006117610">Titre Ier : Des droits civils</TITRE_TM>
<TM>
<TITRE_TM debut="1994-07-30" fin="2004-08-07" id="LEGISCTA000006136060">Chapitre III : De l'étude génétique des caractéristiques d'une personne et de l'identification d'une personne par ses empreintes génétiques.</TITRE_TM>
</TM>
</TM>
</TM>
</TEXTE>
</CONTEXTE>

<VERSIONS>
<VERSION etat="MODIFIE">
<LIEN_ART debut="1994-07-30" etat="MODIFIE" fin="2004-08-07" id="LEGIARTI000006419307" num="16-11" origine="LEGI"/>
</VERSION>
<VERSION etat="MODIFIE">
<LIEN_ART debut="2004-08-07" etat="MODIFIE" fin="2005-07-01" id="LEGIARTI000006419308" num="16-11" origine="LEGI"/>
</VERSION>
<VERSION etat="MODIFIE">
<LIEN_ART debut="2007-07-01" etat="MODIFIE" fin="2011-03-16" id="LEGIARTI000006419309" num="16-11" origine="LEGI"/>
</VERSION>
<VERSION etat="MODIFIE">
<LIEN_ART debut="2011-03-16" etat="MODIFIE" fin="2016-06-05" id="LEGIARTI000023711923" num="16-11" origine="LEGI"/>
</VERSION>
<VERSION etat="MODIFIE">
<LIEN_ART debut="2016-06-05" etat="MODIFIE" fin="2020-01-01" id="LEGIARTI000032656317" num="16-11" origine="LEGI"/>
</VERSION>
<VERSION etat="MODIFIE">
<LIEN_ART debut="2020-01-01" etat="MODIFIE" fin="2023-05-21" id="LEGIARTI000039367698" num="16-11" origine="LEGI"/>
</VERSION>
<VERSION etat="VIGUEUR">
<LIEN_ART debut="2023-05-21" etat="VIGUEUR" fin="2999-01-01" id="LEGIARTI000047569313" num="16-11" origine="LEGI"/>
</VERSION>
</VERSIONS>
<NOTA>
<CONTENU/>
</NOTA>
<BLOC_TEXTUEL>
<CONTENU>
<br/>   L'identification d'une personne par ses empreintes génétiques ne peut être recherchée que dans le cadre de mesures d'enquête ou d'instruction diligentées lors d'une procédure judiciaire ou à des fins médicales ou de recherche scientifique.<br/>
<br/>   En matière civile, cette identification ne peut être recherchée qu'en exécution d'une mesure d'instruction ordonnée par le juge saisi d'une action tendant soit à l'établissement ou la contestation d'un lien de filiation, soit à l'obtention ou la suppression de subsides. Le consentement de l'intéressé doit être préalablement et expressément recueilli.<br/>
<br/>   Lorsque l'identification est effectuée à des fins médicales ou de recherche scientifique, le consentement de la personne doit être au préalable recueilli.<br/>
</CONTENU>
</BLOC_TEXTUEL>
<LIENS>
<LIEN cidtexte="JORFTEXT000000549619" datesignatexte="1994-07-29" id="JORFTEXT000000549619" naturetexte="LOI" nortexte="JUSX9400024L" num="" numtexte="94-653" sens="source" typelien="SPEC_APPLI">Loi 94-653 1994-07-29 art. 11 *champ d'application*</LIEN>
<LIEN cidtexte="JORFTEXT000000549619" datesignatexte="1994-07-29" id="LEGIARTI000006284449" naturetexte="LOI" nortexte="JUSX9400024L" num="5" numtexte="94-653" sens="source" typelien="CREATION">Loi n°94-653 du 29 juillet 1994 - art. 5 () JORF 30 juillet 1994</LIEN>
<LIEN cidtexte="JORFTEXT000000382296" datesignatexte="1997-02-06" id="LEGIARTI000032714078" naturetexte="DECRET" nortexte="JUSC9620898D" num="1" numtexte="97-109" sens="cible" typelien="CITATION">Décret n°97-109 du 6 février 1997 - art. 1 (V)</LIEN>
<LIEN cidtexte="JORFTEXT000000382296" datesignatexte="1997-02-06" id="LEGIARTI000032714165" naturetexte="DECRET" nortexte="JUSC9620898D" num="13" numtexte="97-109" sens="cible" typelien="CITATION">Décret n°97-109 du 6 février 1997 - art. 13 (V)</LIEN>





<LIEN cidtexte="JORFTEXT000025241330" datesignatexte="2012-01-30" id="LEGIARTI000025262933" naturetexte="DECRET" nortexte="IOCD1129420D" num="1" numtexte="2012-125" sens="cible" typelien="CITATION">Décret n°2012-125
 du 30 janvier 2012 - art. 1 (V)</LIEN>
<LIEN cidtexte="JORFTEXT000025241330" datesignatexte="2012-01-30" id="LEGIARTI000025262962" naturetexte="DECRET" nortexte="IOCD1129420D" num="3" numtexte="2012-125" sens="cible" typelien="CITATION">Décret n°2012-125
 du 30 janvier 2012 - art. 3 (V)</LIEN>
<LIEN cidtexte="JORFTEXT000025241330" datesignatexte="2012-01-30" id="LEGIARTI000025262969" naturetexte="DECRET" nortexte="IOCD1129420D" num="4" numtexte="2012-125" sens="cible" typelien="CITATION">Décret n°2012-125
 du 30 janvier 2012 - art. 4 (V)</LIEN>
<LIEN cidtexte="JORFTEXT000025241330" datesignatexte="2012-01-30" id="LEGIARTI000025262953" naturetexte="DECRET" nortexte="IOCD1129420D" num="5" numtexte="2012-125" sens="cible" typelien="CITATION">Décret n°2012-125
 du 30 janvier 2012 - art. 5 (V)</LIEN>

<LIEN cidtexte="JORFTEXT000031560966" datesignatexte="2015-12-02" id="LEGITEXT000031561671" naturetexte="DECRET" nortexte="INTD1514591D" num="" numtexte="2015-1580" sens="cible" typelien="CITATION">DÉCRET n°2015-1580 du 2 décembre 2015 (V)</LIEN>

<LIEN cidtexte="JORFTEXT000032711627" datesignatexte="2016-06-14" id="LEGIARTI000032712139" naturetexte="DECRET" nortexte="JUSD1607271D" num="15" numtexte="2016-796" sens="cible" typelien="CITATION">Décret n°2016-796 du 14 juin 2016 - art. 15</LIEN>

<LIEN cidtexte="LEGITEXT000006072665" datesignatexte="2999-01-01" id="LEGIARTI000043890140" naturetexte="CODE" nortexte="" num="L1130-3" numtexte="" sens="cible" typelien="CITATION">Code de la santé publique - art. L1130-3 (V)</LIEN>
<LIEN cidtexte="LEGITEXT000006072665" datesignatexte="2999-01-01" id="LEGIARTI000025139320" naturetexte="CODE" nortexte="" num="L1131-5" numtexte="" sens="cible" typelien="CITATION">Code de la santé publique - art. L1131-5 (V)</LIEN>
<LIEN cidtexte="LEGITEXT000006072665" datesignatexte="2999-01-01" id="LEGIARTI000023908471" naturetexte="CODE" nortexte="" num="L1133-3" numtexte="" sens="cible" typelien="CITATION">Code de la santé publique - art. L1133-3 (V)</LIEN>
<LIEN cidtexte="LEGITEXT000006072665" datesignatexte="2999-01-01" id="LEGIARTI000006695135" naturetexte="CODE" nortexte="" num="L761-24" numtexte="" sens="cible" typelien="CITATION">Code de la santé publique - art. L761-24 (Ab)</LIEN>
<LIEN cidtexte="LEGITEXT000006071154" datesignatexte="2999-01-01" id="LEGIARTI000054478218" naturetexte="CODE" nortexte="" num="706-54" numtexte="" sens="cible" typelien="CITATION">Code de procédure pénale - art. 706-54 (VT)</LIEN>
<LIEN cidtexte="LEGITEXT000006071154" datesignatexte="2999-01-01" id="LEGIARTI000039344897" naturetexte="CODE" nortexte="" num="R93" numtexte="" sens="cible" typelien="CITATION">Code de procédure pénale - art. R93 (VD)</LIEN>
<LIEN cidtexte="LEGITEXT000006070719" datesignatexte="2999-01-01" id="LEGIARTI000023711940" naturetexte="CODE" nortexte="" num="226-27" numtexte="" sens="cible" typelien="CITATION">Code pénal - art. 226-27 (V)</LIEN>
<LIEN cidtexte="LEGITEXT000006070719" datesignatexte="2999-01-01" id="LEGIARTI000042919801" naturetexte="CODE" nortexte="" num="226-28" numtexte="" sens="cible" typelien="CITATION">Code pénal - art. 226-28 (V)</LIEN>
<LIEN cidtexte="" datesignatexte="" id="" naturetexte="" nortexte="" num="" numtexte="" sens="source" typelien="CODIFICATION">Loi 1803-03-08</LIEN>
</LIENS>
</ARTICLE>
"#,
    },
    // art. 16-11, version 2004-08-07 → 2005-07-01, avant la lacune
    Extract {
        member: "legi/global/code_et_TNC_en_vigueur/code_en_vigueur/LEGI/TEXT/00/00/06/07/07/LEGITEXT000006070721/article/LEGI/ARTI/00/00/06/41/93/LEGIARTI000006419308.xml",
        sha256: "a598d9b27a2845d0accddbec9bdafe46bd4b6c4071088137ea1454438ce76ef1",
        xml: r#"<?xml version="1.0" encoding="UTF-8"?>
<ARTICLE>
<META>
<META_COMMUN>
<ID>LEGIARTI000006419308</ID>
<ANCIEN_ID>ACAXXXXXXXX5X00016LAXXAB</ANCIEN_ID>
<ORIGINE>LEGI</ORIGINE>
<URL>article/LEGI/ARTI/00/00/06/41/93/LEGIARTI000006419308.xml</URL>
<NATURE>Article</NATURE>
</META_COMMUN>
<META_SPEC>
<META_ARTICLE>

<NUM>16-11</NUM>

<ETAT>MODIFIE</ETAT>

<DATE_DEBUT>2004-08-07</DATE_DEBUT>
<DATE_FIN>2005-07-01</DATE_FIN>



<TYPE>AUTONOME</TYPE>
</META_ARTICLE>
</META_SPEC>
</META>
<CONTEXTE>
<TEXTE autorite="" cid="LEGITEXT000006070721" date_publi="2999-01-01" date_signature="2999-01-01" ministere="" nature="CODE" nor="" num="" num_parution_jo="">
<TITRE_TXT c_titre_court="Code civil" debut="1804-03-21" fin="2999-01-01" id_txt="LEGITEXT000006070721">Code civil</TITRE_TXT>
<TM>
<TITRE_TM debut="1804-03-21" fin="2999-01-01" id="LEGISCTA000006089697">Livre Ier : Des personnes</TITRE_TM>
<TM>
<TITRE_TM debut="1994-07-30" fin="2999-01-01" id="LEGISCTA000006117610">Titre Ier : Des droits civils</TITRE_TM>
<TM>
<TITRE_TM debut="2004-08-07" fin="2999-01-01" id="LEGISCTA000006136513">Chapitre III : De l'examen des caractéristiques génétiques d'une personne et de l'identification d'une personne par ses empreintes génétiques</TITRE_TM>
</TM>
</TM>
</TM>
</TEXTE>
</CONTEXTE>

<VERSIONS>
<VERSION etat="MODIFIE">
<LIEN_ART debut="1994-07-30" etat="MODIFIE" fin="2004-08-07" id="LEGIARTI000006419307" num="16-11" origine="LEGI"/>
</VERSION>
<VERSION etat="MODIFIE">
<LIEN_ART debut="2004-08-07" etat="MODIFIE" fin="2005-07-01" id="LEGIARTI000006419308" num="16-11" origine="LEGI"/>
</VERSION>
<VERSION etat="MODIFIE">
<LIEN_ART debut="2007-07-01" etat="MODIFIE" fin="2011-03-16" id="LEGIARTI000006419309" num="16-11" origine="LEGI"/>
</VERSION>
<VERSION etat="MODIFIE">
<LIEN_ART debut="2011-03-16" etat="MODIFIE" fin="2016-06-05" id="LEGIARTI000023711923" num="16-11" origine="LEGI"/>
</VERSION>
<VERSION etat="MODIFIE">
<LIEN_ART debut="2016-06-05" etat="MODIFIE" fin="2020-01-01" id="LEGIARTI000032656317" num="16-11" origine="LEGI"/>
</VERSION>
<VERSION etat="MODIFIE">
<LIEN_ART debut="2020-01-01" etat="MODIFIE" fin="2023-05-21" id="LEGIARTI000039367698" num="16-11" origine="LEGI"/>
</VERSION>
<VERSION etat="VIGUEUR">
<LIEN_ART debut="2023-05-21" etat="VIGUEUR" fin="2999-01-01" id="LEGIARTI000047569313" num="16-11" origine="LEGI"/>
</VERSION>
</VERSIONS>
<NOTA>
<CONTENU/>
</NOTA>
<BLOC_TEXTUEL>
<CONTENU>
<br/>   L'identification d'une personne par ses empreintes génétiques ne peut être recherchée que dans le cadre de mesures d'enquête ou d'instruction diligentées lors d'une procédure judiciaire ou à des fins médicales ou de recherche scientifique.<br/>
<br/>   En matière civile, cette identification ne peut être recherchée qu'en exécution d'une mesure d'instruction ordonnée par le juge saisi d'une action tendant soit à l'établissement ou la contestation d'un lien de filiation, soit à l'obtention ou la suppression de subsides. Le consentement de l'intéressé doit être préalablement et expressément recueilli. Sauf accord exprès de la personne manifesté de son vivant, aucune identification par empreintes génétiques ne peut être réalisée après sa mort.<br/>
<br/>   Lorsque l'identification est effectuée à des fins médicales ou de recherche scientifique, le consentement exprès de la personne doit être recueilli par écrit préalablement à la réalisation de l'identification, après qu'elle a été dûment informée de sa nature et de sa finalité. Le consentement mentionne la finalité de l'identification. Il est révocable sans forme et à tout moment.<br/>
</CONTENU>
</BLOC_TEXTUEL>
<LIENS>
<LIEN cidtexte="JORFTEXT000000382296" datesignatexte="1997-02-06" id="LEGIARTI000032714078" naturetexte="DECRET" nortexte="JUSC9620898D" num="1" numtexte="97-109" sens="cible" typelien="CITATION">Décret n°97-109 du 6 février 1997 - art. 1 (V)</LIEN>
<LIEN cidtexte="JORFTEXT000000382296" datesignatexte="1997-02-06" id="LEGIARTI000032714165" naturetexte="DECRET" nortexte="JUSC9620898D" num="13" numtexte="97-109" sens="cible" typelien="CITATION">Décret n°97-109 du 6 février 1997 - art. 13 (V)</LIEN>
<LIEN cidtexte="JORFTEXT000000441469" datesignatexte="2004-08-06" id="LEGIARTI000006697571" naturetexte="LOI" nortexte="SANX0100053L" num="4" numtexte="2004-800" sens="source" typelien="MODIFICATION">Loi n°2004-800 du 6 août 2004 - art. 4 () JORF 7 août 2004</LIEN>
<LIEN cidtexte="JORFTEXT000000441469" datesignatexte="2004-08-06" id="LEGIARTI000006697572" naturetexte="LOI" nortexte="SANX0100053L" num="5" numtexte="2004-800" sens="source" typelien="MODIFICATION">Loi n°2004-800 du 6 août 2004 - art. 5 () JORF 7 août 2004</LIEN>





<LIEN cidtexte="JORFTEXT000025241330" datesignatexte="2012-01-30" id="LEGIARTI000025262933" naturetexte="DECRET" nortexte="IOCD1129420D" num="1" numtexte="2012-125" sens="cible" typelien="CITATION">Décret n°2012-125
 du 30 janvier 2012 - art. 1 (V)</LIEN>
<LIEN cidtexte="JORFTEXT000025241330" datesignatexte="2012-01-30" id="LEGIARTI000025262962" naturetexte="DECRET" nortexte="IOCD1129420D" num="3" numtexte="2012-125" sens="cible" typelien="CITATION">Décret n°2012-125
 du 30 janvier 2012 - art. 3 (V)</LIEN>
<LIEN cidtexte="JORFTEXT000025241330" datesignatexte="2012-01-30" id="LEGIARTI000025262969" naturetexte="DECRET" nortexte="IOCD1129420D" num="4" numtexte="2012-125" sens="cible" typelien="CITATION">Décret n°2012-125
 du 30 janvier 2012 - art. 4 (V)</LIEN>
<LIEN cidtexte="JORFTEXT000025241330" datesignatexte="2012-01-30" id="LEGIARTI000025262953" naturetexte="DECRET" nortexte="IOCD1129420D" num="5" numtexte="2012-125" sens="cible" typelien="CITATION">Décret n°2012-125
 du 30 janvier 2012 - art. 5 (V)</LIEN>

<LIEN cidtexte="JORFTEXT000031560966" datesignatexte="2015-12-02" id="LEGITEXT000031561671" naturetexte="DECRET" nortexte="INTD1514591D" num="" numtexte="2015-1580" sens="cible" typelien="CITATION">DÉCRET n°2015-1580 du 2 décembre 2015 (V)</LIEN>

<LIEN cidtexte="JORFTEXT000032711627" datesignatexte="2016-06-14" id="LEGIARTI000032712139" naturetexte="DECRET" nortexte="JUSD1607271D" num="15" numtexte="2016-796" sens="cible" typelien="CITATION">Décret n°2016-796 du 14 juin 2016 - art. 15</LIEN>

<LIEN cidtexte="LEGITEXT000006072665" datesignatexte="2999-01-01" id="LEGIARTI000043890140" naturetexte="CODE" nortexte="" num="L1130-3" numtexte="" sens="cible" typelien="CITATION">Code de la santé publique - art. L1130-3 (V)</LIEN>
<LIEN cidtexte="LEGITEXT000006072665" datesignatexte="2999-01-01" id="LEGIARTI000025139320" naturetexte="CODE" nortexte="" num="L1131-5" numtexte="" sens="cible" typelien="CITATION">Code de la santé publique - art. L1131-5 (V)</LIEN>
<LIEN cidtexte="LEGITEXT000006072665" datesignatexte="2999-01-01" id="LEGIARTI000023908471" naturetexte="CODE" nortexte="" num="L1133-3" numtexte="" sens="cible" typelien="CITATION">Code de la santé publique - art. L1133-3 (V)</LIEN>
<LIEN cidtexte="LEGITEXT000006072665" datesignatexte="2999-01-01" id="LEGIARTI000006695135" naturetexte="CODE" nortexte="" num="L761-24" numtexte="" sens="cible" typelien="CITATION">Code de la santé publique - art. L761-24 (Ab)</LIEN>
<LIEN cidtexte="LEGITEXT000006071154" datesignatexte="2999-01-01" id="LEGIARTI000054478218" naturetexte="CODE" nortexte="" num="706-54" numtexte="" sens="cible" typelien="CITATION">Code de procédure pénale - art. 706-54 (VT)</LIEN>
<LIEN cidtexte="LEGITEXT000006071154" datesignatexte="2999-01-01" id="LEGIARTI000039344897" naturetexte="CODE" nortexte="" num="R93" numtexte="" sens="cible" typelien="CITATION">Code de procédure pénale - art. R93 (VD)</LIEN>
<LIEN cidtexte="LEGITEXT000006070719" datesignatexte="2999-01-01" id="LEGIARTI000023711940" naturetexte="CODE" nortexte="" num="226-27" numtexte="" sens="cible" typelien="CITATION">Code pénal - art. 226-27 (V)</LIEN>
<LIEN cidtexte="LEGITEXT000006070719" datesignatexte="2999-01-01" id="LEGIARTI000042919801" naturetexte="CODE" nortexte="" num="226-28" numtexte="" sens="cible" typelien="CITATION">Code pénal - art. 226-28 (V)</LIEN>
<LIEN cidtexte="" datesignatexte="" id="" naturetexte="" nortexte="" num="" numtexte="" sens="source" typelien="CODIFICATION">Loi 1803-03-08</LIEN>
</LIENS>
</ARTICLE>
"#,
    },
    // art. 16-11, version 2007-07-01 → 2011-03-16, après la lacune
    Extract {
        member: "legi/global/code_et_TNC_en_vigueur/code_en_vigueur/LEGI/TEXT/00/00/06/07/07/LEGITEXT000006070721/article/LEGI/ARTI/00/00/06/41/93/LEGIARTI000006419309.xml",
        sha256: "fc8cc6b9242bbb15118777a85ef445ca5aca1012d2689f5cc394f24f1923fd11",
        xml: r#"<?xml version="1.0" encoding="UTF-8"?>
<ARTICLE>
<META>
<META_COMMUN>
<ID>LEGIARTI000006419309</ID>
<ANCIEN_ID>ACAXXXXXXXX5X00016LAXXAC</ANCIEN_ID>
<ORIGINE>LEGI</ORIGINE>
<URL>article/LEGI/ARTI/00/00/06/41/93/LEGIARTI000006419309.xml</URL>
<NATURE>Article</NATURE>
</META_COMMUN>
<META_SPEC>
<META_ARTICLE>

<NUM>16-11</NUM>

<ETAT>MODIFIE</ETAT>

<DATE_DEBUT>2007-07-01</DATE_DEBUT>
<DATE_FIN>2011-03-16</DATE_FIN>



<TYPE>AUTONOME</TYPE>
</META_ARTICLE>
</META_SPEC>
</META>
<CONTEXTE>
<TEXTE autorite="" cid="LEGITEXT000006070721" date_publi="2999-01-01" date_signature="2999-01-01" ministere="" nature="CODE" nor="" num="" num_parution_jo="">
<TITRE_TXT c_titre_court="Code civil" debut="1804-03-21" fin="2999-01-01" id_txt="LEGITEXT000006070721">Code civil</TITRE_TXT>
<TM>
<TITRE_TM debut="1804-03-21" fin="2999-01-01" id="LEGISCTA000006089697">Livre Ier : Des personnes</TITRE_TM>
<TM>
<TITRE_TM debut="1994-07-30" fin="2999-01-01" id="LEGISCTA000006117610">Titre Ier : Des droits civils</TITRE_TM>
<TM>
<TITRE_TM debut="2004-08-07" fin="2999-01-01" id="LEGISCTA000006136513">Chapitre III : De l'examen des caractéristiques génétiques d'une personne et de l'identification d'une personne par ses empreintes génétiques</TITRE_TM>
</TM>
</TM>
</TM>
</TEXTE>
</CONTEXTE>

<VERSIONS>
<VERSION etat="MODIFIE">
<LIEN_ART debut="1994-07-30" etat="MODIFIE" fin="2004-08-07" id="LEGIARTI000006419307" num="16-11" origine="LEGI"/>
</VERSION>
<VERSION etat="MODIFIE">
<LIEN_ART debut="2004-08-07" etat="MODIFIE" fin="2005-07-01" id="LEGIARTI000006419308" num="16-11" origine="LEGI"/>
</VERSION>
<VERSION etat="MODIFIE">
<LIEN_ART debut="2007-07-01" etat="MODIFIE" fin="2011-03-16" id="LEGIARTI000006419309" num="16-11" origine="LEGI"/>
</VERSION>
<VERSION etat="MODIFIE">
<LIEN_ART debut="2011-03-16" etat="MODIFIE" fin="2016-06-05" id="LEGIARTI000023711923" num="16-11" origine="LEGI"/>
</VERSION>
<VERSION etat="MODIFIE">
<LIEN_ART debut="2016-06-05" etat="MODIFIE" fin="2020-01-01" id="LEGIARTI000032656317" num="16-11" origine="LEGI"/>
</VERSION>
<VERSION etat="MODIFIE">
<LIEN_ART debut="2020-01-01" etat="MODIFIE" fin="2023-05-21" id="LEGIARTI000039367698" num="16-11" origine="LEGI"/>
</VERSION>
<VERSION etat="VIGUEUR">
<LIEN_ART debut="2023-05-21" etat="VIGUEUR" fin="2999-01-01" id="LEGIARTI000047569313" num="16-11" origine="LEGI"/>
</VERSION>
</VERSIONS>
<NOTA>
<CONTENU/>
</NOTA>
<BLOC_TEXTUEL>
<CONTENU>
<br/>   L'identification d'une personne par ses empreintes génétiques ne peut être recherchée que dans le cadre de mesures d'enquête ou d'instruction diligentée lors d'une procédure judiciaire ou à des fins médicales ou de recherche scientifique ou d'identification d'un militaire décédé à l'occasion d'une opération conduite par les forces armées ou les formations rattachées.<br/>
<br/>   En matière civile, cette identification ne peut être recherchée qu'en exécution d'une mesure d'instruction ordonnée par le juge saisi d'une action tendant soit à l'établissement ou la contestation d'un lien de filiation, soit à l'obtention ou la suppression de subsides. Le consentement de l'intéressé doit être préalablement et expressément recueilli. Sauf accord exprès de la personne manifesté de son vivant, aucune identification par empreintes génétiques ne peut être réalisée après sa mort.<br/>
<br/>   Lorsque l'identification est effectuée à des fins médicales ou de recherche scientifique, le consentement exprès de la personne doit être recueilli par écrit préalablement à la réalisation de l'identification, après qu'elle a été dûment informée de sa nature et de sa finalité. Le consentement mentionne la finalité de l'identification. Il est révocable sans forme et à tout moment.<br/>
</CONTENU>
</BLOC_TEXTUEL>
<LIENS>
<LIEN cidtexte="JORFTEXT000000382296" datesignatexte="1997-02-06" id="LEGIARTI000032714078" naturetexte="DECRET" nortexte="JUSC9620898D" num="1" numtexte="97-109" sens="cible" typelien="CITATION">Décret n°97-109 du 6 février 1997 - art. 1 (V)</LIEN>
<LIEN cidtexte="JORFTEXT000000382296" datesignatexte="1997-02-06" id="LEGIARTI000032714165" naturetexte="DECRET" nortexte="JUSC9620898D" num="13" numtexte="97-109" sens="cible" typelien="CITATION">Décret n°97-109 du 6 février 1997 - art. 13 (V)</LIEN>
<LIEN cidtexte="JORFTEXT000000808186" datesignatexte="2005-03-24" id="LEGIARTI000006405098" naturetexte="LOI" nortexte="DEFX0400144L" num="93" numtexte="2005-270" sens="source" typelien="MODIFICATION">Loi n°2005-270 du 24 mars 2005 - art. 93 () JORF 26 mars 2005 en vigueur le 1er juillet 2005</LIEN>





<LIEN cidtexte="JORFTEXT000025241330" datesignatexte="2012-01-30" id="LEGIARTI000025262933" naturetexte="DECRET" nortexte="IOCD1129420D" num="1" numtexte="2012-125" sens="cible" typelien="CITATION">Décret n°2012-125
 du 30 janvier 2012 - art. 1 (V)</LIEN>
<LIEN cidtexte="JORFTEXT000025241330" datesignatexte="2012-01-30" id="LEGIARTI000025262962" naturetexte="DECRET" nortexte="IOCD1129420D" num="3" numtexte="2012-125" sens="cible" typelien="CITATION">Décret n°2012-125
 du 30 janvier 2012 - art. 3 (V)</LIEN>
<LIEN cidtexte="JORFTEXT000025241330" datesignatexte="2012-01-30" id="LEGIARTI000025262969" naturetexte="DECRET" nortexte="IOCD1129420D" num="4" numtexte="2012-125" sens="cible" typelien="CITATION">Décret n°2012-125
 du 30 janvier 2012 - art. 4 (V)</LIEN>
<LIEN cidtexte="JORFTEXT000025241330" datesignatexte="2012-01-30" id="LEGIARTI000025262953" naturetexte="DECRET" nortexte="IOCD1129420D" num="5" numtexte="2012-125" sens="cible" typelien="CITATION">Décret n°2012-125
 du 30 janvier 2012 - art. 5 (V)</LIEN>

<LIEN cidtexte="JORFTEXT000031560966" datesignatexte="2015-12-02" id="LEGITEXT000031561671" naturetexte="DECRET" nortexte="INTD1514591D" num="" numtexte="2015-1580" sens="cible" typelien="CITATION">DÉCRET n°2015-1580 du 2 décembre 2015 (V)</LIEN>

<LIEN cidtexte="JORFTEXT000032711627" datesignatexte="2016-06-14" id="LEGIARTI000032712139" naturetexte="DECRET" nortexte="JUSD1607271D" num="15" numtexte="2016-796" sens="cible" typelien="CITATION">Décret n°2016-796 du 14 juin 2016 - art. 15</LIEN>

<LIEN cidtexte="LEGITEXT000006072665" datesignatexte="2999-01-01" id="LEGIARTI000043890140" naturetexte="CODE" nortexte="" num="L1130-3" numtexte="" sens="cible" typelien="CITATION">Code de la santé publique - art. L1130-3 (V)</LIEN>
<LIEN cidtexte="LEGITEXT000006072665" datesignatexte="2999-01-01" id="LEGIARTI000025139320" naturetexte="CODE" nortexte="" num="L1131-5" numtexte="" sens="cible" typelien="CITATION">Code de la santé publique - art. L1131-5 (V)</LIEN>
<LIEN cidtexte="LEGITEXT000006072665" datesignatexte="2999-01-01" id="LEGIARTI000023908471" naturetexte="CODE" nortexte="" num="L1133-3" numtexte="" sens="cible" typelien="CITATION">Code de la santé publique - art. L1133-3 (V)</LIEN>
<LIEN cidtexte="LEGITEXT000006072665" datesignatexte="2999-01-01" id="LEGIARTI000006695135" naturetexte="CODE" nortexte="" num="L761-24" numtexte="" sens="cible" typelien="CITATION">Code de la santé publique - art. L761-24 (Ab)</LIEN>
<LIEN cidtexte="LEGITEXT000006071154" datesignatexte="2999-01-01" id="LEGIARTI000054478218" naturetexte="CODE" nortexte="" num="706-54" numtexte="" sens="cible" typelien="CITATION">Code de procédure pénale - art. 706-54 (VT)</LIEN>
<LIEN cidtexte="LEGITEXT000006071154" datesignatexte="2999-01-01" id="LEGIARTI000039344897" naturetexte="CODE" nortexte="" num="R93" numtexte="" sens="cible" typelien="CITATION">Code de procédure pénale - art. R93 (VD)</LIEN>
<LIEN cidtexte="LEGITEXT000006070719" datesignatexte="2999-01-01" id="LEGIARTI000023711940" naturetexte="CODE" nortexte="" num="226-27" numtexte="" sens="cible" typelien="CITATION">Code pénal - art. 226-27 (V)</LIEN>
<LIEN cidtexte="LEGITEXT000006070719" datesignatexte="2999-01-01" id="LEGIARTI000042919801" naturetexte="CODE" nortexte="" num="226-28" numtexte="" sens="cible" typelien="CITATION">Code pénal - art. 226-28 (V)</LIEN>
<LIEN cidtexte="" datesignatexte="" id="" naturetexte="" nortexte="" num="" numtexte="" sens="source" typelien="CODIFICATION">Loi 1803-03-08</LIEN>
</LIENS>
</ARTICLE>
"#,
    },
    // art. 16-11, version 2011-03-16 → 2016-06-05
    Extract {
        member: "legi/global/code_et_TNC_en_vigueur/code_en_vigueur/LEGI/TEXT/00/00/06/07/07/LEGITEXT000006070721/article/LEGI/ARTI/00/00/23/71/19/LEGIARTI000023711923.xml",
        sha256: "00497f45b12a9bc7789c148b1936fac9e341a5863daecab9950c59ce0c829032",
        xml: r#"<?xml version="1.0" encoding="UTF-8"?>
<ARTICLE>
<META>
<META_COMMUN>
<ID>LEGIARTI000023711923</ID>
<ANCIEN_ID/>
<ORIGINE>LEGI</ORIGINE>
<URL>article/LEGI/ARTI/00/00/23/71/19/LEGIARTI000023711923.xml</URL>
<NATURE>Article</NATURE>
</META_COMMUN>
<META_SPEC>
<META_ARTICLE>

<NUM>16-11</NUM>

<ETAT>MODIFIE</ETAT>

<DATE_DEBUT>2011-03-16</DATE_DEBUT>
<DATE_FIN>2016-06-05</DATE_FIN>



<TYPE>AUTONOME</TYPE>
</META_ARTICLE>
</META_SPEC>
</META>
<CONTEXTE>
<TEXTE autorite="" cid="LEGITEXT000006070721" date_publi="2999-01-01" date_signature="2999-01-01" ministere="" nature="CODE" nor="" num="" num_parution_jo="">
<TITRE_TXT c_titre_court="Code civil" debut="1804-03-21" fin="2999-01-01" id_txt="LEGITEXT000006070721">Code civil</TITRE_TXT>
<TM>
<TITRE_TM debut="1804-03-21" fin="2999-01-01" id="LEGISCTA000006089697">Livre Ier : Des personnes</TITRE_TM>
<TM>
<TITRE_TM debut="1994-07-30" fin="2999-01-01" id="LEGISCTA000006117610">Titre Ier : Des droits civils</TITRE_TM>
<TM>
<TITRE_TM debut="2004-08-07" fin="2999-01-01" id="LEGISCTA000006136513">Chapitre III : De l'examen des caractéristiques génétiques d'une personne et de l'identification d'une personne par ses empreintes génétiques</TITRE_TM>
</TM>
</TM>
</TM>
</TEXTE>
</CONTEXTE>

<VERSIONS>
<VERSION etat="MODIFIE">
<LIEN_ART debut="1994-07-30" etat="MODIFIE" fin="2004-08-07" id="LEGIARTI000006419307" num="16-11" origine="LEGI"/>
</VERSION>
<VERSION etat="MODIFIE">
<LIEN_ART debut="2004-08-07" etat="MODIFIE" fin="2005-07-01" id="LEGIARTI000006419308" num="16-11" origine="LEGI"/>
</VERSION>
<VERSION etat="MODIFIE">
<LIEN_ART debut="2007-07-01" etat="MODIFIE" fin="2011-03-16" id="LEGIARTI000006419309" num="16-11" origine="LEGI"/>
</VERSION>
<VERSION etat="MODIFIE">
<LIEN_ART debut="2011-03-16" etat="MODIFIE" fin="2016-06-05" id="LEGIARTI000023711923" num="16-11" origine="LEGI"/>
</VERSION>
<VERSION etat="MODIFIE">
<LIEN_ART debut="2016-06-05" etat="MODIFIE" fin="2020-01-01" id="LEGIARTI000032656317" num="16-11" origine="LEGI"/>
</VERSION>
<VERSION etat="MODIFIE">
<LIEN_ART debut="2020-01-01" etat="MODIFIE" fin="2023-05-21" id="LEGIARTI000039367698" num="16-11" origine="LEGI"/>
</VERSION>
<VERSION etat="VIGUEUR">
<LIEN_ART debut="2023-05-21" etat="VIGUEUR" fin="2999-01-01" id="LEGIARTI000047569313" num="16-11" origine="LEGI"/>
</VERSION>
</VERSIONS>
<NOTA>
<CONTENU/>
</NOTA>
<BLOC_TEXTUEL>
<CONTENU>
<p>L'identification d'une personne par ses empreintes génétiques ne peut être recherchée que : </p>
<p>1° Dans le cadre de mesures d'enquête ou d'instruction diligentées lors d'une procédure judiciaire ; </p>
<p>2° A des fins médicales ou de recherche scientifique ; </p>
<p>3° Aux fins d'établir, lorsqu'elle est inconnue, l'identité de personnes décédées. </p>
<p>En matière civile, cette identification ne peut être recherchée qu'en exécution d'une mesure d'instruction ordonnée par le juge saisi d'une action tendant soit à l'établissement ou la contestation d'un lien de filiation, soit à l'obtention ou la suppression de subsides. Le consentement de l'intéressé doit être préalablement et expressément recueilli. Sauf accord exprès de la personne manifesté de son vivant, aucune identification par empreintes génétiques ne peut être réalisée après sa mort. </p>
<p>Lorsque l'identification est effectuée à des fins médicales ou de recherche scientifique, le consentement exprès de la personne doit être recueilli par écrit préalablement à la réalisation de l'identification, après qu'elle a été dûment informée de sa nature et de sa finalité. Le consentement mentionne la finalité de l'identification. Il est révocable sans forme et à tout moment. </p>
<p>Lorsque la recherche d'identité mentionnée au 3° concerne soit un militaire décédé à l'occasion d'une opération conduite par les forces armées ou les formations rattachées, soit une victime de catastrophe naturelle, soit une personne faisant l'objet de recherches au titre de l'article 26 de la loi n° 95-73 du 21 janvier 1995 d'orientation et de programmation relative à la sécurité et dont la mort est supposée, des prélèvements destinés à recueillir les traces biologiques de cette personne peuvent être réalisés dans des lieux qu'elle est susceptible d'avoir habituellement fréquentés, avec l'accord du responsable des lieux ou, en cas de refus de celui-ci ou d'impossibilité de recueillir cet accord, avec l'autorisation du juge des libertés et de la détention du tribunal de grande instance. Des prélèvements aux mêmes fins sur les ascendants, descendants ou collatéraux supposés de cette personne peuvent être également réalisés. Le consentement exprès de chaque personne concernée est alors recueilli par écrit préalablement à la réalisation du prélèvement, après que celle-ci a été dûment informée de la nature de ce prélèvement, de sa finalité ainsi que du caractère à tout moment révocable de son consentement. Le consentement mentionne la finalité du prélèvement et de l'identification. <br/>
<br/>Les modalités de mise en œuvre des recherches d'identification mentionnées au 3° du présent article sont précisées par décret en Conseil d'Etat.</p>
</CONTENU>
</BLOC_TEXTUEL>
<LIENS>
<LIEN cidtexte="JORFTEXT000000369046" datesignatexte="1995-01-21" id="LEGIARTI000006527982" naturetexte="LOI" nortexte="INTX9400063L" num="26" numtexte="95-73" sens="source" typelien="CITATION">Loi n° 95-73 du 21 janvier 1995 - art. 26</LIEN>
<LIEN cidtexte="JORFTEXT000000382296" datesignatexte="1997-02-06" id="LEGIARTI000032714078" naturetexte="DECRET" nortexte="JUSC9620898D" num="1" numtexte="97-109" sens="cible" typelien="CITATION">Décret n°97-109 du 6 février 1997 - art. 1 (V)</LIEN>
<LIEN cidtexte="JORFTEXT000000382296" datesignatexte="1997-02-06" id="LEGIARTI000032714165" naturetexte="DECRET" nortexte="JUSC9620898D" num="13" numtexte="97-109" sens="cible" typelien="CITATION">Décret n°97-109 du 6 février 1997 - art. 13 (V)</LIEN>
<LIEN cidtexte="JORFTEXT000023707312" datesignatexte="2011-03-14" id="LEGIARTI000023708917" naturetexte="LOI" nortexte="IOCX0903274L" num="6" numtexte="2011-267" sens="cible" typelien="MODIFIE">LOI n°2011-267
 du 14 mars 2011 - art. 6</LIEN>





<LIEN cidtexte="JORFTEXT000025241330" datesignatexte="2012-01-30" id="LEGIARTI000025262933" naturetexte="DECRET" nortexte="IOCD1129420D" num="1" numtexte="2012-125" sens="cible" typelien="CITATION">Décret n°2012-125
 du 30 janvier 2012 - art. 1 (V)</LIEN>
<LIEN cidtexte="JORFTEXT000025241330" datesignatexte="2012-01-30" id="LEGIARTI000025262962" naturetexte="DECRET" nortexte="IOCD1129420D" num="3" numtexte="2012-125" sens="cible" typelien="CITATION">Décret n°2012-125
 du 30 janvier 2012 - art. 3 (V)</LIEN>
<LIEN cidtexte="JORFTEXT000025241330" datesignatexte="2012-01-30" id="LEGIARTI000025262969" naturetexte="DECRET" nortexte="IOCD1129420D" num="4" numtexte="2012-125" sens="cible" typelien="CITATION">Décret n°2012-125
 du 30 janvier 2012 - art. 4 (V)</LIEN>
<LIEN cidtexte="JORFTEXT000025241330" datesignatexte="2012-01-30" id="LEGIARTI000025262953" naturetexte="DECRET" nortexte="IOCD1129420D" num="5" numtexte="2012-125" sens="cible" typelien="CITATION">Décret n°2012-125
 du 30 janvier 2012 - art. 5 (V)</LIEN>

<LIEN cidtexte="JORFTEXT000031560966" datesignatexte="2015-12-02" id="LEGITEXT000031561671" naturetexte="DECRET" nortexte="INTD1514591D" num="" numtexte="2015-1580" sens="cible" typelien="CITATION">DÉCRET n°2015-1580 du 2 décembre 2015 (V)</LIEN>

<LIEN cidtexte="JORFTEXT000032711627" datesignatexte="2016-06-14" id="LEGIARTI000032712139" naturetexte="DECRET" nortexte="JUSD1607271D" num="15" numtexte="2016-796" sens="cible" typelien="CITATION">Décret n°2016-796 du 14 juin 2016 - art. 15</LIEN>

<LIEN cidtexte="LEGITEXT000006072665" datesignatexte="2999-01-01" id="LEGIARTI000043890140" naturetexte="CODE" nortexte="" num="L1130-3" numtexte="" sens="cible" typelien="CITATION">Code de la santé publique - art. L1130-3 (V)</LIEN>
<LIEN cidtexte="LEGITEXT000006072665" datesignatexte="2999-01-01" id="LEGIARTI000025139320" naturetexte="CODE" nortexte="" num="L1131-5" numtexte="" sens="cible" typelien="CITATION">Code de la santé publique - art. L1131-5 (V)</LIEN>
<LIEN cidtexte="LEGITEXT000006072665" datesignatexte="2999-01-01" id="LEGIARTI000023908471" naturetexte="CODE" nortexte="" num="L1133-3" numtexte="" sens="cible" typelien="CITATION">Code de la santé publique - art. L1133-3 (V)</LIEN>
<LIEN cidtexte="LEGITEXT000006072665" datesignatexte="2999-01-01" id="LEGIARTI000006695135" naturetexte="CODE" nortexte="" num="L761-24" numtexte="" sens="cible" typelien="CITATION">Code de la santé publique - art. L761-24 (Ab)</LIEN>
<LIEN cidtexte="LEGITEXT000006071154" datesignatexte="2999-01-01" id="LEGIARTI000054478218" naturetexte="CODE" nortexte="" num="706-54" numtexte="" sens="cible" typelien="CITATION">Code de procédure pénale - art. 706-54 (VT)</LIEN>
<LIEN cidtexte="LEGITEXT000006071154" datesignatexte="2999-01-01" id="LEGIARTI000039344897" naturetexte="CODE" nortexte="" num="R93" numtexte="" sens="cible" typelien="CITATION">Code de procédure pénale - art. R93 (VD)</LIEN>
<LIEN cidtexte="LEGITEXT000006070719" datesignatexte="2999-01-01" id="LEGIARTI000023711940" naturetexte="CODE" nortexte="" num="226-27" numtexte="" sens="cible" typelien="CITATION">Code pénal - art. 226-27 (V)</LIEN>
<LIEN cidtexte="LEGITEXT000006070719" datesignatexte="2999-01-01" id="LEGIARTI000042919801" naturetexte="CODE" nortexte="" num="226-28" numtexte="" sens="cible" typelien="CITATION">Code pénal - art. 226-28 (V)</LIEN>
<LIEN cidtexte="" datesignatexte="" id="" naturetexte="" nortexte="" num="" numtexte="" sens="source" typelien="CODIFICATION">Loi 1803-03-08</LIEN>
</LIENS>
</ARTICLE>
"#,
    },
    // art. 16-11, version 2016-06-05 → 2020-01-01
    Extract {
        member: "legi/global/code_et_TNC_en_vigueur/code_en_vigueur/LEGI/TEXT/00/00/06/07/07/LEGITEXT000006070721/article/LEGI/ARTI/00/00/32/65/63/LEGIARTI000032656317.xml",
        sha256: "1b3ae3268208c373a86419453d7529ac58b6289696b5d38d34e300e375ee3edc",
        xml: r#"<?xml version="1.0" encoding="UTF-8"?>
<ARTICLE>
<META>
<META_COMMUN>
<ID>LEGIARTI000032656317</ID>
<ANCIEN_ID/>
<ORIGINE>LEGI</ORIGINE>
<URL>article/LEGI/ARTI/00/00/32/65/63/LEGIARTI000032656317.xml</URL>
<NATURE>Article</NATURE>
</META_COMMUN>
<META_SPEC>
<META_ARTICLE>

<NUM>16-11</NUM>

<ETAT>MODIFIE</ETAT>

<DATE_DEBUT>2016-06-05</DATE_DEBUT>
<DATE_FIN>2020-01-01</DATE_FIN>



<TYPE>AUTONOME</TYPE>
</META_ARTICLE>
</META_SPEC>
</META>
<CONTEXTE>
<TEXTE autorite="" cid="LEGITEXT000006070721" date_publi="2999-01-01" date_signature="2999-01-01" ministere="" nature="CODE" nor="" num="" num_parution_jo="">
<TITRE_TXT c_titre_court="Code civil" debut="1804-03-21" fin="2999-01-01" id_txt="LEGITEXT000006070721">Code civil</TITRE_TXT>
<TM>
<TITRE_TM debut="1804-03-21" fin="2999-01-01" id="LEGISCTA000006089697">Livre Ier : Des personnes</TITRE_TM>
<TM>
<TITRE_TM debut="1994-07-30" fin="2999-01-01" id="LEGISCTA000006117610">Titre Ier : Des droits civils</TITRE_TM>
<TM>
<TITRE_TM debut="2004-08-07" fin="2999-01-01" id="LEGISCTA000006136513">Chapitre III : De l'examen des caractéristiques génétiques d'une personne et de l'identification d'une personne par ses empreintes génétiques</TITRE_TM>
</TM>
</TM>
</TM>
</TEXTE>
</CONTEXTE>

<VERSIONS>
<VERSION etat="MODIFIE">
<LIEN_ART debut="1994-07-30" etat="MODIFIE" fin="2004-08-07" id="LEGIARTI000006419307" num="16-11" origine="LEGI"/>
</VERSION>
<VERSION etat="MODIFIE">
<LIEN_ART debut="2004-08-07" etat="MODIFIE" fin="2005-07-01" id="LEGIARTI000006419308" num="16-11" origine="LEGI"/>
</VERSION>
<VERSION etat="MODIFIE">
<LIEN_ART debut="2007-07-01" etat="MODIFIE" fin="2011-03-16" id="LEGIARTI000006419309" num="16-11" origine="LEGI"/>
</VERSION>
<VERSION etat="MODIFIE">
<LIEN_ART debut="2011-03-16" etat="MODIFIE" fin="2016-06-05" id="LEGIARTI000023711923" num="16-11" origine="LEGI"/>
</VERSION>
<VERSION etat="MODIFIE">
<LIEN_ART debut="2016-06-05" etat="MODIFIE" fin="2020-01-01" id="LEGIARTI000032656317" num="16-11" origine="LEGI"/>
</VERSION>
<VERSION etat="MODIFIE">
<LIEN_ART debut="2020-01-01" etat="MODIFIE" fin="2023-05-21" id="LEGIARTI000039367698" num="16-11" origine="LEGI"/>
</VERSION>
<VERSION etat="VIGUEUR">
<LIEN_ART debut="2023-05-21" etat="VIGUEUR" fin="2999-01-01" id="LEGIARTI000047569313" num="16-11" origine="LEGI"/>
</VERSION>
</VERSIONS>
<NOTA>
<CONTENU/>
</NOTA>
<BLOC_TEXTUEL>
<CONTENU>
<p>L'identification d'une personne par ses empreintes génétiques ne peut être recherchée que : </p>
<br/>
<p>1° Dans le cadre de mesures d'enquête ou d'instruction diligentées lors d'une procédure judiciaire ; </p>
<br/>
<p>2° A des fins médicales ou de recherche scientifique ; </p>
<br/>
<p>3° Aux fins d'établir, lorsqu'elle est inconnue, l'identité de personnes décédées ;</p>
<br/>
<p>4° Dans les conditions prévues à l'article L. 2381-1 du code de la défense.  </p>
<br/>
<p>En matière civile, cette identification ne peut être recherchée qu'en exécution d'une mesure d'instruction ordonnée par le juge saisi d'une action tendant soit à l'établissement ou la contestation d'un lien de filiation, soit à l'obtention ou la suppression de subsides. Le consentement de l'intéressé doit être préalablement et expressément recueilli. Sauf accord exprès de la personne manifesté de son vivant, aucune identification par empreintes génétiques ne peut être réalisée après sa mort. </p>
<br/>
<p>Lorsque l'identification est effectuée à des fins médicales ou de recherche scientifique, le consentement exprès de la personne doit être recueilli par écrit préalablement à la réalisation de l'identification, après qu'elle a été dûment informée de sa nature et de sa finalité. Le consentement mentionne la finalité de l'identification. Il est révocable sans forme et à tout moment. </p>
<br/>
<p>Lorsque la recherche d'identité mentionnée au 3° concerne soit un militaire décédé à l'occasion d'une opération conduite par les forces armées ou les formations rattachées, soit une victime de catastrophe naturelle, soit une personne faisant l'objet de recherches au titre de l'article 26 de la loi n° 95-73 du 21 janvier 1995 d'orientation et de programmation relative à la sécurité et dont la mort est supposée, des prélèvements destinés à recueillir les traces biologiques de cette personne peuvent être réalisés dans des lieux qu'elle est susceptible d'avoir habituellement fréquentés, avec l'accord du responsable des lieux ou, en cas de refus de celui-ci ou d'impossibilité de recueillir cet accord, avec l'autorisation du juge des libertés et de la détention du tribunal de grande instance. Des prélèvements aux mêmes fins sur les ascendants, descendants ou collatéraux supposés de cette personne peuvent être également réalisés. Le consentement exprès de chaque personne concernée est alors recueilli par écrit préalablement à la réalisation du prélèvement, après que celle-ci a été dûment informée de la nature de ce prélèvement, de sa finalité ainsi que du caractère à tout moment révocable de son consentement. Le consentement mentionne la finalité du prélèvement et de l'identification. </p>
<br/>
<p>Les modalités de mise en œuvre des recherches d'identification mentionnées au 3° du présent article sont précisées par décret en Conseil d'Etat.</p>
</CONTENU>
</BLOC_TEXTUEL>
<LIENS>
<LIEN cidtexte="JORFTEXT000000369046" datesignatexte="1995-01-21" id="LEGIARTI000006527982" naturetexte="LOI" nortexte="INTX9400063L" num="26" numtexte="95-73" sens="source" typelien="CITATION">Loi n° 95-73 du 21 janvier 1995 - art. 26</LIEN>
<LIEN cidtexte="JORFTEXT000000382296" datesignatexte="1997-02-06" id="LEGIARTI000032714078" naturetexte="DECRET" nortexte="JUSC9620898D" num="1" numtexte="97-109" sens="cible" typelien="CITATION">Décret n°97-109 du 6 février 1997 - art. 1 (V)</LIEN>
<LIEN cidtexte="JORFTEXT000000382296" datesignatexte="1997-02-06" id="LEGIARTI000032714165" naturetexte="DECRET" nortexte="JUSC9620898D" num="13" numtexte="97-109" sens="cible" typelien="CITATION">Décret n°97-109 du 6 février 1997 - art. 13 (V)</LIEN>





<LIEN cidtexte="JORFTEXT000025241330" datesignatexte="2012-01-30" id="LEGIARTI000025262933" naturetexte="DECRET" nortexte="IOCD1129420D" num="1" numtexte="2012-125" sens="cible" typelien="CITATION">Décret n°2012-125
 du 30 janvier 2012 - art. 1 (V)</LIEN>
<LIEN cidtexte="JORFTEXT000025241330" datesignatexte="2012-01-30" id="LEGIARTI000025262962" naturetexte="DECRET" nortexte="IOCD1129420D" num="3" numtexte="2012-125" sens="cible" typelien="CITATION">Décret n°2012-125
 du 30 janvier 2012 - art. 3 (V)</LIEN>
<LIEN cidtexte="JORFTEXT000025241330" datesignatexte="2012-01-30" id="LEGIARTI000025262969" naturetexte="DECRET" nortexte="IOCD1129420D" num="4" numtexte="2012-125" sens="cible" typelien="CITATION">Décret n°2012-125
 du 30 janvier 2012 - art. 4 (V)</LIEN>
<LIEN cidtexte="JORFTEXT000025241330" datesignatexte="2012-01-30" id="LEGIARTI000025262953" naturetexte="DECRET" nortexte="IOCD1129420D" num="5" numtexte="2012-125" sens="cible" typelien="CITATION">Décret n°2012-125
 du 30 janvier 2012 - art. 5 (V)</LIEN>

<LIEN cidtexte="JORFTEXT000031560966" datesignatexte="2015-12-02" id="LEGITEXT000031561671" naturetexte="DECRET" nortexte="INTD1514591D" num="" numtexte="2015-1580" sens="cible" typelien="CITATION">DÉCRET n°2015-1580 du 2 décembre 2015 (V)</LIEN>

<LIEN cidtexte="JORFTEXT000032627231" datesignatexte="2016-06-03" id="LEGIARTI000032631385" naturetexte="LOI" nortexte="JUSD1532276L" num="116" numtexte="2016-731" sens="cible" typelien="MODIFIE">LOI n°2016-731 du 3 juin 2016 - art. 116</LIEN>
<LIEN cidtexte="JORFTEXT000032711627" datesignatexte="2016-06-14" id="LEGIARTI000032712139" naturetexte="DECRET" nortexte="JUSD1607271D" num="15" numtexte="2016-796" sens="cible" typelien="CITATION">Décret n°2016-796 du 14 juin 2016 - art. 15</LIEN>

<LIEN cidtexte="LEGITEXT000006072665" datesignatexte="2999-01-01" id="LEGIARTI000043890140" naturetexte="CODE" nortexte="" num="L1130-3" numtexte="" sens="cible" typelien="CITATION">Code de la santé publique - art. L1130-3 (V)</LIEN>
<LIEN cidtexte="LEGITEXT000006072665" datesignatexte="2999-01-01" id="LEGIARTI000025139320" naturetexte="CODE" nortexte="" num="L1131-5" numtexte="" sens="cible" typelien="CITATION">Code de la santé publique - art. L1131-5 (V)</LIEN>
<LIEN cidtexte="LEGITEXT000006072665" datesignatexte="2999-01-01" id="LEGIARTI000023908471" naturetexte="CODE" nortexte="" num="L1133-3" numtexte="" sens="cible" typelien="CITATION">Code de la santé publique - art. L1133-3 (V)</LIEN>
<LIEN cidtexte="LEGITEXT000006072665" datesignatexte="2999-01-01" id="LEGIARTI000006695135" naturetexte="CODE" nortexte="" num="L761-24" numtexte="" sens="cible" typelien="CITATION">Code de la santé publique - art. L761-24 (Ab)</LIEN>
<LIEN cidtexte="LEGITEXT000006071154" datesignatexte="2999-01-01" id="LEGIARTI000054478218" naturetexte="CODE" nortexte="" num="706-54" numtexte="" sens="cible" typelien="CITATION">Code de procédure pénale - art. 706-54 (VT)</LIEN>
<LIEN cidtexte="LEGITEXT000006071154" datesignatexte="2999-01-01" id="LEGIARTI000039344897" naturetexte="CODE" nortexte="" num="R93" numtexte="" sens="cible" typelien="CITATION">Code de procédure pénale - art. R93 (VD)</LIEN>
<LIEN cidtexte="LEGITEXT000006070719" datesignatexte="2999-01-01" id="LEGIARTI000023711940" naturetexte="CODE" nortexte="" num="226-27" numtexte="" sens="cible" typelien="CITATION">Code pénal - art. 226-27 (V)</LIEN>
<LIEN cidtexte="LEGITEXT000006070719" datesignatexte="2999-01-01" id="LEGIARTI000042919801" naturetexte="CODE" nortexte="" num="226-28" numtexte="" sens="cible" typelien="CITATION">Code pénal - art. 226-28 (V)</LIEN>
<LIEN cidtexte="LEGITEXT000006071307" datesignatexte="2999-01-01" id="LEGIARTI000032646851" naturetexte="CODE" nortexte="" num="L2381-1" numtexte="" sens="source" typelien="CITATION">Code de la défense. - art. L2381-1</LIEN>
<LIEN cidtexte="" datesignatexte="" id="" naturetexte="" nortexte="" num="" numtexte="" sens="source" typelien="CODIFICATION">Loi 1803-03-08</LIEN>
</LIENS>
</ARTICLE>
"#,
    },
    // art. 16-11, version 2020-01-01 → 2023-05-21
    Extract {
        member: "legi/global/code_et_TNC_en_vigueur/code_en_vigueur/LEGI/TEXT/00/00/06/07/07/LEGITEXT000006070721/article/LEGI/ARTI/00/00/39/36/76/LEGIARTI000039367698.xml",
        sha256: "bcda9ae8d7fbd72356d621bc1d802f01fbac04ce80546c49860a5b3a415de2d8",
        xml: r#"<?xml version="1.0" encoding="UTF-8"?>
<ARTICLE>
<META>
<META_COMMUN>
<ID>LEGIARTI000039367698</ID>
<ANCIEN_ID/>
<ORIGINE>LEGI</ORIGINE>
<URL>article/LEGI/ARTI/00/00/39/36/76/LEGIARTI000039367698.xml</URL>
<NATURE>Article</NATURE>
</META_COMMUN>
<META_SPEC>
<META_ARTICLE>

<NUM>16-11</NUM>

<ETAT>MODIFIE</ETAT>

<DATE_DEBUT>2020-01-01</DATE_DEBUT>
<DATE_FIN>2023-05-21</DATE_FIN>



<TYPE>AUTONOME</TYPE>
</META_ARTICLE>
</META_SPEC>
</META>
<CONTEXTE>
<TEXTE autorite="" cid="LEGITEXT000006070721" date_publi="2999-01-01" date_signature="2999-01-01" ministere="" nature="CODE" nor="" num="" num_parution_jo="">
<TITRE_TXT c_titre_court="Code civil" debut="1804-03-21" fin="2999-01-01" id_txt="LEGITEXT000006070721">Code civil</TITRE_TXT>
<TM>
<TITRE_TM debut="1804-03-21" fin="2999-01-01" id="LEGISCTA000006089697">Livre Ier : Des personnes</TITRE_TM>
<TM>
<TITRE_TM debut="1994-07-30" fin="2999-01-01" id="LEGISCTA000006117610">Titre Ier : Des droits civils</TITRE_TM>
<TM>
<TITRE_TM debut="2004-08-07" fin="2999-01-01" id="LEGISCTA000006136513">Chapitre III : De l'examen des caractéristiques génétiques d'une personne et de l'identification d'une personne par ses empreintes génétiques</TITRE_TM>
</TM>
</TM>
</TM>
</TEXTE>
</CONTEXTE>

<VERSIONS>
<VERSION etat="MODIFIE">
<LIEN_ART debut="1994-07-30" etat="MODIFIE" fin="2004-08-07" id="LEGIARTI000006419307" num="16-11" origine="LEGI"/>
</VERSION>
<VERSION etat="MODIFIE">
<LIEN_ART debut="2004-08-07" etat="MODIFIE" fin="2005-07-01" id="LEGIARTI000006419308" num="16-11" origine="LEGI"/>
</VERSION>
<VERSION etat="MODIFIE">
<LIEN_ART debut="2007-07-01" etat="MODIFIE" fin="2011-03-16" id="LEGIARTI000006419309" num="16-11" origine="LEGI"/>
</VERSION>
<VERSION etat="MODIFIE">
<LIEN_ART debut="2011-03-16" etat="MODIFIE" fin="2016-06-05" id="LEGIARTI000023711923" num="16-11" origine="LEGI"/>
</VERSION>
<VERSION etat="MODIFIE">
<LIEN_ART debut="2016-06-05" etat="MODIFIE" fin="2020-01-01" id="LEGIARTI000032656317" num="16-11" origine="LEGI"/>
</VERSION>
<VERSION etat="MODIFIE">
<LIEN_ART debut="2020-01-01" etat="MODIFIE" fin="2023-05-21" id="LEGIARTI000039367698" num="16-11" origine="LEGI"/>
</VERSION>
<VERSION etat="VIGUEUR">
<LIEN_ART debut="2023-05-21" etat="VIGUEUR" fin="2999-01-01" id="LEGIARTI000047569313" num="16-11" origine="LEGI"/>
</VERSION>
</VERSIONS>
<NOTA>
<CONTENU>
<p>Conformément à l'article 36 de l'ordonnance n° 2019-964 du 18 septembre 2019, ces dispositions entrent en vigueur au 1er janvier 2020.</p>
</CONTENU>
</NOTA>
<BLOC_TEXTUEL>
<CONTENU>
<p>L'identification d'une personne par ses empreintes génétiques ne peut être recherchée que :</p>
<p>1° Dans le cadre de mesures d'enquête ou d'instruction diligentées lors d'une procédure judiciaire ;</p>
<p>2° A des fins médicales ou de recherche scientifique ;</p>
<p>3° Aux fins d'établir, lorsqu'elle est inconnue, l'identité de personnes décédées ;</p>
<p>4° Dans les conditions prévues à l'article L. 2381-1 du code de la défense.</p>
<p>En matière civile, cette identification ne peut être recherchée qu'en exécution d'une mesure d'instruction ordonnée par le juge saisi d'une action tendant soit à l'établissement ou la contestation d'un lien de filiation, soit à l'obtention ou la suppression de subsides. Le consentement de l'intéressé doit être préalablement et expressément recueilli. Sauf accord exprès de la personne manifesté de son vivant, aucune identification par empreintes génétiques ne peut être réalisée après sa mort.</p>
<p>Lorsque l'identification est effectuée à des fins médicales ou de recherche scientifique, le consentement exprès de la personne doit être recueilli par écrit préalablement à la réalisation de l'identification, après qu'elle a été dûment informée de sa nature et de sa finalité. Le consentement mentionne la finalité de l'identification. Il est révocable sans forme et à tout moment.</p>
<p>Lorsque la recherche d'identité mentionnée au 3° concerne soit un militaire décédé à l'occasion d'une opération conduite par les forces armées ou les formations rattachées, soit une victime de catastrophe naturelle, soit une personne faisant l'objet de recherches au titre de l'article 26 de la loi n° 95-73 du 21 janvier 1995 d'orientation et de programmation relative à la sécurité et dont la mort est supposée, des prélèvements destinés à recueillir les traces biologiques de cette personne peuvent être réalisés dans des lieux qu'elle est susceptible d'avoir habituellement fréquentés, avec l'accord du responsable des lieux ou, en cas de refus de celui-ci ou d'impossibilité de recueillir cet accord, avec l'autorisation du juge des libertés et de la détention du tribunal judiciaire. Des prélèvements aux mêmes fins sur les ascendants, descendants ou collatéraux supposés de cette personne peuvent être également réalisés. Le consentement exprès de chaque personne concernée est alors recueilli par écrit préalablement à la réalisation du prélèvement, après que celle-ci a été dûment informée de la nature de ce prélèvement, de sa finalité ainsi que du caractère à tout moment révocable de son consentement. Le consentement mentionne la finalité du prélèvement et de l'identification.</p>
<p>Les modalités de mise en œuvre des recherches d'identification mentionnées au 3° du présent article sont précisées par décret en Conseil d'Etat.</p>
</CONTENU>
</BLOC_TEXTUEL>
<LIENS>
<LIEN cidtexte="JORFTEXT000000369046" datesignatexte="1995-01-21" id="LEGIARTI000006527982" naturetexte="LOI" nortexte="INTX9400063L" num="26" numtexte="95-73" sens="source" typelien="CITATION">Loi n° 95-73 du 21 janvier 1995 - art. 26</LIEN>
<LIEN cidtexte="JORFTEXT000000382296" datesignatexte="1997-02-06" id="LEGIARTI000032714078" naturetexte="DECRET" nortexte="JUSC9620898D" num="1" numtexte="97-109" sens="cible" typelien="CITATION">Décret n°97-109 du 6 février 1997 - art. 1 (V)</LIEN>
<LIEN cidtexte="JORFTEXT000000382296" datesignatexte="1997-02-06" id="LEGIARTI000032714165" naturetexte="DECRET" nortexte="JUSC9620898D" num="13" numtexte="97-109" sens="cible" typelien="CITATION">Décret n°97-109 du 6 février 1997 - art. 13 (V)</LIEN>





<LIEN cidtexte="JORFTEXT000025241330" datesignatexte="2012-01-30" id="LEGIARTI000025262933" naturetexte="DECRET" nortexte="IOCD1129420D" num="1" numtexte="2012-125" sens="cible" typelien="CITATION">Décret n°2012-125
 du 30 janvier 2012 - art. 1 (V)</LIEN>
<LIEN cidtexte="JORFTEXT000025241330" datesignatexte="2012-01-30" id="LEGIARTI000025262962" naturetexte="DECRET" nortexte="IOCD1129420D" num="3" numtexte="2012-125" sens="cible" typelien="CITATION">Décret n°2012-125
 du 30 janvier 2012 - art. 3 (V)</LIEN>
<LIEN cidtexte="JORFTEXT000025241330" datesignatexte="2012-01-30" id="LEGIARTI000025262969" naturetexte="DECRET" nortexte="IOCD1129420D" num="4" numtexte="2012-125" sens="cible" typelien="CITATION">Décret n°2012-125
 du 30 janvier 2012 - art. 4 (V)</LIEN>
<LIEN cidtexte="JORFTEXT000025241330" datesignatexte="2012-01-30" id="LEGIARTI000025262953" naturetexte="DECRET" nortexte="IOCD1129420D" num="5" numtexte="2012-125" sens="cible" typelien="CITATION">Décret n°2012-125
 du 30 janvier 2012 - art. 5 (V)</LIEN>

<LIEN cidtexte="JORFTEXT000031560966" datesignatexte="2015-12-02" id="LEGITEXT000031561671" naturetexte="DECRET" nortexte="INTD1514591D" num="" numtexte="2015-1580" sens="cible" typelien="CITATION">DÉCRET n°2015-1580 du 2 décembre 2015 (V)</LIEN>

<LIEN cidtexte="JORFTEXT000032711627" datesignatexte="2016-06-14" id="LEGIARTI000032712139" naturetexte="DECRET" nortexte="JUSD1607271D" num="15" numtexte="2016-796" sens="cible" typelien="CITATION">Décret n°2016-796 du 14 juin 2016 - art. 15</LIEN>
<LIEN cidtexte="JORFTEXT000039110186" datesignatexte="2019-09-18" id="LEGIARTI000039110945" naturetexte="ORDONNANCE" nortexte="JUSB1917648R" num="35" numtexte="2019-964" sens="source" typelien="MODIFICATION">Ordonnance n°2019-964 du 18 septembre 2019 - art. 35 (VD)</LIEN>

<LIEN cidtexte="LEGITEXT000006072665" datesignatexte="2999-01-01" id="LEGIARTI000043890140" naturetexte="CODE" nortexte="" num="L1130-3" numtexte="" sens="cible" typelien="CITATION">Code de la santé publique - art. L1130-3 (V)</LIEN>
<LIEN cidtexte="LEGITEXT000006072665" datesignatexte="2999-01-01" id="LEGIARTI000025139320" naturetexte="CODE" nortexte="" num="L1131-5" numtexte="" sens="cible" typelien="CITATION">Code de la santé publique - art. L1131-5 (V)</LIEN>
<LIEN cidtexte="LEGITEXT000006072665" datesignatexte="2999-01-01" id="LEGIARTI000023908471" naturetexte="CODE" nortexte="" num="L1133-3" numtexte="" sens="cible" typelien="CITATION">Code de la santé publique - art. L1133-3 (V)</LIEN>
<LIEN cidtexte="LEGITEXT000006072665" datesignatexte="2999-01-01" id="LEGIARTI000006695135" naturetexte="CODE" nortexte="" num="L761-24" numtexte="" sens="cible" typelien="CITATION">Code de la santé publique - art. L761-24 (Ab)</LIEN>
<LIEN cidtexte="LEGITEXT000006071154" datesignatexte="2999-01-01" id="LEGIARTI000054478218" naturetexte="CODE" nortexte="" num="706-54" numtexte="" sens="cible" typelien="CITATION">Code de procédure pénale - art. 706-54 (VT)</LIEN>
<LIEN cidtexte="LEGITEXT000006071154" datesignatexte="2999-01-01" id="LEGIARTI000039344897" naturetexte="CODE" nortexte="" num="R93" numtexte="" sens="cible" typelien="CITATION">Code de procédure pénale - art. R93 (VD)</LIEN>
<LIEN cidtexte="LEGITEXT000006070719" datesignatexte="2999-01-01" id="LEGIARTI000023711940" naturetexte="CODE" nortexte="" num="226-27" numtexte="" sens="cible" typelien="CITATION">Code pénal - art. 226-27 (V)</LIEN>
<LIEN cidtexte="LEGITEXT000006070719" datesignatexte="2999-01-01" id="LEGIARTI000042919801" naturetexte="CODE" nortexte="" num="226-28" numtexte="" sens="cible" typelien="CITATION">Code pénal - art. 226-28 (V)</LIEN>
<LIEN cidtexte="LEGITEXT000006071307" datesignatexte="2999-01-01" id="LEGIARTI000032646851" naturetexte="CODE" nortexte="" num="L2381-1" numtexte="" sens="source" typelien="CITATION">Code de la défense. - art. L2381-1</LIEN>
</LIENS>
</ARTICLE>
"#,
    },
    // art. 16-11, version en vigueur depuis 2023-05-21
    Extract {
        member: "legi/global/code_et_TNC_en_vigueur/code_en_vigueur/LEGI/TEXT/00/00/06/07/07/LEGITEXT000006070721/article/LEGI/ARTI/00/00/47/56/93/LEGIARTI000047569313.xml",
        sha256: "cd33c9d2d440b3041ad81a4ca7463f0d0a2082dbd44ba1e2fdc70e52aeb321e0",
        xml: r#"<?xml version="1.0" encoding="UTF-8"?>
<ARTICLE>
<META>
<META_COMMUN>
<ID>LEGIARTI000047569313</ID>
<ANCIEN_ID/>
<ORIGINE>LEGI</ORIGINE>
<URL>article/LEGI/ARTI/00/00/47/56/93/LEGIARTI000047569313.xml</URL>
<NATURE>Article</NATURE>
</META_COMMUN>
<META_SPEC>
<META_ARTICLE>

<NUM>16-11</NUM>

<ETAT>VIGUEUR</ETAT>

<DATE_DEBUT>2023-05-21</DATE_DEBUT>
<DATE_FIN>2999-01-01</DATE_FIN>



<TYPE>AUTONOME</TYPE>
</META_ARTICLE>
</META_SPEC>
</META>
<CONTEXTE>
<TEXTE autorite="" cid="LEGITEXT000006070721" date_publi="2999-01-01" date_signature="2999-01-01" ministere="" nature="CODE" nor="" num="" num_parution_jo="">
<TITRE_TXT c_titre_court="Code civil" debut="1804-03-21" fin="2999-01-01" id_txt="LEGITEXT000006070721">Code civil</TITRE_TXT>
<TM>
<TITRE_TM debut="1804-03-21" fin="2999-01-01" id="LEGISCTA000006089697">Livre Ier : Des personnes</TITRE_TM>
<TM>
<TITRE_TM debut="1994-07-30" fin="2999-01-01" id="LEGISCTA000006117610">Titre Ier : Des droits civils</TITRE_TM>
<TM>
<TITRE_TM debut="2004-08-07" fin="2999-01-01" id="LEGISCTA000006136513">Chapitre III : De l'examen des caractéristiques génétiques d'une personne et de l'identification d'une personne par ses empreintes génétiques</TITRE_TM>
</TM>
</TM>
</TM>
</TEXTE>
</CONTEXTE>

<VERSIONS>
<VERSION etat="MODIFIE">
<LIEN_ART debut="1994-07-30" etat="MODIFIE" fin="2004-08-07" id="LEGIARTI000006419307" num="16-11" origine="LEGI"/>
</VERSION>
<VERSION etat="MODIFIE">
<LIEN_ART debut="2004-08-07" etat="MODIFIE" fin="2005-07-01" id="LEGIARTI000006419308" num="16-11" origine="LEGI"/>
</VERSION>
<VERSION etat="MODIFIE">
<LIEN_ART debut="2007-07-01" etat="MODIFIE" fin="2011-03-16" id="LEGIARTI000006419309" num="16-11" origine="LEGI"/>
</VERSION>
<VERSION etat="MODIFIE">
<LIEN_ART debut="2011-03-16" etat="MODIFIE" fin="2016-06-05" id="LEGIARTI000023711923" num="16-11" origine="LEGI"/>
</VERSION>
<VERSION etat="MODIFIE">
<LIEN_ART debut="2016-06-05" etat="MODIFIE" fin="2020-01-01" id="LEGIARTI000032656317" num="16-11" origine="LEGI"/>
</VERSION>
<VERSION etat="MODIFIE">
<LIEN_ART debut="2020-01-01" etat="MODIFIE" fin="2023-05-21" id="LEGIARTI000039367698" num="16-11" origine="LEGI"/>
</VERSION>
<VERSION etat="VIGUEUR">
<LIEN_ART debut="2023-05-21" etat="VIGUEUR" fin="2999-01-01" id="LEGIARTI000047569313" num="16-11" origine="LEGI"/>
</VERSION>
</VERSIONS>
<NOTA>
<CONTENU>
<p>Conformément à l'article 36 de l'ordonnance n° 2019-964 du 18 septembre 2019, ces dispositions entrent en vigueur au 1er janvier 2020.</p>
</CONTENU>
</NOTA>
<BLOC_TEXTUEL>
<CONTENU>
<p>L'identification d'une personne par ses empreintes génétiques ne peut être recherchée que :</p>
<p>1° Dans le cadre de mesures d'enquête ou d'instruction diligentées lors d'une procédure judiciaire ;</p>
<p>2° A des fins médicales ou de recherche scientifique ;</p>
<p>3° Aux fins d'établir, lorsqu'elle est inconnue, l'identité de personnes décédées ;</p>
<p>4° Dans les conditions prévues à l'article L. 2381-1 du code de la défense ;</p>
<p>5° A des fins de lutte contre le dopage, dans les conditions prévues à l'article L. 232-12-2 du code du sport.</p>
<p>En matière civile, cette identification ne peut être recherchée qu'en exécution d'une mesure d'instruction ordonnée par le juge saisi d'une action tendant soit à l'établissement ou la contestation d'un lien de filiation, soit à l'obtention ou la suppression de subsides. Le consentement de l'intéressé doit être préalablement et expressément recueilli. Sauf accord exprès de la personne manifesté de son vivant, aucune identification par empreintes génétiques ne peut être réalisée après sa mort.</p>
<p>Lorsque l'identification est effectuée à des fins médicales ou de recherche scientifique, le consentement exprès de la personne doit être recueilli par écrit préalablement à la réalisation de l'identification, après qu'elle a été dûment informée de sa nature et de sa finalité. Le consentement mentionne la finalité de l'identification. Il est révocable sans forme et à tout moment.</p>
<p>Lorsque la recherche d'identité mentionnée au 3° concerne soit un militaire décédé à l'occasion d'une opération conduite par les forces armées ou les formations rattachées, soit une victime de catastrophe naturelle, soit une personne faisant l'objet de recherches au titre de l'article 26 de la loi n° 95-73 du 21 janvier 1995 d'orientation et de programmation relative à la sécurité et dont la mort est supposée, des prélèvements destinés à recueillir les traces biologiques de cette personne peuvent être réalisés dans des lieux qu'elle est susceptible d'avoir habituellement fréquentés, avec l'accord du responsable des lieux ou, en cas de refus de celui-ci ou d'impossibilité de recueillir cet accord, avec l'autorisation du juge des libertés et de la détention du tribunal judiciaire. Des prélèvements aux mêmes fins sur les ascendants, descendants ou collatéraux supposés de cette personne peuvent être également réalisés. Le consentement exprès de chaque personne concernée est alors recueilli par écrit préalablement à la réalisation du prélèvement, après que celle-ci a été dûment informée de la nature de ce prélèvement, de sa finalité ainsi que du caractère à tout moment révocable de son consentement. Le consentement mentionne la finalité du prélèvement et de l'identification.</p>
<p>Les modalités de mise en œuvre des recherches d'identification mentionnées au 3° du présent article sont précisées par décret en Conseil d'Etat.</p>
<br/>
</CONTENU>
</BLOC_TEXTUEL>
<LIENS>
<LIEN cidtexte="JORFTEXT000000369046" datesignatexte="1995-01-21" id="LEGIARTI000006527982" naturetexte="LOI" nortexte="INTX9400063L" num="26" numtexte="95-73" sens="source" typelien="CITATION">Loi n° 95-73 du 21 janvier 1995 - art. 26</LIEN>
<LIEN cidtexte="JORFTEXT000000382296" datesignatexte="1997-02-06" id="LEGIARTI000032714078" naturetexte="DECRET" nortexte="JUSC9620898D" num="1" numtexte="97-109" sens="cible" typelien="CITATION">Décret n°97-109 du 6 février 1997 - art. 1 (V)</LIEN>
<LIEN cidtexte="JORFTEXT000000382296" datesignatexte="1997-02-06" id="LEGIARTI000032714165" naturetexte="DECRET" nortexte="JUSC9620898D" num="13" numtexte="97-109" sens="cible" typelien="CITATION">Décret n°97-109 du 6 février 1997 - art. 13 (V)</LIEN>





<LIEN cidtexte="JORFTEXT000025241330" datesignatexte="2012-01-30" id="LEGIARTI000025262933" naturetexte="DECRET" nortexte="IOCD1129420D" num="1" numtexte="2012-125" sens="cible" typelien="CITATION">Décret n°2012-125
 du 30 janvier 2012 - art. 1 (V)</LIEN>
<LIEN cidtexte="JORFTEXT000025241330" datesignatexte="2012-01-30" id="LEGIARTI000025262962" naturetexte="DECRET" nortexte="IOCD1129420D" num="3" numtexte="2012-125" sens="cible" typelien="CITATION">Décret n°2012-125
 du 30 janvier 2012 - art. 3 (V)</LIEN>
<LIEN cidtexte="JORFTEXT000025241330" datesignatexte="2012-01-30" id="LEGIARTI000025262969" naturetexte="DECRET" nortexte="IOCD1129420D" num="4" numtexte="2012-125" sens="cible" typelien="CITATION">Décret n°2012-125
 du 30 janvier 2012 - art. 4 (V)</LIEN>
<LIEN cidtexte="JORFTEXT000025241330" datesignatexte="2012-01-30" id="LEGIARTI000025262953" naturetexte="DECRET" nortexte="IOCD1129420D" num="5" numtexte="2012-125" sens="cible" typelien="CITATION">Décret n°2012-125
 du 30 janvier 2012 - art. 5 (V)</LIEN>

<LIEN cidtexte="JORFTEXT000031560966" datesignatexte="2015-12-02" id="LEGITEXT000031561671" naturetexte="DECRET" nortexte="INTD1514591D" num="" numtexte="2015-1580" sens="cible" typelien="CITATION">DÉCRET n°2015-1580 du 2 décembre 2015 (V)</LIEN>

<LIEN cidtexte="JORFTEXT000032711627" datesignatexte="2016-06-14" id="LEGIARTI000032712139" naturetexte="DECRET" nortexte="JUSD1607271D" num="15" numtexte="2016-796" sens="cible" typelien="CITATION">Décret n°2016-796 du 14 juin 2016 - art. 15</LIEN>

<LIEN cidtexte="JORFTEXT000047561974" datesignatexte="2023-05-19" id="LEGIARTI000047564429" naturetexte="LOI" nortexte="SPOX2233026L" num="5" numtexte="2023-380" sens="cible" typelien="MODIFIE">LOI n°2023-380 du 19 mai 2023 - art. 5 (V)</LIEN>
<LIEN cidtexte="LEGITEXT000006072665" datesignatexte="2999-01-01" id="LEGIARTI000043890140" naturetexte="CODE" nortexte="" num="L1130-3" numtexte="" sens="cible" typelien="CITATION">Code de la santé publique - art. L1130-3 (V)</LIEN>
<LIEN cidtexte="LEGITEXT000006072665" datesignatexte="2999-01-01" id="LEGIARTI000025139320" naturetexte="CODE" nortexte="" num="L1131-5" numtexte="" sens="cible" typelien="CITATION">Code de la santé publique - art. L1131-5 (V)</LIEN>
<LIEN cidtexte="LEGITEXT000006072665" datesignatexte="2999-01-01" id="LEGIARTI000023908471" naturetexte="CODE" nortexte="" num="L1133-3" numtexte="" sens="cible" typelien="CITATION">Code de la santé publique - art. L1133-3 (V)</LIEN>
<LIEN cidtexte="LEGITEXT000006072665" datesignatexte="2999-01-01" id="LEGIARTI000006695135" naturetexte="CODE" nortexte="" num="L761-24" numtexte="" sens="cible" typelien="CITATION">Code de la santé publique - art. L761-24 (Ab)</LIEN>
<LIEN cidtexte="LEGITEXT000006071154" datesignatexte="2999-01-01" id="LEGIARTI000054478218" naturetexte="CODE" nortexte="" num="706-54" numtexte="" sens="cible" typelien="CITATION">Code de procédure pénale - art. 706-54 (VT)</LIEN>
<LIEN cidtexte="LEGITEXT000006071154" datesignatexte="2999-01-01" id="LEGIARTI000039344897" naturetexte="CODE" nortexte="" num="R93" numtexte="" sens="cible" typelien="CITATION">Code de procédure pénale - art. R93 (VD)</LIEN>
<LIEN cidtexte="LEGITEXT000006070719" datesignatexte="2999-01-01" id="LEGIARTI000023711940" naturetexte="CODE" nortexte="" num="226-27" numtexte="" sens="cible" typelien="CITATION">Code pénal - art. 226-27 (V)</LIEN>
<LIEN cidtexte="LEGITEXT000006070719" datesignatexte="2999-01-01" id="LEGIARTI000042919801" naturetexte="CODE" nortexte="" num="226-28" numtexte="" sens="cible" typelien="CITATION">Code pénal - art. 226-28 (V)</LIEN>
<LIEN cidtexte="LEGITEXT000006071307" datesignatexte="2999-01-01" id="LEGIARTI000032646851" naturetexte="CODE" nortexte="" num="L2381-1" numtexte="" sens="source" typelien="CITATION">Code de la défense. - art. L2381-1</LIEN>
</LIENS>
</ARTICLE>
"#,
    },
    // art. 210, version chevauchant la suivante en février 1994
    Extract {
        member: "legi/global/code_et_TNC_en_vigueur/code_en_vigueur/LEGI/TEXT/00/00/06/07/07/LEGITEXT000006070721/article/LEGI/ARTI/00/00/06/42/27/LEGIARTI000006422706.xml",
        sha256: "b8530255ac5632bbc39dd787667505ed42dd496d64b59e0551d6fb6762183df5",
        xml: r#"<?xml version="1.0" encoding="UTF-8"?>
<ARTICLE>
<META>
<META_COMMUN>
<ID>LEGIARTI000006422706</ID>
<ANCIEN_ID>ACAXXXXXXXX5X00210AAXXAA</ANCIEN_ID>
<ORIGINE>LEGI</ORIGINE>
<URL>article/LEGI/ARTI/00/00/06/42/27/LEGIARTI000006422706.xml</URL>
<NATURE>Article</NATURE>
</META_COMMUN>
<META_SPEC>
<META_ARTICLE>

<NUM>210</NUM>

<ETAT>MODIFIE</ETAT>

<DATE_DEBUT>1804-03-21</DATE_DEBUT>
<DATE_FIN>1994-03-01</DATE_FIN>



<TYPE>AUTONOME</TYPE>
</META_ARTICLE>
</META_SPEC>
</META>
<CONTEXTE>
<TEXTE autorite="" cid="LEGITEXT000006070721" date_publi="2999-01-01" date_signature="2999-01-01" ministere="" nature="CODE" nor="" num="" num_parution_jo="">
<TITRE_TXT c_titre_court="Code civil" debut="1804-03-21" fin="2999-01-01" id_txt="LEGITEXT000006070721">Code civil</TITRE_TXT>
<TM>
<TITRE_TM debut="1804-03-21" fin="2999-01-01" id="LEGISCTA000006089697">Livre Ier : Des personnes</TITRE_TM>
<TM>
<TITRE_TM debut="1804-03-21" fin="2999-01-01" id="LEGISCTA000006117710">Titre V : Du mariage</TITRE_TM>
<TM>
<TITRE_TM debut="1804-03-21" fin="2999-01-01" id="LEGISCTA000006136127">Chapitre V : Des obligations qui naissent du mariage</TITRE_TM>
</TM>
</TM>
</TM>
</TEXTE>
</CONTEXTE>

<VERSIONS>
<VERSION etat="MODIFIE">
<LIEN_ART debut="1804-03-21" etat="MODIFIE" fin="1994-03-01" id="LEGIARTI000006422706" num="210" origine="LEGI"/>
</VERSION>
<VERSION etat="VIGUEUR">
<LIEN_ART debut="1994-02-01" etat="VIGUEUR" fin="2999-01-01" id="LEGIARTI000006422707" num="210" origine="LEGI"/>
</VERSION>
</VERSIONS>
<NOTA>
<CONTENU/>
</NOTA>
<BLOC_TEXTUEL>
<CONTENU>
<br/>   Si la personne qui doit fournir des aliments justifie qu'elle ne peut payer la pension alimentaire, le tribunal pourra, en connaissance de cause, ordonner qu'elle recevra dans sa demeure, qu'elle nourrira et entretiendra celui auquel elle devra des aliments.<br/>
<br/>
</CONTENU>
</BLOC_TEXTUEL>
<LIENS>
<LIEN cidtexte="LEGITEXT000006072637" datesignatexte="2999-01-01" id="LEGIARTI000006681210" naturetexte="CODE" nortexte="" num="83" numtexte="" sens="cible" typelien="CITATION">Code de la famille et de l'aide sociale. - art. 83 (Ab)</LIEN>
<LIEN cidtexte="" datesignatexte="" id="" naturetexte="" nortexte="" num="" numtexte="" sens="source" typelien="CODIFICATION">Loi 1803-03-14</LIEN>
<LIEN cidtexte="" datesignatexte="" id="" naturetexte="" nortexte="" num="" numtexte="" sens="source" typelien="CREATION">Loi 1803-03-17 promulguée le 27 mars 1803</LIEN>
</LIENS>
</ARTICLE>
"#,
    },
    // art. 311-21, période inversée (MODIFIE_MORT_NE)
    Extract {
        member: "legi/global/code_et_TNC_en_vigueur/code_en_vigueur/LEGI/TEXT/00/00/06/07/07/LEGITEXT000006070721/article/LEGI/ARTI/00/00/06/42/48/LEGIARTI000006424849.xml",
        sha256: "00a491eabe85c03036a2f00758fb067d364849ff98b311f594ff668682ba1304",
        xml: r#"<?xml version="1.0" encoding="UTF-8"?>
<ARTICLE>
<META>
<META_COMMUN>
<ID>LEGIARTI000006424849</ID>
<ANCIEN_ID>ACAXXXXXXXX5X00311WAXXAA</ANCIEN_ID>
<ORIGINE>LEGI</ORIGINE>
<URL>article/LEGI/ARTI/00/00/06/42/48/LEGIARTI000006424849.xml</URL>
<NATURE>Article</NATURE>
</META_COMMUN>
<META_SPEC>
<META_ARTICLE>

<NUM>311-21</NUM>

<ETAT>MODIFIE_MORT_NE</ETAT>

<DATE_DEBUT>2003-09-01</DATE_DEBUT>
<DATE_FIN>2003-06-19</DATE_FIN>



<TYPE>AUTONOME</TYPE>
</META_ARTICLE>
</META_SPEC>
</META>
<CONTEXTE>
<TEXTE autorite="" cid="LEGITEXT000006070721" date_publi="2999-01-01" date_signature="2999-01-01" ministere="" nature="CODE" nor="" num="" num_parution_jo="">
<TITRE_TXT c_titre_court="Code civil" debut="1804-03-21" fin="2999-01-01" id_txt="LEGITEXT000006070721">Code civil</TITRE_TXT>
<TM>
<TITRE_TM debut="1804-03-21" fin="2999-01-01" id="LEGISCTA000006089697">Livre Ier : Des personnes</TITRE_TM>
<TM>
<TITRE_TM debut="1972-08-01" fin="2999-01-01" id="LEGISCTA000006117789">Titre VII : De la filiation</TITRE_TM>
<TM>
<TITRE_TM debut="1972-08-01" fin="2006-07-01" id="LEGISCTA000006136170">Chapitre Ier : Dispositions communes à la filiation légitime et à la filiation naturelle</TITRE_TM>
<TM>
<TITRE_TM debut="2005-01-01" fin="2006-07-01" id="LEGISCTA000006150049">Section 5 : Des règles de dévolution du nom de famille</TITRE_TM>
</TM>
</TM>
</TM>
</TM>
</TEXTE>
</CONTEXTE>

<VERSIONS>
<VERSION etat="MODIFIE_MORT_NE">
<LIEN_ART debut="2003-09-01" etat="MODIFIE_MORT_NE" fin="2003-06-19" id="LEGIARTI000006424849" num="311-21" origine="LEGI"/>
</VERSION>
<VERSION etat="MODIFIE">
<LIEN_ART debut="2005-01-01" etat="MODIFIE" fin="2006-07-01" id="LEGIARTI000006424850" num="311-21" origine="LEGI"/>
</VERSION>
<VERSION etat="MODIFIE">
<LIEN_ART debut="2006-07-01" etat="MODIFIE" fin="2013-05-19" id="LEGIARTI000006424851" num="311-21" origine="LEGI"/>
</VERSION>
<VERSION etat="MODIFIE">
<LIEN_ART debut="2013-05-19" etat="MODIFIE" fin="2021-08-04" id="LEGIARTI000027432045" num="311-21" origine="LEGI"/>
</VERSION>
<VERSION etat="VIGUEUR">
<LIEN_ART debut="2021-08-04" etat="VIGUEUR" fin="2999-01-01" id="LEGIARTI000043895575" num="311-21" origine="LEGI"/>
</VERSION>
</VERSIONS>
<NOTA>
<CONTENU/>
</NOTA>
<BLOC_TEXTUEL>
<CONTENU>
<br/>   Lorsque la filiation d'un enfant est établie à l'égard de ses deux parents au plus tard le jour de la déclaration de sa naissance ou par la suite mais simultanément, ces derniers choisissent le nom de famille qui lui est dévolu : soit le nom du père, soit le nom de la mère, soit leurs deux noms accolés dans l'ordre choisi par eux dans la limite d'un nom de famille pour chacun d'eux. En l'absence de déclaration conjointe à l'officier de l'état civil mentionnant le choix du nom de l'enfant, celui-ci prend le nom du père.<br/>
<br/>   Le nom dévolu au premier enfant vaut pour les autres enfants communs.<br/>
<br/>   Lorsque les parents ou l'un d'entre eux portent un double nom de famille, ils peuvent, par une déclaration écrite conjointe, ne transmettre qu'un seul nom à leurs enfants.<br/>
</CONTENU>
</BLOC_TEXTUEL>
<LIENS>
<LIEN cidtexte="JORFTEXT000000328986" datesignatexte="1974-05-15" id="LEGIARTI000045282123" naturetexte="DECRET" nortexte="" num="9" numtexte="74-449" sens="cible" typelien="CITATION">Décret n°74-449 du 15 mai 1974 - art. 9 (V)</LIEN>
<LIEN cidtexte="JORFTEXT000000699753" datesignatexte="1993-12-30" id="LEGIARTI000047096142" naturetexte="DECRET" nortexte="JUSX9301612D" num="13" numtexte="93-1362" sens="cible" typelien="CITATION">Décret n°93-1362 du 30 décembre 1993 - art. 13 (V)</LIEN>
<LIEN cidtexte="JORFTEXT000000699753" datesignatexte="1993-12-30" id="LEGIARTI000047096135" naturetexte="DECRET" nortexte="JUSX9301612D" num="42" numtexte="93-1362" sens="cible" typelien="CITATION">Décret n°93-1362 du 30 décembre 1993 - art. 42 (V)</LIEN>
<LIEN cidtexte="JORFTEXT000000593399" datesignatexte="2002-03-04" id="LEGIARTI000006208778" naturetexte="LOI" nortexte="JUSX0104677L" num="4" numtexte="2002-304" sens="source" typelien="CREATION">Loi n°2002-304 du 4 mars 2002 - art. 4 () JORF 5 mars 2002 en vigueur le 1er septembre 2003</LIEN>
<LIEN cidtexte="JORFTEXT000000237596" datesignatexte="2004-10-29" id="LEGIARTI000045282162" naturetexte="DECRET" nortexte="JUSC0420812D" num="1" numtexte="2004-1159" sens="cible" typelien="CITATION">Décret n°2004-1159 du 29 octobre 2004 - art. 1 (V)</LIEN>
<LIEN cidtexte="JORFTEXT000000237596" datesignatexte="2004-10-29" id="LEGIARTI000027469421" naturetexte="DECRET" nortexte="JUSC0420812D" num="14" numtexte="2004-1159" sens="cible" typelien="CITATION">Décret n°2004-1159 du 29 octobre 2004 - art. 14 (M)</LIEN>
<LIEN cidtexte="JORFTEXT000000237596" datesignatexte="2004-10-29" id="LEGIARTI000006286393" naturetexte="DECRET" nortexte="JUSC0420812D" num="4" numtexte="2004-1159" sens="cible" typelien="CITATION">Décret n°2004-1159 du 29 octobre 2004 - art. 4 (M)</LIEN>
<LIEN cidtexte="JORFTEXT000000237596" datesignatexte="2004-10-29" id="LEGIARTI000006286394" naturetexte="DECRET" nortexte="JUSC0420812D" num="5" numtexte="2004-1159" sens="cible" typelien="CITATION">Décret n°2004-1159 du 29 octobre 2004 - art. 5 (V)</LIEN>
<LIEN cidtexte="JORFTEXT000000237596" datesignatexte="2004-10-29" id="LEGIARTI000006286398" naturetexte="DECRET" nortexte="JUSC0420812D" num="9" numtexte="2004-1159" sens="cible" typelien="CITATION">Décret n°2004-1159 du 29 octobre 2004 - art. 9 (V)</LIEN>
<LIEN cidtexte="JORFTEXT000000451869" datesignatexte="2005-07-04" id="LEGIARTI000020123569" naturetexte="ORDONNANCE" nortexte="JUSX0500068R" num="20" numtexte="2005-759" sens="cible" typelien="CITATION">Ordonnance n°2005-759 du 4 juillet 2005 - art. 20 (V)</LIEN>



<LIEN cidtexte="JORFTEXT000027466507" datesignatexte="2013-05-24" id="LEGITEXT000027467269" naturetexte="DECRET" nortexte="JUSC1310218D" num="" numtexte="2013-429" sens="cible" typelien="CITATION">Décret n°2013-429
 du 24 mai 2013 (V)</LIEN>


<LIEN cidtexte="JORFTEXT000039696962" datesignatexte="2019-12-30" id="LEGIARTI000039820262" naturetexte="DECRET" nortexte="JUSC1931943D" num="2" numtexte="2019-1507" sens="cible" typelien="CITATION">Décret n°2019-1507 du 30 décembre 2019 - art. 2</LIEN>
<LIEN cidtexte="LEGITEXT000006070721" datesignatexte="2999-01-01" id="LEGIARTI000006448122" naturetexte="CODE" nortexte="" num="2291" numtexte="" sens="cible" typelien="CITATION">Code civil - art. 2291 (T)</LIEN>
<LIEN cidtexte="LEGITEXT000006070721" datesignatexte="2999-01-01" id="LEGIARTI000006450486" naturetexte="CODE" nortexte="" num="2497" numtexte="" sens="cible" typelien="CITATION">Code civil - art. 2497 (Ab)</LIEN>
<LIEN cidtexte="LEGITEXT000006070721" datesignatexte="2999-01-01" id="LEGIARTI000006424864" naturetexte="CODE" nortexte="" num="311-22" numtexte="" sens="cible" typelien="CITATION">Code civil - art. 311-22 (V)</LIEN>
<LIEN cidtexte="LEGITEXT000006070721" datesignatexte="2999-01-01" id="LEGIARTI000043895566" naturetexte="CODE" nortexte="" num="311-23" numtexte="" sens="cible" typelien="CITATION">Code civil - art. 311-23 (V)</LIEN>
<LIEN cidtexte="LEGITEXT000006070721" datesignatexte="2999-01-01" id="LEGIARTI000006424575" naturetexte="CODE" nortexte="" num="311-24" numtexte="" sens="cible" typelien="CITATION">Code civil - art. 311-24 (V)</LIEN>
<LIEN cidtexte="LEGITEXT000006070721" datesignatexte="2999-01-01" id="LEGIARTI000045289223" naturetexte="CODE" nortexte="" num="311-24-2" numtexte="" sens="cible" typelien="CITATION">Code civil - art. 311-24-2 (VD)</LIEN>
<LIEN cidtexte="LEGITEXT000006070721" datesignatexte="2999-01-01" id="LEGIARTI000037392567" naturetexte="CODE" nortexte="" num="316-5" numtexte="" sens="cible" typelien="CITATION">Code civil - art. 316-5 (V)</LIEN>
<LIEN cidtexte="LEGITEXT000006070721" datesignatexte="2999-01-01" id="LEGIARTI000006425203" naturetexte="CODE" nortexte="" num="331" numtexte="" sens="cible" typelien="CITATION">Code civil - art. 331 (MMN)</LIEN>
<LIEN cidtexte="LEGITEXT000006070721" datesignatexte="2999-01-01" id="LEGIARTI000006425236" naturetexte="CODE" nortexte="" num="332-1" numtexte="" sens="cible" typelien="CITATION">Code civil - art. 332-1 (Ab)</LIEN>
<LIEN cidtexte="LEGITEXT000006070721" datesignatexte="2999-01-01" id="LEGIARTI000006425267" naturetexte="CODE" nortexte="" num="333-5" numtexte="" sens="cible" typelien="CITATION">Code civil - art. 333-5 (Ab)</LIEN>
<LIEN cidtexte="LEGITEXT000006070721" datesignatexte="2999-01-01" id="LEGIARTI000046375877" naturetexte="CODE" nortexte="" num="357" numtexte="" sens="cible" typelien="CITATION">Code civil - art. 357 (VD)</LIEN>
<LIEN cidtexte="LEGITEXT000006070721" datesignatexte="2999-01-01" id="LEGIARTI000006426154" naturetexte="CODE" nortexte="" num="357-1" numtexte="" sens="cible" typelien="CITATION">Code civil - art. 357-1 (V)</LIEN>
<LIEN cidtexte="LEGITEXT000006070721" datesignatexte="2999-01-01" id="LEGIARTI000033460908" naturetexte="CODE" nortexte="" num="55" numtexte="" sens="cible" typelien="CITATION">Code civil - art. 55 (V)</LIEN>
<LIEN cidtexte="LEGITEXT000006070721" datesignatexte="2999-01-01" id="LEGIARTI000054527335" naturetexte="CODE" nortexte="" num="61-3-1" numtexte="" sens="cible" typelien="CITATION">Code civil - art. 61-3-1 (V)</LIEN>
<LIEN cidtexte="" datesignatexte="" id="" naturetexte="" nortexte="" num="" numtexte="" sens="source" typelien="CODIFICATION">Loi 1803-03-14</LIEN>
</LIENS>
</ARTICLE>
"#,
    },
    // art. 214, période vide marquée VIGUEUR
    Extract {
        member: "legi/global/code_et_TNC_en_vigueur/code_en_vigueur/LEGI/TEXT/00/00/06/07/07/LEGITEXT000006070721/article/LEGI/ARTI/00/00/55/02/75/LEGIARTI000055027581.xml",
        sha256: "9521c82d31b9457461c33dfa4c36a2fdc243f226244da908ffbad0f038616819",
        xml: r#"<?xml version="1.0" encoding="UTF-8"?>
<ARTICLE>
<META>
<META_COMMUN>
<ID>LEGIARTI000055027581</ID>
<ANCIEN_ID/>
<ORIGINE>LEGI</ORIGINE>
<URL>article/LEGI/ARTI/00/00/55/02/75/LEGIARTI000055027581.xml</URL>
<NATURE>Article</NATURE>
</META_COMMUN>
<META_SPEC>
<META_ARTICLE>

<NUM>214</NUM>

<ETAT>VIGUEUR</ETAT>

<DATE_DEBUT>1976-07-01</DATE_DEBUT>
<DATE_FIN>1976-07-01</DATE_FIN>



<TYPE>AUTONOME</TYPE>
</META_ARTICLE>
</META_SPEC>
</META>
<CONTEXTE>
<TEXTE autorite="" cid="LEGITEXT000006070721" date_publi="2999-01-01" date_signature="2999-01-01" ministere="" nature="CODE" nor="" num="" num_parution_jo="">
<TITRE_TXT c_titre_court="Code civil" debut="1804-03-21" fin="2999-01-01" id_txt="LEGITEXT000006070721">Code civil</TITRE_TXT>
<TM>
<TITRE_TM debut="1804-03-21" fin="2999-01-01" id="LEGISCTA000006089697">Livre Ier : Des personnes</TITRE_TM>
<TM>
<TITRE_TM debut="1804-03-21" fin="2999-01-01" id="LEGISCTA000006117710">Titre V : Du mariage</TITRE_TM>
<TM>
<TITRE_TM debut="1942-09-22" fin="2999-01-01" id="LEGISCTA000006136137">Chapitre VI : Des devoirs et des droits respectifs des époux</TITRE_TM>
</TM>
</TM>
</TM>
</TEXTE>
</CONTEXTE>

<VERSIONS>
<VERSION etat="MODIFIE">
<LIEN_ART debut="1938-02-20" etat="MODIFIE" fin="1942-11-04" id="LEGIARTI000020558840" num="214" origine="LEGI"/>
</VERSION>
<VERSION etat="MODIFIE">
<LIEN_ART debut="1942-11-04" etat="MODIFIE" fin="1976-07-01" id="LEGIARTI000020558688" num="214" origine="LEGI"/>
</VERSION>
<VERSION etat="VIGUEUR">
<LIEN_ART debut="1976-07-01" etat="VIGUEUR" fin="1976-07-01" id="LEGIARTI000055027581" num="214" origine="LEGI"/>
</VERSION>
<VERSION etat="VIGUEUR">
<LIEN_ART debut="1976-07-01" etat="VIGUEUR" fin="2999-01-01" id="LEGIARTI000006422754" num="214" origine="LEGI"/>
</VERSION>
</VERSIONS>
<NOTA>
<CONTENU/>
</NOTA>
<BLOC_TEXTUEL>
<CONTENU>
<p>Si les conventions matrimoniales ne règlent pasla contribution des époux aux charges du mariage, ils y contribuent à proportion de leurs facultés respectives.</p>
<p>Les charges du mariage incombent au mari, à titre principal. Il est obligé de fournir à la femme tout ce qui est nécessaire pour les besoins de la vie selon ses facultés et son état.</p>
<p>La femme s’acquitte de sa contribution en la prélevant sur les ressources dont elle a l’administration et la jouissance, par ses apports en dot ou en communauté, par son activité au foyer ou sa collaboration à la profession du mari.</p>
<p>Si l’un des époux ne remplit pas ses obligations, il peut y être contraint par l’autre dans les formes prévues au code de procédure civile.</p>
</CONTENU>
</BLOC_TEXTUEL>
<LIENS>
<LIEN cidtexte="JORFTEXT000000315299" datesignatexte="1908-07-14" id="LEGIARTI000006758821" naturetexte="LOI" nortexte="" num="26" numtexte="" sens="cible" typelien="CITATION">Loi du 14 juillet 1908 - art. 26 (VT)</LIEN>
<LIEN cidtexte="JORFTEXT000000522961" datesignatexte="1928-03-30" id="LEGIARTI000006879007" naturetexte="LOI" nortexte="" num="2 septies" numtexte="" sens="cible" typelien="CITATION">Loi n°1928-03-30 du 30 mars 1928 - art. 2 septies (V)</LIEN>
<LIEN cidtexte="JORFTEXT000000686940" datesignatexte="1949-10-05" id="LEGIARTI000006351743" naturetexte="DECRET" nortexte="" num="48" numtexte="49-1416" sens="cible" typelien="CITATION">Décret n°49-1416 du 5 octobre 1949 - art. 48 (Ab)</LIEN>
<LIEN cidtexte="JORFTEXT000000503950" datesignatexte="1965-07-13" id="LEGIARTI000006272712" naturetexte="LOI" nortexte="" num="1" numtexte="65-570" sens="cible" typelien="MODIFIE">Loi n°65-570 du 13 juillet 1965 - art. 1</LIEN>
<LIEN cidtexte="JORFTEXT000000868342" datesignatexte="1968-04-05" id="LEGIARTI000006765598" naturetexte="DECRET" nortexte="" num="29" numtexte="68-382" sens="cible" typelien="CITATION">Décret n°68-382 du 5 avril 1968 - art. 29 (M)</LIEN>
<LIEN cidtexte="JORFTEXT000000876334" datesignatexte="1968-10-11" id="LEGIARTI000006765469" naturetexte="DECRET" nortexte="" num="27" numtexte="68-960" sens="cible" typelien="CITATION">Décret n°68-960 du 11 octobre 1968 - art. 27 (M)</LIEN>
<LIEN cidtexte="JORFTEXT000000322541" datesignatexte="1973-01-02" id="LEGIARTI000006756850" naturetexte="LOI" nortexte="" num="1" numtexte="73-5" sens="cible" typelien="CITATION">Loi n° 73-5 du 2 janvier 1973 - art. 1 (VT)</LIEN>
<LIEN cidtexte="JORFTEXT000000494379" datesignatexte="1973-03-01" id="LEGIARTI000006286363" naturetexte="DECRET" nortexte="" num="8" numtexte="73-216" sens="cible" typelien="CITATION">Décret n°73-216 du 1 mars 1973 - art. 8 (Ab)</LIEN>
<LIEN cidtexte="JORFTEXT000000522103" datesignatexte="1975-07-11" id="LEGIARTI000006755989" naturetexte="LOI" nortexte="" num="15" numtexte="75-618" sens="cible" typelien="CITATION">Loi n°75-618 du 11 juillet 1975 - art. 15 (V)</LIEN>
<LIEN cidtexte="JORFTEXT000000858478" datesignatexte="1975-12-31" id="LEGIARTI000006286349" naturetexte="DECRET" nortexte="" num="22" numtexte="75-1339" sens="cible" typelien="CITATION">Décret n°75-1339 du 31 décembre 1975 - art. 22 (V)</LIEN>
<LIEN cidtexte="JORFTEXT000000501096" datesignatexte="1984-12-22" id="LEGIARTI000006756849" naturetexte="LOI" nortexte="" num="9" numtexte="84-1171" sens="cible" typelien="CITATION">Loi n°84-1171 du 22 décembre 1984 - art. 9 (V)</LIEN>
<LIEN cidtexte="JORFTEXT000000700676" datesignatexte="1985-05-30" id="LEGIARTI000006768582" naturetexte="DECRET" nortexte="" num="18" numtexte="85-560" sens="cible" typelien="CITATION">Décret n°85-560 du 30 mai 1985 - art. 18 (Ab)</LIEN>
<LIEN cidtexte="JORFTEXT000000875188" datesignatexte="1988-12-01" id="LEGIARTI000006756359" naturetexte="LOI" nortexte="ASEX8800072L" num="23" numtexte="88-1088" sens="cible" typelien="CITATION">Loi n°88-1088 du 1 décembre 1988 - art. 23 (Ab)</LIEN>





<LIEN cidtexte="LEGITEXT000006071164" datesignatexte="2999-01-01" id="LEGIARTI000006519939" naturetexte="CODE" nortexte="" num="R*321-14" numtexte="" sens="cible" typelien="CITATION">CODE DE L'ORGANISATION JUDICIAIRE. - art. R*321-14 (M)</LIEN>
<LIEN cidtexte="LEGITEXT000006071164" datesignatexte="2999-01-01" id="LEGIARTI000006519921" naturetexte="CODE" nortexte="" num="R*321-9" numtexte="" sens="cible" typelien="CITATION">CODE DE L'ORGANISATION JUDICIAIRE. - art. R*321-9 (M)</LIEN>
<LIEN cidtexte="LEGITEXT000006070162" datesignatexte="2999-01-01" id="LEGIARTI000006346490" naturetexte="CODE" nortexte="" num="L354-10" numtexte="" sens="cible" typelien="CITATION">CODE DES COMMUNES. - art. L354-10 (Ab)</LIEN>
<LIEN cidtexte="LEGITEXT000006070721" datesignatexte="2999-01-01" id="LEGIARTI000006440765" naturetexte="CODE" nortexte="" num="1537" numtexte="" sens="cible" typelien="CITATION">Code civil - art. 1537 (V)</LIEN>
<LIEN cidtexte="LEGITEXT000006070721" datesignatexte="2999-01-01" id="LEGIARTI000027432006" naturetexte="CODE" nortexte="" num="75" numtexte="" sens="cible" typelien="CITATION">Code civil - art. 75 (V)</LIEN>
<LIEN cidtexte="LEGITEXT000006074069" datesignatexte="2999-01-01" id="LEGIARTI000006797259" naturetexte="CODE" nortexte="" num="L262-35" numtexte="" sens="cible" typelien="CITATION">Code de l'action sociale et des familles - art. L262-35 (V)</LIEN>
<LIEN cidtexte="LEGITEXT000006073189" datesignatexte="2999-01-01" id="LEGIARTI000006743216" naturetexte="CODE" nortexte="" num="L524-4" numtexte="" sens="cible" typelien="CITATION">Code de la sécurité sociale. - art. L524-4 (VT)</LIEN>
<LIEN cidtexte="LEGITEXT000006073189" datesignatexte="2999-01-01" id="LEGIARTI000053281698" naturetexte="CODE" nortexte="" num="L581-3" numtexte="" sens="cible" typelien="CITATION">Code de la sécurité sociale. - art. L581-3 (VD)</LIEN>
<LIEN cidtexte="LEGITEXT000006073189" datesignatexte="2999-01-01" id="LEGIARTI000006750786" naturetexte="CODE" nortexte="" num="R581-9" numtexte="" sens="cible" typelien="CITATION">Code de la sécurité sociale. - art. R581-9 (V)</LIEN>
<LIEN cidtexte="LEGITEXT000006070716" datesignatexte="2999-01-01" id="LEGIARTI000006410721" naturetexte="CODE" nortexte="" num="465-1" numtexte="" sens="cible" typelien="CITATION">Code de procédure civile - art. 465-1 (V)</LIEN>
<LIEN cidtexte="LEGITEXT000006074066" datesignatexte="2999-01-01" id="LEGIARTI000006791933" naturetexte="CODE" nortexte="" num="L30" numtexte="" sens="cible" typelien="CITATION">Code des pensions de retraite des marins frança... - art. L30 (VT)</LIEN>
<LIEN cidtexte="LEGITEXT000006074066" datesignatexte="2999-01-01" id="LEGIARTI000006792006" naturetexte="CODE" nortexte="" num="R21" numtexte="" sens="cible" typelien="CITATION">Code des pensions de retraite des marins français  - art. R21 (V)</LIEN>
<LIEN cidtexte="LEGITEXT000006074068" datesignatexte="2999-01-01" id="LEGIARTI000031710971" naturetexte="CODE" nortexte="" num="L163-1" numtexte="" sens="cible" typelien="CITATION">Code des pensions militaires d'invalidité et de... - art. L163-1 (VD)</LIEN>
<LIEN cidtexte="LEGITEXT000006074068" datesignatexte="2999-01-01" id="LEGIARTI000006794335" naturetexte="CODE" nortexte="" num="L105" numtexte="" sens="cible" typelien="CITATION">Code des pensions militaires d'invalidité et des v - art. L105 (V)</LIEN>
<LIEN cidtexte="LEGITEXT000025024948" datesignatexte="2999-01-01" id="LEGIARTI000041398600" naturetexte="CODE" nortexte="" num="L161-3" numtexte="" sens="cible" typelien="CITATION">Code des procédures civiles d'exécution - art. L161-3 (V)</LIEN>
<LIEN cidtexte="LEGITEXT000025024948" datesignatexte="2999-01-01" id="LEGIARTI000044629357" naturetexte="CODE" nortexte="" num="L213-1" numtexte="" sens="cible" typelien="CITATION">Code des procédures civiles d'exécution - art. L213-1 (V)</LIEN>
<LIEN cidtexte="LEGITEXT000006069577" datesignatexte="2999-01-01" id="LEGIARTI000054373682" naturetexte="CODE" nortexte="" num="156" numtexte="" sens="cible" typelien="CITATION">Code général des impôts, CGI. - art. 156 (V)</LIEN>
<LIEN cidtexte="LEGITEXT000006069577" datesignatexte="2999-01-01" id="LEGIARTI000042907576" naturetexte="CODE" nortexte="" num="80 quater" numtexte="" sens="cible" typelien="CITATION">Code général des impôts, CGI. - art. 80 quater (V)</LIEN>
<LIEN cidtexte="LEGITEXT000006070716" datesignatexte="2999-01-01" id="LEGIARTI000006411844" naturetexte="CODE" nortexte="" num="1069-3" numtexte="" sens="cible" typelien="CITATION">Nouveau code de procédure civile - art. 1069-3 (Ab)</LIEN>
<LIEN cidtexte="LEGITEXT000006070716" datesignatexte="2999-01-01" id="LEGIARTI000006412358" naturetexte="CODE" nortexte="" num="1282" numtexte="" sens="cible" typelien="CITATION">Nouveau code de procédure civile - art. 1282 (Ab)</LIEN>
</LIENS>
</ARTICLE>
"#,
    },
];
