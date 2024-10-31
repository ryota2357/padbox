use crate::{path_util, Error, Recipe, RecipeList};
use std::{fs, path::PathBuf};
use toml::Table as TomlTable;

pub struct Config {
    config_dir: PathBuf,
    config_path: PathBuf,
    config: TomlTable,
}

impl Config {
    pub fn load() -> Result<Self, Error> {
        let config_dir = path_util::padbox_config_dir()?;
        let config_path = path_util::padbox_config_path()?;
        let config = {
            let content = fs::read_to_string(&config_path).map_err(|e| {
                let message = format!("failed to read config file: {}", config_path.display());
                Error::IO { message, source: e }
            })?;
            content.parse().map_err(|e| Error::ConfigParse {
                path: config_path.clone(),
                source: e,
            })?
        };
        Ok(Self {
            config_dir,
            config_path,
            config,
        })
    }

    pub fn paddir(&self) -> Result<PathBuf, Error> {
        let paddir = match self.config.get("paddir") {
            Some(value) => {
                let Some(str) = value.as_str() else {
                    return Err(Error::ConfigValue {
                        field: "paddir",
                        expected_ty: "string",
                        path: self.config_path.clone(),
                    });
                };
                let path = PathBuf::from(str);
                path_util::ensure_dir(&path)?;
                path
            }
            None => {
                let default = path_util::user_home()?.join(".pad");
                path_util::ensure_dir(&default)?;
                default
            }
        };
        Ok(paddir)
    }

    pub fn recipes(&self) -> Result<RecipeList, Error> {
        let recipe_path = match self.config.get("recipe") {
            Some(value) => {
                let Some(str) = value.as_str() else {
                    return Err(Error::ConfigValue {
                        field: "source",
                        expected_ty: "string",
                        path: self.config_path.clone(),
                    });
                };
                let path = PathBuf::from(str);
                path_util::ensure_dir(&path)?;
                path
            }
            None => {
                let default = self.config_dir.join("recipe");
                path_util::ensure_dir(&default)?;
                default
            }
        };
        let mut recipes = Vec::new();
        for entry in fs::read_dir(&recipe_path).map_err(|e| {
            let message = format!("failed to read source directory: {}", recipe_path.display());
            Error::IO { message, source: e }
        })? {
            let entry = entry.map_err(|e| {
                let message = format!(
                    "failed to read source directory entry: {}",
                    recipe_path.display()
                );
                Error::IO { message, source: e }
            })?;
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            let name = entry.file_name().to_string_lossy().to_string();
            recipes.push(Recipe::new(name, path));
        }
        Ok(RecipeList::new(recipes))
    }
}
