use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Note {
    pub name: String,
    pub meta: NoteMetadata,
    pub body: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NoteMetadata {
    pub source: Option<String>,
    pub scope: String,
    pub r#type: ZettelType,
    pub created: String,  // for now
    pub modified: String, // for now
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum ZettelType {
    Main,
    Source,
}

/// Outputs a jsx-formatted note
///
/// ## Examples
///
/// ```
/// use site_builder::{Note, NoteMetadata, ZettelType, format_metadata};
///
/// let note = Note {
///    name: String::from("hello-world"),
///    meta: NoteMetadata {
///        source: Some(String::from("https://example.com")),
///        scope: String::from("public"),
///        r#type: ZettelType::Main,
///        created: String::from("2024-07-06T08:08"),
///        modified: String::from("2024-07-08T16:08"),
///    },
///    body: String::from("Hello, world."),
/// };
///
/// assert_eq!(
///     format_metadata(&note),
///     "<NoteMeta name=\"hello-world\" source=\"https://example.com\" scope=\"public\" type=\"main\" created=\"2024-07-06T08:08\" modified=\"2024-07-08T16:08\" />"
/// );
/// ```
pub fn format_metadata(note: &Note) -> String {
    let meta = &note.meta;

    let note_source = match &meta.source {
        Some(url) => url,
        None => "",
    };

    let note_type = match meta.r#type {
        ZettelType::Main => "main",
        ZettelType::Source => "source",
    };

    format!(
        "<NoteMeta name=\"{}\" source=\"{}\" scope=\"{}\" type=\"{}\" created=\"{}\" modified=\"{}\" />",
        note.name, note_source, meta.scope, note_type, meta.created, meta.modified,
    )
    .to_string()
}
