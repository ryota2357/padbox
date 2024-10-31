use crate::Error;
use nix::unistd::{Uid, User};
use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

pub(crate) fn user_home() -> Result<PathBuf, Error> {
    match env::var("HOME") {
        Ok(val) => PathBuf::from(val.clone()).canonicalize().map_err(|e| {
            let message = format!("failed to canonicalize $HOME: {}", val);
            Error::IO { message, source: e }
        }),
        Err(_) => match User::from_uid(Uid::current()) {
            Ok(Some(home)) => Ok(home.dir),
            Ok(None) => {
                let message = "failed to get current user's home directory".to_string();
                let source = io::Error::from(io::ErrorKind::NotFound);
                Err(Error::IO { message, source })
            }
            Err(errno) => {
                let message = "failed to get current user's home directory".to_string();
                let source = io::Error::new(io::ErrorKind::Other, errno);
                Err(Error::IO { message, source })
            }
        },
    }
}

pub(crate) fn config_home() -> Result<PathBuf, Error> {
    match env::var("XDG_CONFIG_HOME") {
        Ok(val) => PathBuf::from(val.clone()).canonicalize().map_err(|e| {
            let message = format!("failed to canonicalize $XDG_CONFIG_HOME: {}", val);
            Error::IO { message, source: e }
        }),
        Err(_) => {
            let home = user_home()?;
            home.canonicalize().map_err(|e| {
                let message = format!("failed to canonicalize $HOME: {}", home.display());
                Error::IO { message, source: e }
            })
        }
    }
}

pub(crate) fn padbox_config_dir() -> Result<PathBuf, Error> {
    let config_dir = config_home()?.join("padbox");
    ensure_dir(&config_dir)?;
    Ok(config_dir)
}

pub(crate) fn padbox_config_path() -> Result<PathBuf, Error> {
    let config_path = padbox_config_dir()?.join("config.toml");
    ensure_file(&config_path)?;
    Ok(config_path)
}

pub(crate) fn ensure_file(path: &Path) -> Result<(), Error> {
    if path.exists() && path.is_file() {
        return Ok(());
    }
    fs::write(path, "\n").map_err(|e| {
        let message = format!("failed to create file: {}", path.display());
        Error::IO { message, source: e }
    })
}

pub(crate) fn ensure_dir(path: &Path) -> Result<(), Error> {
    if path.exists() && path.is_dir() {
        return Ok(());
    }
    fs::create_dir_all(path).map_err(|e| {
        let message = format!("failed to create directory: {}", path.display());
        Error::IO { message, source: e }
    })
}
