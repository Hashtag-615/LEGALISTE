//! Construction du corpus d'un texte LEGI à partir de ses documents
//! d'article.
//!
//! Les versions d'une disposition sont prises dans la liste `VERSIONS`,
//! telle que publiée ; chaque document est confronté à sa propre entrée
//! de liste et toutes les listes d'une même disposition doivent
//! coïncider. Toute divergence arrête la construction : rien n'est
//! corrigé ni écarté en silence.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use crate::corpus::legi_xml::{self, VersionLink, XmlError};
use crate::corpus::{Corpus, CorpusError, SourceRecord};
use crate::domain::legi::{self, LegiError, LegiId, LegiIdKind, LegiStatus};
use crate::domain::provision::{Provision, ProvisionError, ProvisionVersion};

/// Corpus d'un texte et compte rendu de sa lecture.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextExtract {
    corpus: Corpus,
    text: LegiId,
    document_count: usize,
    versions_without_document: Vec<LegiId>,
}

impl TextExtract {
    pub fn corpus(&self) -> &Corpus {
        &self.corpus
    }

    pub fn text(&self) -> LegiId {
        self.text
    }

    pub fn document_count(&self) -> usize {
        self.document_count
    }

    /// Versions citées par une liste `VERSIONS` sans document propre
    /// dans le flux lu, par identifiant croissant.
    pub fn versions_without_document(&self) -> &[LegiId] {
        &self.versions_without_document
    }

    /// Recompose un extrait conservé hors de l'archive (base SQLite),
    /// sous les invariants de `build` : chaque version du corpus a son
    /// document ou figure, une seule fois, parmi les versions sans
    /// document.
    pub fn from_parts(
        corpus: Corpus,
        text: LegiId,
        document_count: usize,
        versions_without_document: Vec<LegiId>,
    ) -> Result<Self, ExtractError> {
        if !matches!(text.kind(), LegiIdKind::Text | LegiIdKind::JorfText) {
            return Err(ExtractError::NotAText(text));
        }
        let sorted = versions_without_document.windows(2).all(|w| w[0] < w[1]);
        let known = versions_without_document
            .iter()
            .all(|id| corpus.provision(*id).is_some());
        let counted =
            document_count + versions_without_document.len() == corpus.state().version_count();
        if document_count == 0 || !sorted || !known || !counted {
            return Err(ExtractError::InconsistentParts(text));
        }

        Ok(Self {
            corpus,
            text,
            document_count,
            versions_without_document,
        })
    }
}

/// `stream` : documents d'article concaténés, tels que `tar` les livre.
pub fn build(
    source: SourceRecord,
    text: LegiId,
    stream: &str,
) -> Result<TextExtract, ExtractError> {
    if !matches!(text.kind(), LegiIdKind::Text | LegiIdKind::JorfText) {
        return Err(ExtractError::NotAText(text));
    }

    let documents = legi_xml::split_documents(stream)
        .map_err(|error| ExtractError::Xml { document: 0, error })?;
    if documents.is_empty() {
        return Err(ExtractError::NoDocument(text));
    }

    let mut lists: BTreeMap<Vec<LegiId>, Vec<VersionLink>> = BTreeMap::new();
    let mut read = BTreeSet::new();

    for (rank, raw) in documents.iter().enumerate() {
        let document = legi_xml::parse_article(raw).map_err(|error| ExtractError::Xml {
            document: rank + 1,
            error,
        })?;
        let id: LegiId = document.id.parse()?;

        check_document(&document, id, text)?;
        if !read.insert(id) {
            return Err(ExtractError::DuplicateDocument(id));
        }

        let mut links = document.versions;
        links.sort_by(|a, b| a.id.cmp(&b.id));
        let key = links
            .iter()
            .map(|link| link.id.parse())
            .collect::<Result<Vec<LegiId>, _>>()?;
        match lists.get(&key) {
            Some(known) if *known != links => return Err(ExtractError::DivergentVersionLists(id)),
            Some(_) => {}
            None => {
                lists.insert(key, links);
            }
        }
    }

    let provisions = lists
        .into_values()
        .map(|links| provision(&links))
        .collect::<Result<Vec<_>, _>>()?;
    let corpus = Corpus::new(source, provisions)?;

    let mut versions_without_document: Vec<LegiId> = corpus
        .provisions()
        .iter()
        .flat_map(|provision| provision.versions())
        .map(|version| version.id())
        .filter(|id| !read.contains(id))
        .collect();
    versions_without_document.sort();

    Ok(TextExtract {
        corpus,
        text,
        document_count: documents.len(),
        versions_without_document,
    })
}

