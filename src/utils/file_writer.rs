//! src/utils/file_writer.rs

use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use serde::Serialize;

#[derive(Debug)]
pub enum FileWriteError {
    Io(std::io::Error),
    Render(tera::Error),
    Invalid(String),
    Write {
        path: PathBuf,
        source: std::io::Error,
    },
}

impl From<std::io::Error> for FileWriteError {
    fn from(e: std::io::Error) -> Self {
        FileWriteError::Io(e)
    }
}

impl From<tera::Error> for FileWriteError {
    fn from(e: tera::Error) -> Self {
        FileWriteError::Render(e)
    }
}

// Rendering templates with `Tera` require a value that implements `serde::Serializer`,
// and adding the `#[serde(untagged)]` directive tells `Serde` and `Tera` to serialize the
// enum as the contained value
#[derive(Serialize)]
#[serde(untagged)]
pub enum Val {
    Str(String),
    Num(i64),
    Bool(bool),
    Seq(Vec<Val>),
    Map(HashMap<String, Val>),
}

fn render(content: String, properties: &HashMap<String, Val>) -> Result<Vec<u8>, FileWriteError> {
    let mut context = tera::Context::new();

    for (key, val) in properties.iter() {
        context.insert(key.as_str(), val);
    }

    let rendered = match tera::Tera::one_off(&content, &context, false) {
        Ok(r) => {
            if r.is_empty() {
                return Err(FileWriteError::Invalid("Empty resource file".into()));
            }
            r
        }
        Err(e) => return Err(FileWriteError::Render(e.into())),
    };

    Ok(rendered.as_bytes().to_vec())
}

pub fn create_files_with_properties(
    root: &Path,
    current: &Path,
    properties: &HashMap<String, Val>,
) -> Result<(), FileWriteError> {
    for entry in fs::read_dir(current).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let relative_path = path.strip_prefix(root).unwrap();

        if path.is_dir() {
            let _ = create_files_with_properties(root, &path, &properties);
            continue;
        }

        let target_root = Path::new("."); // TODO: using current dir for now

        let target = target_root.join(relative_path);

        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent).map_err(|e| FileWriteError::Write {
                path: parent.to_path_buf(),
                source: e,
            })?;
        }

        let content = fs::read_to_string(&path)?;

        let rendered = render(content, properties)?;
        fs::write(target, rendered).map_err(|e| FileWriteError::Write {
            path: path.clone(),
            source: e,
        })?;
    }

    Ok(())
}

// TODO: resources are required to be read as directories, not the config file itself
// which might be what we actually want in the future. Fix this tomfoolery you dimwit
pub fn create_files(root: &Path, current: &Path) -> Result<(), FileWriteError> {
    for entry in fs::read_dir(current).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let relative_path = path.strip_prefix(root).unwrap();

        if path.is_dir() {
            let _ = create_files(root, &path);
            continue;
        }

        let target_root = Path::new("."); // TODO: using current dir for now

        let target = target_root.join(relative_path);

        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent).map_err(|e| FileWriteError::Write {
                path: parent.to_path_buf(),
                source: e,
            })?;
        }

        let content = fs::read_to_string(&path)?;

        fs::write(target, content).map_err(|e| FileWriteError::Write {
            path: path.clone(),
            source: e,
        })?;
    }

    Ok(())
}
