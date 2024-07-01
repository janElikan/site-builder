use std::fs;
use std::path::Path;

use color_eyre::eyre::{OptionExt, Result};
use gray_matter::{engine::YAML, Matter};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
struct Note {
    name: String,
    meta: NoteMetadata,
    body: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct NoteMetadata {
    source: Option<String>,
    scope: String,
    r#type: ZettelType,
    created: String,  // for now
    modified: String, // for now
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum ZettelType {
    Main,
    Source,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let note = read_note("/nix/persist/active-externalism/data/website.md")?;
    dbg!(note);

    Ok(())
}

fn read_note<P: AsRef<Path>>(path: P) -> Result<Note> {
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
            .to_string(),
        meta: file
            .data
            .ok_or_eyre("The file has no frontmatter")?
            .deserialize()?,
        body: file.content,
    })
}
