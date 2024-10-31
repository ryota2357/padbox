use colored::Colorize;
use padbox::{eprintln_error, eprintln_tip, Config, Error};
use std::{env, process};

fn main() {
    if env::args().count() < 2 {
        eprintln_error("recipe name is required");
        process::exit(1);
    }
    if env::args().count() > 2 {
        eprintln_error("too many arguments");
        process::exit(1);
    }
    let recipe_name = unsafe { env::args().nth(1).unwrap_unchecked() };
    if let Err(e) = handle(recipe_name) {
        e.eprintln();
    }
}

fn handle(recipe_name: String) -> Result<(), Error> {
    let config = Config::load()?;
    let recipe = match config.recipes()?.get(&recipe_name) {
        Ok(recipe) => recipe.clone(),
        Err(similar) => {
            eprintln_error(format!("recipe {} not found", &recipe_name.yellow()));
            if !similar.is_empty() {
                eprint!("\n  ");
                eprintln_tip(format!(
                    "a similar recipe found: {}",
                    similar
                        .into_iter()
                        .map(|s| format!("'{}'", s.green()))
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
            }
            return Ok(());
        }
    };
    let project = recipe.into_project()?;
    let out_path = project.realise(&config.paddir()?)?;
    println!("{}", out_path.display());
    Ok(())
}
