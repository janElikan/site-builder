use std::fs;

use color_eyre::{eyre::Result, owo_colors::OwoColorize};

use site_builder::{Note, ZettelType};

fn main() -> Result<()> {
    color_eyre::install()?;

    let workdir = "/nix/persist/active-externalism/data";
    let included_scopes = ["public"];

    // flat because my vault is flat, at least for now
    let all_notes: Vec<_> = fs::read_dir(workdir)?
        .map(|entry| entry.unwrap().path())
        .filter(|path| !path.is_dir())
        .map(site_builder::read_note)
        .filter(|result| result.is_ok()) // not all files are notes
        .flatten()
        .filter(|note| included_scopes.contains(&note.meta.scope.as_str()))
        .collect();

    println!("{}", "## Source".bold().yellow());

    let source_notes = all_notes
        .iter()
        .filter(|note| note.meta.r#type == ZettelType::Source)
        .map(|note| {
            let note = concat_source_note(note, &all_notes);
            println!();

            note
        })
        .map(save_to_file)
        .count();

    println!("{}", "## Main".bold().yellow());

    let main_notes = all_notes
        .into_iter()
        .filter(|note| note.meta.r#type == ZettelType::Main)
        .map(|note| {
            println!("- {}", &note.name);

            note
        })
        .map(save_to_file)
        .count();

    println!(
        "\nProcessed {} notes, {} source and {} main",
        source_notes + main_notes,
        source_notes,
        main_notes,
    );

    Ok(())
}

fn concat_source_note(source_note: &Note, all_notes: &[Note]) -> Note {
    println!("- {}", &source_note.name.bold());

    let linked_notes = source_note
        .body
        .lines()
        .filter(|line| line.starts_with("- "))
        .map(extract_link)
        .map(|link| {
            let note = all_notes.iter().find(|note| note.name == link);
            (link, note)
        })
        .map(|(link, note)| match note {
            Some(note) => {
                println!("  - {}", &note.name.green());
                // println!("├└─ `{}`", &note.name);

                format!(
                    "{}\n\n{}\n\n---\n\n",
                    site_builder::format_metadata(note),
                    note.body
                )
            }
            None => {
                println!("  - {} not found", link.red());

                "*not created yet*\n\n---\n\n".to_string()
            }
        });

    Note {
        name: source_note.name.clone(),
        meta: source_note.meta.clone(),
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
