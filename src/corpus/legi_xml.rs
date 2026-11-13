//! Lecteur XML dédié aux documents d'article de LEGI.
//!
//! Il couvre ce que l'archive contient, constaté le 2026-11-13 sur les
//! 6 395 documents d'article du Code civil : UTF-8, une déclaration XML
//! par document, éléments imbriqués, attributs entre guillemets, entités
//! prédéfinies et numériques. Tout autre balisage (DOCTYPE) est refusé
//! plutôt que deviné ; l'imbrication est contrôlée ; un document mal
//! formé est rejeté, jamais réparé. Les valeurs sont restituées brutes,
//! sans espace retiré.

// Taille justifiée : la syntaxe XML et les contrôles propres aux
// articles forment une seule unité de lecture ; la syntaxe deviendra un
// module distinct lorsqu'un second type de document LEGI la réutilisera.

use std::error::Error;
use std::fmt;

const DECLARATION: &str = "<?xml ";

/// Entrée `LIEN_ART` de la liste `VERSIONS`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VersionLink {
    pub id: String,
    pub number: String,
    pub status: String,
    pub start: String,
    pub end: String,
    pub origin: String,
}

/// Champs d'un document d'article. Un élément `ETAT` absent est lu
/// comme un état vide, que les contrôles d'anomalie signalent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArticleDocument {
    pub id: String,
    pub url: String,
    pub text: String,
    pub number: String,
    pub status: String,
    pub start: String,
    pub end: String,
    pub versions: Vec<VersionLink>,
}

