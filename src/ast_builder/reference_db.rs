use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use anyhow::Result;

use openapiv3::PathItem;

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
            let file_type = select_file_type(&reference).unwrap();
            let file = std::fs::read_to_string(reference).unwrap();

            let item: PathItem = match file_type {
                SupportFileType::Json => serde_json::from_str(&file).unwrap(),
                SupportFileType::Yaml => serde_yaml::from_str(&file).unwrap(),
            };
            item
        }))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct FileKey(PathBuf);
impl FileKey {
    fn new<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        Ok(Self(std::fs::canonicalize(path)?))
    }
}
impl AsRef<Path> for FileKey {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}
