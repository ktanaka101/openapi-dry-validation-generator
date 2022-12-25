use std::{collections::HashMap, path::PathBuf};

use anyhow::Result;

use openapiv3::PathItem;
use url::Url;

use crate::{select_file_type, SupportFileType};

#[derive(Debug)]
pub(super) struct ReferenceDatabase {
    path_item_by_file: HashMap<FileKey, PathItem>,
}
impl ReferenceDatabase {
    pub(super) fn new() -> Self {
        Self {
            path_item_by_file: HashMap::new(),
        }
    }

    pub(super) fn resolve_path_item(&mut self, reference: &str) -> Result<&PathItem> {
        let reference = FileKey::new(reference)?;

        let entry = self.path_item_by_file.entry(reference);
        Ok(entry.or_insert_with_key(|reference| {
            let file_type = reference.file_type().unwrap();
            let file = reference.read_content();

            let item: PathItem = match file_type {
                SupportFileType::Json => serde_json::from_str(&file).unwrap(),
                SupportFileType::Yaml => serde_yaml::from_str(&file).unwrap(),
            };
            item
        }))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum FileKey {
    Local(PathBuf),
    Remote(Url),
}
impl FileKey {
    fn new(path: &str) -> Result<Self> {
        if path.starts_with("https://") || path.starts_with("http://") {
            Ok(Self::Remote(url::Url::parse(path)?))
        } else {
            Ok(Self::Local(std::fs::canonicalize(path)?))
        }
    }

    fn file_type(&self) -> Result<SupportFileType> {
        match self {
            FileKey::Local(path) => select_file_type(path),
            FileKey::Remote(url) => {
                let url = url.to_string();
                if url.ends_with(".json") {
                    Ok(SupportFileType::Json)
                } else if url.ends_with(".yaml") || url.ends_with(".yml") {
                    Ok(SupportFileType::Yaml)
                } else {
                    anyhow::bail!(format!("Unknown file type. url: {url}"))
                }
            }
        }
    }

    fn read_content(&self) -> String {
        match self {
            FileKey::Local(path) => std::fs::read_to_string(path).unwrap(),
            FileKey::Remote(url) => reqwest::blocking::get(url.to_string())
                .unwrap()
                .text()
                .unwrap(),
        }
    }
}
