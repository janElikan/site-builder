use std::{env, fs};

use color_eyre::{
    eyre::{Context, Result},
    owo_colors::OwoColorize,
};

use site_builder::{Note, ZettelType};

fn main() -> Result<()> {
    color_eyre::install()?;

    let vault_path = read_env_var("SITE_VAULT_PATH")?;
    let output_path = read_env_var("SITE_OUTPUT_PATH")?;
    let included_scopes = read_env_var("SITE_INCLUDE_SCOPES")?;
    let included_scopes: Vec<_> = included_scopes
        .split(',')
        .map(|scope| scope.trim())
        .collect();

    let _ = fs::remove_dir_all(&output_path);
    fs::create_dir_all(&output_path)?;

    // flat because my vault is flat, at least for now
    let all_notes: Vec<_> = fs::read_dir(vault_path)?
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
        .map(|note| save_to_file(note, &output_path))
        .count();

    println!("{}", "## Main".bold().yellow());
    let main_notes = all_notes
        .into_iter()
        .filter(|note| note.meta.r#type == ZettelType::Main)
        .map(|note| {
            println!("- {}", &note.name);

            note
        })
        .map(|note| save_to_file(note, &output_path))
        .count();

    println!("\n{}", "## Summary".bold().yellow());
    println!(
        "Processed {} notes, {} source and {} main.\n",
        source_notes + main_notes,
        source_notes,
        main_notes,
    );
    println!("Used notes with these {}:", "scopes".bold());
    included_scopes
        .iter()
        .for_each(|scope| println!("- {}", scope.green()));
    println!(
        "\nSaved them under {}, overwriting the existing files there.",
        &output_path.green()
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
            let link = &link[1..]; // ignore the slash
            let note = all_notes.iter().find(|note| note.name == link);
            (link, note)
        })
        .map(|(link, note)| match note {
            Some(note) => {
                println!("  - {}", &note.name.green());

                format!("{}\n\n{}", site_builder::format_metadata(note), note.body)
            }
            None => {
                println!("  - {} not found", link.red());

                "*not created yet*".to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n\n---\n\n");

    let body = format!(
        "import NoteMeta from \"../components/NoteMeta.astro\"\n\n{}",
        linked_notes
    );

    Note {
        name: source_note.name.clone(),
        meta: source_note.meta.clone(),
        body,
    }
}

fn extract_link(line: &str) -> &str {
    line.split_once("](").unwrap().1.split_once(')').unwrap().0
}

fn save_to_file(note: Note, workdir: &str) {
    let layout = "../layouts/MainLayout.astro";
    let body = site_builder::get_frontmatter(&note, layout) + &note.body;

    fs::write(format!("{}/{}.mdx", workdir, note.name), body).unwrap();
}

fn read_env_var(var_name: &str) -> Result<String> {
    env::var(var_name).wrap_err_with(|| format!("{} not provided, exiting", var_name.red()))
}
