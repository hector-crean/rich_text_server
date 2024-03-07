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
}

pub fn read_json<P: AsRef<Path>>(path: P) -> Value {
    let file = fs::File::open(path).expect("file should open read only");
    let json: serde_json::Value =
        serde_json::from_reader(file).expect("file should be proper JSON");
    json
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

pub enum TargetFile<'target> {
    RelativePath(&'target str),
    AbsolutePath(&'target str),
}

impl<'target> TargetFile<'target> {}

pub struct FileSearcher<'dest, Q: AsRef<Path>> {
    dest_folder_path: &'dest Q,
}

impl<'target, 'dest: 'target, Q: AsRef<Path>> FileSearcher<'target, Q> {
    pub fn new(dest_folder_path: &'dest Q) -> Self {
        Self { dest_folder_path }
    }
    pub fn search_and_copy<R: AsRef<Path>>(
        &self,
        target_file: &TargetFile<'target>,
        search_directory: R,
    ) -> std::io::Result<()> {
        match target_file {
            TargetFile::RelativePath(relative_path) => {
                for entry in fs::read_dir(search_directory)? {
                    let entry = entry?;
                    let path = entry.path();
                    tracing::info!("searching: {:?}", path.display());

                    match FileSystemEntity::classify(&path)? {
                        FileSystemEntity::File => {
                            let canon_path = path.canonicalize()?;

                            if canon_path.ends_with(relative_path) {
                                let slug = Slug::new(relative_path);
                                let dest_path = self.dest_folder_path.as_ref().join(&*slug);

                                // Perform the copying or other operations here.
                                match fs::copy(path, dest_path) {
                                    Ok(_) => {
                                        tracing::info!("✔️, {:?}", canon_path);
                                    }
                                    Err(err) => {
                                        tracing::info!("❌, {:?}", canon_path);
                                    }
                                }
                            } else {
                            }
                        }
                        FileSystemEntity::Folder => self.search_and_copy(target_file, path)?,
                    }
                }
            }
            TargetFile::AbsolutePath(absolute_path) => {
                let slug = Slug::new(&absolute_path);

                let dest_path = self.dest_folder_path.as_ref().join(&*slug);
                fs::copy(absolute_path, dest_path)?;
            }
        }
        Ok(())
    }
}
