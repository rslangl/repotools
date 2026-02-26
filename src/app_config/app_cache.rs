//! src/app_config/app_cache.rs

use serde::Deserialize;
use std::{fmt, fs, io, path::PathBuf};

#[derive(Debug)]
pub enum CacheError {
    Io(io::Error),
    Invalid(String),
}

impl From<io::Error> for CacheError {
    fn from(e: io::Error) -> Self {
        CacheError::Io(e)
    }
}

impl From<String> for CacheError {
    fn from(e: String) -> Self {
        CacheError::Invalid(e)
    }
}

impl fmt::Display for CacheError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CacheError::Io(e) => {
                write!(f, "IO error: {}", e)
            }
            CacheError::Invalid(e) => {
                write!(f, "Invalid input: {}", e)
            }
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct AppCache {
    pub cache_dir: PathBuf,
}

pub fn get_cache(file_path: Option<String>) -> Result<AppCache, CacheError> {
    let cache_path = match file_path {
        Some(path) => PathBuf::from(path),
        None => {
            let xdg = xdg::BaseDirectories::with_prefix("repotools");

            let p = xdg
                .get_data_home()
                .ok_or(CacheError::Invalid(String::from("")))?;

            fs::create_dir_all(&p).map_err(CacheError::Io)?;

            p
        }
    };

    Ok(AppCache {
        cache_dir: cache_path,
    })
}
