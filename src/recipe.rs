use crate::{Error, Project};
use std::{fmt, fs, path::PathBuf};
use strsim::jaro_winkler;

#[derive(Debug, Clone)]
pub struct Recipe {
    name: String,
    path: PathBuf,
}

impl Recipe {
    pub fn new(name: String, path: PathBuf) -> Self {
        Self { name, path }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn into_project(self) -> Result<Project, Error> {
        let config = {
            let path = self.path.join("config.toml");
            if path.exists() {
                let content = fs::read_to_string(&path).map_err(|e| {
                    let message = format!("failed to read config file: {}", path.display());
                    Error::IO { message, source: e }
                })?;
                let parsed = content
                    .parse()
                    .map_err(|e| Error::ConfigParse { path, source: e })?;
                Some(parsed)
            } else {
                None
            }
        };
        let source_path = self.path.join("source");
        Project::new(self.name, source_path, config)
    }
}

pub struct RecipeList {
    recipes: Vec<Recipe>,
}

impl RecipeList {
    pub fn new(sources: Vec<Recipe>) -> Self {
        Self { recipes: sources }
    }

    pub fn get(&self, name: &str) -> Result<&Recipe, Vec<&str>> {
        self.recipes
            .iter()
            .find(|recipe| recipe.name() == name)
            .ok_or_else(|| self.find_similar(name))
    }

    fn find_similar(&self, name: &str) -> Vec<&str> {
        let mut simi = self
            .recipes
            .iter()
            .enumerate()
            .map(|(i, recipe)| (jaro_winkler(recipe.name(), name), i))
            .filter(|(similarity, _)| *similarity > 0.7)
            .collect::<Vec<_>>();
        simi.sort_unstable_by(|(a, _), (b, _)| b.partial_cmp(a).unwrap());
        simi.into_iter()
            .take(3)
            .map(|(_, i)| self.recipes[i].name())
            .collect()
    }
}

impl fmt::Debug for RecipeList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.recipes.iter()).finish()
    }
}
