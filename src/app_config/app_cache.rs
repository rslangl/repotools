//! src/app_config/app_cache.rs

use serde::Deserialize;
use std::{fmt, io, path::PathBuf};

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
                write!(f, "{}", e)
            }
            CacheError::Invalid(e) => {
                write!(f, "{}", e)
            }
        }
    }
}

#[derive(Deserialize)]
pub struct AppCache {
    pub cache_dir: PathBuf,
}

pub fn get_cache(file_path: Option<String>) -> Result<AppCache, CacheError> {
    let cache_path = match file_path {
        Some(path) => PathBuf::from(path),
        None => return Err(CacheError::Invalid("Cache path not found".into())),
    };

    Ok(AppCache {
        cache_dir: cache_path,
    })
}
