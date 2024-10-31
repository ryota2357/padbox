use clap::Args;
use colored::Colorize;
use padbox::{eprintln_error, eprintln_tip, Config, Error};

#[derive(Args)]
/// Create a new playground from a recipe
pub struct New {
    recipe: String,
}

impl New {
    pub fn run(&self) -> Result<(), Error> {
        let config = Config::load()?;
        let recipe = match config.recipes()?.get(&self.recipe) {
            Ok(recipe) => recipe.clone(),
            Err(similar) => {
                eprintln_error(format!("recipe {} not found", self.recipe.yellow()));
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
        let project_name = project.name().to_owned();
        let out_path = project.realise(&config.paddir()?)?;
        println!("{} {project_name}", "Created:".blue());
        println!();
        println!("To enter the playground, run:");
        println!(
            "  {} {}",
            "cd".cyan(),
            format!("'{}'", out_path.display()).yellow()
        );
        Ok(())
    }
}
