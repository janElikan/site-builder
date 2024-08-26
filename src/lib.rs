use std::fs;
use std::io::BufReader;
use std::path::Path;

use color_eyre::eyre::{OptionExt, Result};
use gray_matter::{engine::YAML, Matter};
use regex::{Captures, Regex};
use serde::{Deserialize, Serialize};
use xml::reader::XmlEvent;

const WIKILINK_REGEX: &str = r"\[\[(.+?)(\|.+?)?\]\]";
const WIKILINK_EMBED_REGEX: &str = r"\!\[\[(.+?)(\|.+?)?\]\]";

const COLOR_TRANSPARENT: &str = "transparent";
const COLOR_TEXT: &str = "var(--color-text)";
const COLOR_GRAY: &str = "var(--color-subtext)";
const COLOR_BACKGROUND: &str = "var(--color-base)";
const COLOR_BROWN: &str = "var(--color-maroon)";

const COLOR_CYAN: &str = "var(--color-sapphire)";
const COLOR_BLUE: &str = "var(--color-blue)";
const COLOR_PURPLE: &str = "var(--color-lavander)";
const COLOR_MAGENTA: &str = "var(--color-mavue)";
const COLOR_PINK: &str = "var(--color-pink)";

const COLOR_GREEN: &str = "var(--color-green)";
const COLOR_TEAL: &str = "var(--color-teal)";
const COLOR_DARK_ORANGE: &str = "var(--color-yellow)";
const COLOR_BRIGHT_ORANGE: &str = "var(--color-peach)";
const COLOR_RED: &str = "var(--color-red)";

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

pub fn read_note<P: AsRef<Path>>(path: P) -> Result<Note> {
    let matter = Matter::<YAML>::new();

    let file = fs::read_to_string(&path)?;
    let file = matter.parse(&file);

    Ok(Note {
        name: path
            .as_ref()
            .iter()
            .last()
            .ok_or_eyre("Encountered a file without a name?")?
            .to_str()
            .expect("The file should still have a name after type conversion")
            .rsplit_once('.')
            .expect("Pretty sure it's a markdown file")
            .0
            .to_string(),
        meta: file
            .data
            .ok_or_eyre("The file has no frontmatter")?
            .deserialize()?,
        body: file.content,
    })
}

pub fn format_links(block: &str) -> String {
    Regex::new(WIKILINK_REGEX)
        .unwrap()
        .replace_all(block, |caps: &Captures| {
            let link = caps.get(1).unwrap().as_str();
            let label = match caps.get(2) {
                Some(label) => &label.as_str()[1..],
                None => link,
            };

            let link = if link.starts_with("https://")
                || link.starts_with("http://")
                || link.starts_with("mailto:")
            {
                link.to_string()
            } else {
                format!("/{link}")
            };

            format!("[{}]({})", label, link)
        })
        .to_string()
}

pub fn embed_svgs(block: &str, asset_directory: &Path) -> String {
    Regex::new(WIKILINK_EMBED_REGEX)
        .unwrap()
        .replace_all(block, |caps: &Captures| {
            let filename = format!("{}.svg", caps.get(1).unwrap().as_str());
            let mut path = asset_directory.to_path_buf();
            path.push(filename);
            let _alt = caps.get(2).map(|label| &label.as_str()[1..]);

            println!("embedding `{}`", &path.to_str().unwrap());

            process_svg(&path)
        })
        .to_string()
}

fn process_svg(path: &Path) -> String {
    let file = fs::File::open(path).expect("can't open image");
    let file = BufReader::new(file);
    let parser = xml::reader::EventReader::new(file);

    let mut output = String::new();
    let mut ignore_current = false;
    parser.into_iter().for_each(|event| match event {
        Ok(XmlEvent::StartElement {
            name, attributes, ..
        }) => {
            let name = name.to_string().replace("{http://www.w3.org/2000/svg}", "");

            if name == "style" {
                ignore_current = true;
            }

            let attributes: Vec<_> = attributes
                .into_iter()
                .map(|attribute| (attribute.name, attribute.value))
                .filter(|(key, _)| &key.to_string() != "filter")
                .map(|(key, val)| {
                    let len = val.len();
                    let is_color = val.starts_with('#') && len == 7;

                    let val = if is_color {
                        match val.as_str() {
                            "#ffffff" => COLOR_BACKGROUND,
                            "#868e96" => COLOR_GRAY,
                            "#1e1e1e" => COLOR_TEXT,
                            "#846358" => COLOR_BROWN,
                            "#0c8599" => COLOR_CYAN,
                            "#1971c2" => COLOR_BLUE,
                            "#6741d9" => COLOR_PURPLE,
                            "#9c36b5" => COLOR_MAGENTA,
                            "#c2255c" => COLOR_PINK,
                            "#2f9e44" => COLOR_GREEN,
                            "#099268" => COLOR_TEAL,
                            "#f08c00" => COLOR_DARK_ORANGE,
                            "#e8590c" => COLOR_BRIGHT_ORANGE,
                            "#e03131" => COLOR_RED,
                            c => {
                                println!("unknown color: {}", c);
                                c
                            }
                        }
                        .to_string()
                    } else {
                        val
                    };

                    (key, val)
                })
                .map(|(key, val)| format!("{}=\"{}\"", key, val))
                .collect();

            output.push('<');
            output.push_str(&name);
            if !attributes.is_empty() {
                output.push(' ');
                output.push_str(&attributes.join(" "));
            }
            output.push('>');
        }
        Ok(XmlEvent::Characters(chars)) => {
            if !ignore_current {
                output.push_str(&chars)
            }
        }
        Ok(XmlEvent::EndElement { name }) => {
            let name = name.to_string().replace("{http://www.w3.org/2000/svg}", "");

            if name == "style" {
                ignore_current = false;
            }

            output.push_str("</");
            output.push_str(&name);
            output.push('>');
        }
        Err(why) => {
            dbg!(why);
        }
        _ => (),
    });

    output
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
        "<NoteMeta name=\"{}\" source=\"{}\" scope=\"{}\" type=\"{}\" created=\"{}Z\" modified=\"{}Z\" />",
        note.name, note_source, meta.scope, note_type, meta.created, meta.modified,
    )
    .to_string()
}

/// Formats metadata for the SSG (astro, in my case)
pub fn get_frontmatter(note: &Note, layout: &str) -> String {
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
        "---\nlayout: {}\ntitle: \"{}\"\nsource: \"{}\"\nscope: \"{}\"\ntype: \"{}\"\ncreated: \"{}Z\"\nmodified: \"{}Z\"\n---\n\n",
        layout, note.name, note_source, meta.scope, note_type, meta.created, meta.modified,
    )
    .to_string()
}
