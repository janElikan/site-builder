use std::ffi::OsStr;
use std::path::Path;
use std::{fs, path::PathBuf};

use color_eyre::eyre::{OptionExt, Result};
use gray_matter::{engine::YAML, Matter};
use regex::{Captures, Regex};

use site_builder::{format_metadata, Note, NoteMetadata, ZettelType};

fn main() -> Result<()> {
    color_eyre::install()?;

    let workdir = "/nix/persist/active-externalism/data";

    // flat because my vault is flat, at least for now
    let all_notes: Vec<_> = fs::read_dir(workdir)?
        .map(|entry| entry.unwrap().path())
        .filter(|path| !path.is_dir())
        .map(read_note)
        .filter(|result| result.is_ok()) // not all files are notes
        .flatten()
        .collect();

    all_notes
        .iter()
        .filter(|note| note.meta.r#type == ZettelType::Source)
        .map(|note| concat_source_note(note, &all_notes))
        .for_each(save_to_file);

    all_notes
        .into_iter()
        .filter(|note| note.meta.r#type == ZettelType::Main)
        .for_each(save_to_file);

    Ok(())
}

fn read_note<P: AsRef<Path>>(path: P) -> Result<Note> {
    let matter = Matter::<YAML>::new();

    let file = fs::read_to_string(&path)?;
    let file = matter.parse(&file);

    let body = file.content;
    let regex = Regex::new(r"\[\[(.+?)(\|.+?)?\]\]").unwrap();

    let body = regex
        .replace_all(&body, |caps: &Captures| {
            let link = caps.get(1).unwrap().as_str();
            let label = match caps.get(2) {
                Some(label) => &label.as_str()[1..],
                None => link,
            };

            format!("[{}]({})", label, link)
        })
        .to_string();

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
        body,
    })
}

fn concat_source_note(note: &Note, all_notes: &Vec<Note>) -> Note {
    let linked_notes = note
        .body
        .lines()
        .filter(|line| line.starts_with("- "))
        .map(extract_link)
        .map(|link| all_notes.iter().find(|note| note.name == link))
        .map(|note| match note {
            Some(note) => format!("{}\n\n{}\n\n---\n\n", format_metadata(note), note.body),
            None => {
                println!("WARNING: failed to find a note"); // this is very clear, I know :)

                "*not created yet*\n\n---\n\n".to_string()
            }
        });

    Note {
        name: note.name.clone(),
        meta: note.meta.clone(),
        body: linked_notes.collect(),
    }
}

fn save_to_file(note: Note) {
    let _ = fs::create_dir("./dist");

    // TODO add frontmatter

    fs::write(format!("./dist/{}.md", note.name), note.body).unwrap();
}

fn extract_link(line: &str) -> &str {
    line.split_once("](").unwrap().1.split_once(')').unwrap().0
}
