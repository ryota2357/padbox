use crate::eprintln_error;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{message}")]
    IO {
        message: String,
        #[source]
        source: std::io::Error,
    },

    #[error("failed to parse config file: {path}")]
    ConfigParse {
        path: std::path::PathBuf,
        #[source]
        source: toml::de::Error,
    },

    #[error("'{field}' in config.toml is not a {expected_ty}: {path}")]
    ConfigValue {
        field: &'static str,
        expected_ty: &'static str,
        path: std::path::PathBuf,
    },

    #[error("{0}")]
    __IOError(#[from] std::io::Error),

    #[error("{0}")]
    __GlobSetError(#[from] globset::Error),

    #[error("{0}")]
    __IgnoreError(#[from] ignore::Error),
}

impl Error {
    pub fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        <Self as std::error::Error>::source(self)
    }

    pub fn eprintln(&self) {
        let message = self.to_string();
        eprintln_error(&message);
        if let Some(source) = self.source() {
            let source = source.to_string();
            if source != message {
                eprintln!("  caused by: {}", source);
            }
        }
    }
}
