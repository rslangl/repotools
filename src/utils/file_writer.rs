//! src/utils/file_writer.rs

use std::{
    collections::HashMap,
    fmt, fs,
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

impl fmt::Display for FileWriteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileWriteError::Io(e) => {
                write!(f, "{}", e)
            }
            FileWriteError::Render(e) => {
                write!(f, "{}", e)
            }
            FileWriteError::Invalid(e) => {
                write!(f, "{}", e)
            }
            FileWriteError::Write { path, source } => {
                write!(f, "{}:{}", path.display(), source)
            }
        }
    }
}

// Rendering templates with `Tera` require a value that implements `serde::Serializer`,
// and adding the `#[serde(untagged)]` directive tells `Serde` and `Tera` to serialize the
// enum as the contained value
#[derive(Serialize, Debug)]
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
        Err(e) => {
            // TODO: entire error string is not passed upwards
            return Err(FileWriteError::Render(e))
        }
    };

    Ok(rendered.as_bytes().to_vec())
}

fn create_recurse(
    root: &Path,
    current: &Path,
    properties: &Option<HashMap<String, Val>>,
) -> Result<(), FileWriteError> {
    for entry in fs::read_dir(current).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let relative_path = path.strip_prefix(root).unwrap();

        if path.is_dir() {
            let _ = create_recurse(root, &path, &properties);
            continue;
        }

        let target_root = Path::new("."); // TODO: using current dir for now

        let mut target = target_root.join(relative_path);

        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent).map_err(|e| FileWriteError::Write {
                path: parent.to_path_buf(),
                source: e,
            })?;
        }

        let content = fs::read_to_string(&path)?;

        match path.extension() {
            Some(ext) => {
                if ext == r#"j2"# {
                    let rendered = match properties {
                        Some(p) => {
                            let r = render(content, p)?;
                            r
                        }
                        None => {
                            return Err(FileWriteError::Invalid(
                                "Error reading template properties".into(),
                            ));
                        }
                    };
                    target.set_extension("");
                    fs::write(target, rendered).map_err(|e| FileWriteError::Write {
                        path: path.clone(),
                        source: e,
                    })?;
                }
            }
            None => fs::write(target, content).map_err(|e| FileWriteError::Write {
                path: path.clone(),
                source: e,
            })?,
        };

        // let rendered = match properties {
        //     Some(p) => render(content, p)?,
        //     None => {
        //         return Err(FileWriteError::Invalid(
        //             "Error reading template properties".into(),
        //         ));
        //     }
        // };

        //     fs::write(target, rendered).map_err(|e| FileWriteError::Write {
        //         path: path.clone(),
        //         source: e,
        //     })?;
    }

    Ok(())
}

pub fn write(
    path: PathBuf,
    properties: Option<HashMap<String, Val>>,
) -> Result<(), FileWriteError> {
    if !(path.is_dir()) {
        let content = fs::read_to_string(&path)?;

        fs::write(&path, content).map_err(|e| FileWriteError::Write {
            path: path,
            source: e,
        })?;
        return Ok(());
    }
    create_recurse(&path, &path, &properties)?;
    Ok(())
}