/// Découpe la sortie de `tar` en documents, chacun commençant par sa
/// déclaration XML.
pub fn split_documents(stream: &str) -> Result<Vec<&str>, XmlError> {
    if stream.is_empty() {
        return Ok(Vec::new());
    }
    if !stream.starts_with(DECLARATION) {
        return Err(XmlError::MissingDeclaration);
    }

    let starts: Vec<usize> = stream.match_indices(DECLARATION).map(|(i, _)| i).collect();
    let ends = starts.iter().skip(1).copied().chain([stream.len()]);
    Ok(starts
        .iter()
        .zip(ends)
        .map(|(&s, e)| &stream[s..e])
        .collect())
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Field {
    Id,
    Url,
    Number,
    Status,
    Start,
    End,
}

const LEAVES: [(Field, &[&str]); 6] = [
    (Field::Id, &["ARTICLE", "META", "META_COMMUN", "ID"]),
    (Field::Url, &["ARTICLE", "META", "META_COMMUN", "URL"]),
    (
        Field::Number,
        &["ARTICLE", "META", "META_SPEC", "META_ARTICLE", "NUM"],
    ),
    (
        Field::Status,
        &["ARTICLE", "META", "META_SPEC", "META_ARTICLE", "ETAT"],
    ),
    (
        Field::Start,
        &["ARTICLE", "META", "META_SPEC", "META_ARTICLE", "DATE_DEBUT"],
    ),
    (
        Field::End,
        &["ARTICLE", "META", "META_SPEC", "META_ARTICLE", "DATE_FIN"],
    ),
];
const TEXT: &[&str] = &["ARTICLE", "CONTEXTE", "TEXTE"];
const LINK: &[&str] = &["ARTICLE", "VERSIONS", "VERSION", "LIEN_ART"];

#[derive(Default)]
struct Builder {
    values: [Option<String>; 6],
    text: Option<String>,
    versions: Vec<VersionLink>,
    /// Feuille en cours de lecture et texte accumulé.
    open_leaf: Option<(Field, String)>,
}

pub fn parse_article(document: &str) -> Result<ArticleDocument, XmlError> {
    let mut stack: Vec<&str> = Vec::new();
    let mut builder = Builder::default();
    let mut rest = document;
    let mut root_seen = false;

    while let Some(lt) = rest.find('<') {
        builder.text(&rest[..lt], stack.is_empty())?;
        rest = &rest[lt..];

        if rest.starts_with("<?") {
            rest = skip_past(rest, "?>")?;
        } else if rest.starts_with("<!--") {
            rest = skip_past(rest, "-->")?;
        } else if let Some(cdata) = rest.strip_prefix("<![CDATA[") {
            let end = cdata.find("]]>").ok_or(XmlError::Unterminated("CDATA"))?;
            builder.raw_text(&cdata[..end], stack.is_empty())?;
            rest = &cdata[end + 3..];
        } else if rest.starts_with("<!") {
            return Err(XmlError::UnsupportedMarkup);
        } else {
            let end = tag_end(rest)?;
            let inner = &rest[1..end];
            rest = &rest[end + 1..];

            if let Some(name) = inner.strip_prefix('/') {
                let name = name.trim_end();
                match stack.last() {
                    Some(&open) if open == name => builder.close(),
                    open => {
                        return Err(XmlError::MismatchedTag {
                            expected: open.map(|o| o.to_string()),
                            found: name.to_string(),
                        });
                    }
                }
                stack.pop();
                continue;
            }

            let (inner, empty) = match inner.strip_suffix('/') {
                Some(inner) => (inner, true),
                None => (inner, false),
            };
            let split = inner.find(char::is_whitespace).unwrap_or(inner.len());
            let (name, attributes) = inner.split_at(split);
            if name.is_empty() {
                return Err(XmlError::MalformedTag);
            }
            if stack.is_empty() {
                if root_seen || name != "ARTICLE" {
                    return Err(XmlError::UnexpectedRoot(name.to_string()));
                }
                root_seen = true;
            }

            stack.push(name);
            builder.open(&stack, attributes)?;
            if empty {
                builder.close();
                stack.pop();
            }
        }
    }

    builder.text(rest, stack.is_empty())?;
    if let Some(open) = stack.last() {
        return Err(XmlError::Unclosed(open.to_string()));
    }
    if !root_seen {
        return Err(XmlError::UnexpectedRoot(String::new()));
    }
    builder.finish()
}

impl Builder {
    fn open(&mut self, stack: &[&str], attributes: &str) -> Result<(), XmlError> {
        if self.open_leaf.is_some() {
            return Err(XmlError::UnexpectedChild(stack.join("/")));
        }
        if let Some(&(field, _)) = LEAVES.iter().find(|(_, path)| *path == stack) {
            if self.values[field as usize].is_some() {
                return Err(XmlError::DuplicateElement(stack.join("/")));
            }
            self.open_leaf = Some((field, String::new()));
        } else if stack == TEXT {
            if self.text.is_some() {
                return Err(XmlError::DuplicateElement(stack.join("/")));
            }
            self.text = Some(required(&parse_attributes(attributes)?, "TEXTE", "cid")?);
        } else if stack == LINK {
            let attributes = parse_attributes(attributes)?;
            let get = |name| required(&attributes, "LIEN_ART", name);
            self.versions.push(VersionLink {
                id: get("id")?,
                number: get("num")?,
                status: get("etat")?,
                start: get("debut")?,
                end: get("fin")?,
                origin: get("origine")?,
            });
        }
        Ok(())
    }

    /// Une feuille retenue n'a pas d'enfant : l'élément fermé est
    /// nécessairement la feuille ouverte.
    fn close(&mut self) {
        if let Some((field, value)) = self.open_leaf.take() {
            self.values[field as usize] = Some(value);
        }
    }

    /// Texte entre deux balises : décodé s'il appartient à une feuille
    /// retenue, ignoré sinon ; hors de la racine, seuls les blancs sont
    /// admis.
    fn text(&mut self, text: &str, outside_root: bool) -> Result<(), XmlError> {
        if let Some((_, value)) = &mut self.open_leaf {
            value.push_str(&decode(text)?);
        } else if outside_root && !text.trim().is_empty() {
            return Err(XmlError::TextOutsideRoot);
        }
        Ok(())
    }

    fn raw_text(&mut self, text: &str, outside_root: bool) -> Result<(), XmlError> {
        if outside_root {
            return Err(XmlError::TextOutsideRoot);
        }
        if let Some((_, value)) = &mut self.open_leaf {
            value.push_str(text);
        }
        Ok(())
    }

    fn finish(self) -> Result<ArticleDocument, XmlError> {
        let [id, url, number, status, start, end] = self.values;
        let need = |value: Option<String>, field: Field| {
            value.ok_or_else(|| XmlError::MissingElement(LEAVES[field as usize].1.join("/")))
        };

        Ok(ArticleDocument {
            id: need(id, Field::Id)?,
            url: need(url, Field::Url)?,
            text: self
                .text
                .ok_or_else(|| XmlError::MissingElement(TEXT.join("/")))?,
            number: need(number, Field::Number)?,
            status: status.unwrap_or_default(),
            start: need(start, Field::Start)?,
            end: need(end, Field::End)?,
            versions: self.versions,
        })
    }
}

fn skip_past<'a>(text: &'a str, terminator: &'static str) -> Result<&'a str, XmlError> {
    let end = text
        .find(terminator)
        .ok_or(XmlError::Unterminated(terminator))?;
    Ok(&text[end + terminator.len()..])
}

/// Position du `>` fermant la balise, hors des valeurs d'attribut.
fn tag_end(tag: &str) -> Result<usize, XmlError> {
    let mut quote = None;
    for (i, byte) in tag.bytes().enumerate().skip(1) {
        match (quote, byte) {
            (None, b'"' | b'\'') => quote = Some(byte),
            (Some(q), _) if byte == q => quote = None,
            (None, b'>') => return Ok(i),
            (None, b'<') => return Err(XmlError::MalformedTag),
            _ => {}
        }
    }
    Err(XmlError::Unterminated(">"))
}

fn parse_attributes(raw: &str) -> Result<Vec<(&str, String)>, XmlError> {
    let mut attributes: Vec<(&str, String)> = Vec::new();
    let mut rest = raw.trim_start();

    while !rest.is_empty() {
        let malformed = || XmlError::MalformedAttributes(raw.trim().to_string());
        let eq = rest.find('=').ok_or_else(malformed)?;
        let name = rest[..eq].trim_end();
        let after = rest[eq + 1..].trim_start();
        let quote = after.chars().next().filter(|c| *c == '"' || *c == '\'');
        let quote = quote.ok_or_else(malformed)?;
        let close = after[1..].find(quote).ok_or_else(malformed)? + 1;

        if name.is_empty() || attributes.iter().any(|(n, _)| *n == name) {
            return Err(malformed());
        }
        attributes.push((name, decode(&after[1..close])?));
        rest = after[close + 1..].trim_start();
    }
    Ok(attributes)
}

fn required(
    attributes: &[(&str, String)],
    element: &'static str,
    name: &'static str,
) -> Result<String, XmlError> {
    attributes
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, value)| value.clone())
        .ok_or(XmlError::MissingAttribute { element, name })
}