/// Contrôles de provenance et de cohérence interne d'un document.
fn check_document(
    document: &legi_xml::ArticleDocument,
    id: LegiId,
    text: LegiId,
) -> Result<(), ExtractError> {
    let found: LegiId = document.text.parse()?;
    if found != text {
        return Err(ExtractError::ForeignText {
            document: id,
            found,
        });
    }
    if !document.url.ends_with(&format!("/{id}.xml")) {
        return Err(ExtractError::UrlMismatch(id));
    }
    if let Some(link) = document.versions.iter().find(|link| link.origin != "LEGI") {
        return Err(ExtractError::UnsupportedOrigin {
            document: id,
            origin: link.origin.clone(),
        });
    }

    let own = document.versions.iter().find(|link| link.id == document.id);
    let own = own.ok_or(ExtractError::NotInOwnVersions(id))?;
    let meta = (
        &document.number,
        &document.status,
        &document.start,
        &document.end,
    );
    if meta != (&own.number, &own.status, &own.start, &own.end) {
        return Err(ExtractError::MetadataDiffersFromList(id));
    }
    Ok(())
}

fn provision(links: &[VersionLink]) -> Result<Provision, ExtractError> {
    let versions = links
        .iter()
        .map(|link| {
            Ok(ProvisionVersion::new(
                link.id.parse()?,
                &link.number,
                LegiStatus::from_legi(&link.status)?,
                legi::validity_period(&link.start, &link.end)?,
            )?)
        })
        .collect::<Result<Vec<_>, ExtractError>>()?;

    Ok(Provision::new(versions)?)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExtractError {
    NotAText(LegiId),
    NoDocument(LegiId),
    /// `document` : rang dans le flux, à partir de 1 ; 0 pour le flux.
    Xml {
        document: usize,
        error: XmlError,
    },
    ForeignText {
        document: LegiId,
        found: LegiId,
    },
    UrlMismatch(LegiId),
    UnsupportedOrigin {
        document: LegiId,
        origin: String,
    },
    DuplicateDocument(LegiId),
    NotInOwnVersions(LegiId),
    MetadataDiffersFromList(LegiId),
    DivergentVersionLists(LegiId),
    InconsistentParts(LegiId),
    Legi(LegiError),
    Provision(ProvisionError),
    Corpus(CorpusError),
}

impl fmt::Display for ExtractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotAText(id) => write!(f, "{id} ne désigne pas un texte"),
            Self::NoDocument(id) => write!(f, "Aucun document d'article lu pour {id}"),
            Self::Xml { document, error } => write!(f, "Document n° {document} : {error}"),
            Self::ForeignText { document, found } => {
                write!(f, "{document} appartient au texte {found}")
            }
            Self::UrlMismatch(id) => write!(f, "{id} : URL sans rapport avec l'identifiant"),
            Self::UnsupportedOrigin { document, origin } => {
                write!(
                    f,
                    "{document} : version d'origine « {origin} » non prise en charge"
                )
            }
            Self::DuplicateDocument(id) => write!(f, "Document {id} lu plusieurs fois"),
            Self::NotInOwnVersions(id) => write!(f, "{id} absent de sa propre liste VERSIONS"),
            Self::MetadataDiffersFromList(id) => {
                write!(f, "{id} : META_ARTICLE diffère de son entrée LIEN_ART")
            }
            Self::InconsistentParts(id) => {
                write!(f, "{id} : extrait recomposé incohérent")
            }
            Self::DivergentVersionLists(id) => {
                write!(
                    f,
                    "{id} : liste VERSIONS divergente pour une même disposition"
                )
            }
            Self::Legi(error) => error.fmt(f),
            Self::Provision(error) => error.fmt(f),
            Self::Corpus(error) => error.fmt(f),
        }
    }
}

impl Error for ExtractError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Xml { error, .. } => Some(error),
            Self::Legi(error) => Some(error),
            Self::Provision(error) => Some(error),
            Self::Corpus(error) => Some(error),
            _ => None,
        }
    }
}

impl From<LegiError> for ExtractError {
    fn from(error: LegiError) -> Self {
        Self::Legi(error)
    }
}

impl From<ProvisionError> for ExtractError {
    fn from(error: ProvisionError) -> Self {
        Self::Provision(error)
    }
}

impl From<CorpusError> for ExtractError {
    fn from(error: CorpusError) -> Self {
        Self::Corpus(error)
    }
}
