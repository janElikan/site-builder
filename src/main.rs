use std::fs;

use color_eyre::eyre::Result;
use gray_matter::{engine::YAML, Matter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Note {
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

    let matter = Matter::<YAML>::new();

    let file = fs::read_to_string("/nix/persist/active-externalism/data/website.md")?;
    let file = matter.parse(&file);
    let props: Note = file.data.unwrap().deserialize()?;
    let body = file.content;

    dbg!(&props, body);

    Ok(())
}