/// Décode les entités prédéfinies et numériques ; toute autre entité
/// est une erreur.
fn decode(raw: &str) -> Result<String, XmlError> {
    let mut decoded = String::with_capacity(raw.len());
    let mut rest = raw;

    while let Some(amp) = rest.find('&') {
        decoded.push_str(&rest[..amp]);
        let semicolon = rest[amp..].find(';').map(|i| amp + i);
        let semicolon =
            semicolon.ok_or_else(|| XmlError::UnknownEntity(rest[amp..].to_string()))?;
        let entity = &rest[amp + 1..semicolon];
        let character = match entity {
            "amp" => Some('&'),
            "lt" => Some('<'),
            "gt" => Some('>'),
            "quot" => Some('"'),
            "apos" => Some('\''),
            _ => numeric(entity),
        };
        decoded.push(character.ok_or_else(|| XmlError::UnknownEntity(format!("&{entity};")))?);
        rest = &rest[semicolon + 1..];
    }
    decoded.push_str(rest);
    Ok(decoded)
}

fn numeric(entity: &str) -> Option<char> {
    let code = if let Some(hex) = entity.strip_prefix("#x") {
        u32::from_str_radix(hex, 16).ok()?
    } else {
        entity.strip_prefix('#')?.parse().ok()?
    };
    char::from_u32(code)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum XmlError {
    MissingDeclaration,
    UnsupportedMarkup,
    Unterminated(&'static str),
    MalformedTag,
    MalformedAttributes(String),
    MismatchedTag {
        expected: Option<String>,
        found: String,
    },
    UnexpectedRoot(String),
    Unclosed(String),
    TextOutsideRoot,
    UnexpectedChild(String),
    DuplicateElement(String),
    MissingElement(String),
    MissingAttribute {
        element: &'static str,
        name: &'static str,
    },
    UnknownEntity(String),
}

impl fmt::Display for XmlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingDeclaration => f.write_str("Déclaration XML absente en tête de flux"),
            Self::UnsupportedMarkup => f.write_str("Balisage non pris en charge (DOCTYPE)"),
            Self::Unterminated(what) => write!(f, "Construction non terminée : « {what} » attendu"),
            Self::MalformedTag => f.write_str("Balise mal formée"),
            Self::MalformedAttributes(raw) => write!(f, "Attributs mal formés : {raw}"),
            Self::MismatchedTag { expected, found } => write!(
                f,
                "Fermeture « {found} » inattendue (ouvert : {})",
                expected.as_deref().unwrap_or("aucun")
            ),
            Self::UnexpectedRoot(name) => write!(f, "Racine inattendue : « {name} »"),
            Self::Unclosed(name) => write!(f, "Élément non fermé : {name}"),
            Self::TextOutsideRoot => f.write_str("Texte hors de l'élément racine"),
            Self::UnexpectedChild(path) => write!(f, "Élément imbriqué inattendu : {path}"),
            Self::DuplicateElement(path) => write!(f, "Élément répété : {path}"),
            Self::MissingElement(path) => write!(f, "Élément absent : {path}"),
            Self::MissingAttribute { element, name } => {
                write!(f, "Attribut « {name} » absent de {element}")
            }
            Self::UnknownEntity(entity) => write!(f, "Entité inconnue : {entity}"),
        }
    }
}

