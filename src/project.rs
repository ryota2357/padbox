use crate::{path_util, Error};
use globset::Glob;
use ignore::{Walk, WalkBuilder};
use std::{
    fs,
    path::{Path, PathBuf},
};
use toml::Table as TomlTable;

pub struct Project {
    name: String,
    walk: Option<Walk>,
    source_path: PathBuf,
}

impl Project {
    pub(crate) fn new(
        recipe_name: String,
        source_path: PathBuf,
        config: Option<TomlTable>,
    ) -> Result<Self, Error> {
        Ok(Self {
            name: generate_project_name(&recipe_name),
            walk: build_walk(&source_path, &config)?,
            source_path,
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn realise(self, out_dir: &Path) -> Result<PathBuf, Error> {
        let out_path = out_dir.join(&self.name);
        path_util::ensure_dir(&out_path)?;

        let Some(walk) = self.walk else {
            return Ok(out_path);
        };
        for entry in walk {
            let entry = match entry {
                Ok(entry) => entry,
                Err(err) => {
                    fs::remove_dir_all(out_path)?;
                    todo!("failed to realise project: {}", err);
                }
            };
            let from_path = entry.path();
            let to_path = {
                let entry_path = entry.path();
                match entry_path.strip_prefix(&self.source_path) {
                    Ok(path) => out_path.join(path),
                    Err(_) => {
                        fs::remove_dir_all(out_path)?;
                        panic!(
                            "[BUG] failed to strip prefix:\n  source_path: {:?}\n  entry_path: {:?}",
                            self.source_path, entry_path
                        );
                    }
                }
            };
            // println!("{:?} --> {:?}", from_path, to_path);
            if entry.metadata()?.is_dir() {
                fs::create_dir(&to_path)?;
            } else {
                fs::copy(from_path, &to_path)?;
            }
        }
        Ok(out_path)
    }
}

/// YYYYmmdd-HHMMSS-zzzz-RECIPE-XXXXXX
fn generate_project_name(recipe_name: &str) -> String {
    let now = chrono::Local::now();
    let random = rand::random::<u32>();
    format!(
        "{}-{}-{:06x}",
        now.format("%Y%m%d-%H%M%S-%z"),
        recipe_name,
        random % 0x1000000
    )
}

fn build_walk(source_path: &Path, config: &Option<TomlTable>) -> Result<Option<Walk>, Error> {
    if !source_path.exists() {
        return Ok(None);
    }
    let mut builder = WalkBuilder::new(source_path);
    builder.standard_filters(false);
    builder.follow_links(false);
    if let Some(config) = config {
        let exclude_globs = match config.get("exclude") {
            Some(exclude) => {
                let Some(array) = exclude.as_array() else {
                    return Err(Error::ConfigValue {
                        field: "exclude",
                        expected_ty: "array",
                        path: source_path.join("config.toml"),
                    });
                };
                let mut globs = Vec::with_capacity(array.len());
                for value in array {
                    let Some(pattern) = value.as_str() else {
                        return Err(Error::ConfigValue {
                            field: "exclude",
                            expected_ty: "array<string>",
                            path: source_path.join("config.toml"),
                        });
                    };
                    let pattern = source_path.join(pattern).to_string_lossy().to_string();
                    globs.push(Glob::new(&pattern)?.compile_matcher());
                }
                globs
            }
            None => Vec::new(),
        };
        builder.filter_entry(move |entry| {
            let path = entry.path();
            exclude_globs.iter().all(|matcher| !matcher.is_match(path))
        });
    };
    let mut walk = builder.build();
    walk.next(); // Skip the root directory
    Ok(Some(walk))
}
