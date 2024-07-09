use std::fs;

use color_eyre::eyre::Result;

use site_builder::{Note, ZettelType};

fn main() -> Result<()> {
    color_eyre::install()?;

    let workdir = "/nix/persist/active-externalism/data";

    // flat because my vault is flat, at least for now
    let all_notes: Vec<_> = fs::read_dir(workdir)?
        .map(|entry| entry.unwrap().path())
        .filter(|path| !path.is_dir())
        .map(site_builder::read_note)
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

fn concat_source_note(note: &Note, all_notes: &[Note]) -> Note {
    let linked_notes = note
        .body
        .lines()
        .filter(|line| line.starts_with("- "))
        .map(extract_link)
        .map(|link| all_notes.iter().find(|note| note.name == link))
        .map(|note| match note {
            Some(note) => format!(
                "{}\n\n{}\n\n---\n\n",
                site_builder::format_metadata(note),
                note.body
            ),
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

fn extract_link(line: &str) -> &str {
    line.split_once("](").unwrap().1.split_once(')').unwrap().0
}

fn save_to_file(note: Note) {
    let _ = fs::create_dir("./dist");

    // TODO add frontmatter

    fs::write(format!("./dist/{}.md", note.name), note.body).unwrap();
}