impl Error for XmlError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn article(meta_extra: &str, links: &str) -> String {
        format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<ARTICLE>\n<META>\n<META_COMMUN>\n\
             <ID>LEGIARTI000000000001</ID>\n<URL>article/LEGIARTI000000000001.xml</URL>\n\
             </META_COMMUN>\n<META_SPEC>\n<META_ARTICLE>\n<NUM>1 &amp; 2</NUM>\n{meta_extra}\
             <DATE_DEBUT>2020-01-01</DATE_DEBUT>\n<DATE_FIN>2999-01-01</DATE_FIN>\n\
             </META_ARTICLE>\n</META_SPEC>\n</META>\n<CONTEXTE>\n\
             <TEXTE cid=\"LEGITEXT000000000009\" nature=\"CODE\">\n<TITRE_TXT>T</TITRE_TXT>\n\
             </TEXTE>\n</CONTEXTE>\n<VERSIONS>\n<VERSION etat=\"VIGUEUR\">\n{links}</VERSION>\n\
             </VERSIONS>\n<BLOC_TEXTUEL>\n<CONTENU>a<br/>b&#13;<p align=\"x>y\">c</p>\
             </CONTENU>\n</BLOC_TEXTUEL>\n</ARTICLE>\n"
        )
    }

    const LINK_1: &str = "<LIEN_ART debut=\"2020-01-01\" etat=\"VIGUEUR\" fin=\"2999-01-01\" \
                          id=\"LEGIARTI000000000001\" num=\"1 &amp; 2\" origine=\"LEGI\"/>\n";

    #[test]
    fn extracts_fields_and_decodes_entities() {
        let document = parse_article(&article("<ETAT>VIGUEUR</ETAT>\n", LINK_1)).unwrap();

        assert_eq!(document.id, "LEGIARTI000000000001");
        assert_eq!(document.text, "LEGITEXT000000000009");
        assert_eq!(document.number, "1 & 2");
        assert_eq!(document.status, "VIGUEUR");
        assert_eq!(
            (document.start.as_str(), document.end.as_str()),
            ("2020-01-01", "2999-01-01")
        );
        assert_eq!(document.versions.len(), 1);
        assert_eq!(document.versions[0].number, "1 & 2");
    }

    #[test]
    fn absent_or_empty_status_is_read_as_empty() {
        for etat in ["", "<ETAT/>\n", "<ETAT></ETAT>\n"] {
            assert_eq!(parse_article(&article(etat, LINK_1)).unwrap().status, "");
        }
    }

    #[test]
    fn splits_concatenated_documents() {
        let one = article("<ETAT>VIGUEUR</ETAT>\n", LINK_1);
        let stream = format!("{one}{one}");

        assert_eq!(
            split_documents(&stream).unwrap(),
            [one.as_str(), one.as_str()]
        );
        assert_eq!(split_documents("").unwrap(), Vec::<&str>::new());
        assert_eq!(
            split_documents("<ARTICLE/>"),
            Err(XmlError::MissingDeclaration)
        );
    }

    #[test]
    fn rejects_malformed_documents_instead_of_repairing_them() {
        let good = article("<ETAT>VIGUEUR</ETAT>\n", LINK_1);
        let cases = [
            (
                good.replace("</VERSIONS>", "</VERSION>"),
                "fermeture croisée",
            ),
            (
                good.replace("<URL>", "<URL><X/>"),
                "enfant dans une feuille",
            ),
            (
                good.replace("1 &amp; 2</NUM>", "1 &nbsp; 2</NUM>"),
                "entité inconnue",
            ),
            (
                good.replace("<ID>LEGIARTI000000000001</ID>\n", ""),
                "ID absent",
            ),
            (good.replace(" origine=\"LEGI\"", ""), "attribut absent"),
            (
                good.replace("<ARTICLE>", "<!DOCTYPE x>\n<ARTICLE>"),
                "DOCTYPE",
            ),
            (
                good.replace("<DATE_FIN>", "<DATE_FIN>2999</DATE_FIN>\n<DATE_FIN>"),
                "doublon",
            ),
            (format!("{good}<ARTICLE/>"), "seconde racine"),
        ];

        for (document, case) in cases {
            assert!(parse_article(&document).is_err(), "{case} accepté");
        }
    }
}
