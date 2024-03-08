use regex::Regex;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::ops::Deref;
use std::path::Path;
use std::path::PathBuf;
use thiserror::Error;
use uuid::Uuid;

use crate::document::Document;

#[derive(Error, Debug)]
enum FindError {
    #[error(transparent)]
    RegexError(#[from] regex::Error),
    #[error("File name has no extension")]
    NoFileExtension,
    #[error("Not a valid file name")]
    InvalidFileName,
    #[error("No valid base file")]
    InvalidBaseFile,
    #[error("An OS string is not valid utf-8")]
    OsStringNotUtf8,
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    ReadJsonError(#[from] ReadJsonError),
}

#[derive(Error, Debug)]
pub enum ReadJsonError {
    #[error("Failed to open the file")]
    FileOpenError(#[from] std::io::Error),
    #[error("Failed to parse JSON")]
    JsonParseError(#[from] serde_json::Error),
}

pub fn read_json<P: AsRef<Path>, V: for<'de> serde::Deserialize<'de>>(
    path: P,
) -> Result<V, ReadJsonError> {
    let file = fs::File::open(path)?;
    let json: V = serde_json::from_reader(file)?;
    Ok(json)
}

pub struct JsonStrVisitor {
    pub collected: Vec<String>,
}

impl JsonStrVisitor {
    pub fn new() -> Self {
        Self {
            collected: Vec::new(),
        }
    }
    pub fn collected(self) -> Vec<String> {
        self.collected
    }
    pub fn visit<Pred, Op>(&mut self, value: &Value, predicate: &Pred, op: &Op) -> &Self
    where
        Pred: Fn(&str) -> bool,
        Op: Fn(&str) -> &str,
    {
        match value {
            Value::Object(map) => {
                for (_, v) in map {
                    self.visit(v, predicate, op);
                }
            }
            Value::Array(arr) => {
                for v in arr {
                    self.visit(v, predicate, op);
                }
            }
            Value::String(s) => match predicate(op(s)) {
                true => self.collected.push(s.to_string()),
                false => {}
            },
            _ => {}
        }

        self
    }
}

struct Slug(String);
impl Slug {
    pub fn new(s: &str) -> Self {
        let slug = s
            .to_lowercase()
            .chars()
            .filter(|&c| c.is_alphanumeric() || c == ' ' || c == '.')
            .collect::<String>()
            .replace(" ", "-");

        Self(slug)
    }
}
impl Deref for Slug {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Define an enum to represent a File or a Folder
pub enum FileSystemEntity {
    File,
    Folder,
}

impl FileSystemEntity {
    // Function to classify a Path as either a File or a Folder
    pub fn classify(path: &Path) -> io::Result<Self> {
        let metadata = path.metadata()?;
        Ok(if metadata.is_file() {
            FileSystemEntity::File
        } else if metadata.is_dir() {
            FileSystemEntity::Folder
        } else {
            // You can add more cases here for other types of file system entities if needed
            // For now, we'll just treat unknown entities as files
            FileSystemEntity::File
        })
    }
}

pub struct JsonFileSearcher;

impl<'target> JsonFileSearcher {
    pub fn new() -> Self {
        Self {}
    }
    pub fn search_and_replace_props<R: AsRef<Path>>(
        &self,
        search_directory: R,
        uuid: &Uuid,
        props: &HashMap<String, serde_json::Value>,
    ) -> Result<(), ReadJsonError> {
        for entry in fs::read_dir(search_directory)? {
            let entry = entry?;
            let path = entry.path();
            tracing::info!("searching: {:?}", path.display());

            match FileSystemEntity::classify(&path)? {
                FileSystemEntity::File => {
                    let mut doc: Document = read_json(&path)?;
                    let _ = doc.update_block_props(uuid.clone(), props.clone());

                    let buf = serde_json::to_string(&doc)?;

                    fs::write(&path, buf);
                }
                FileSystemEntity::Folder => self.search_and_replace_props(path, uuid, props)?,
            }
        }

        Ok(())
    }
}
