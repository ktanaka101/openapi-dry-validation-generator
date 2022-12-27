use std::{collections::HashMap, path::PathBuf};

use anyhow::Result;

use openapiv3::{OpenAPI, Parameter, PathItem};
use url::Url;

use crate::{select_file_type, SupportFileType};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum FileReference {
    Local,
    LocalFile(FileKey),
}
impl FileReference {
    fn local_file(reference: &str) -> Result<Self> {
        Ok(Self::LocalFile(FileKey::new(reference)?))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FileReferenceWithPath {
    reference: FileReference,
    path: String,
}
impl FileReferenceWithPath {
    fn new(reference: &str) -> Result<Self> {
        let paths = reference.split('#');
        let paths = paths.collect::<Vec<_>>();
        if paths.len() != 2 {
            anyhow::bail!("Failed to parse reference.(reference: {reference})");
        }
        let file_path = paths[0];
        let path = paths[1];

        Ok(FileReferenceWithPath {
            reference: if file_path.is_empty() {
                FileReference::Local
            } else {
                FileReference::local_file(file_path)?
            },
            path: path.to_string(),
        })
    }
}

#[derive(Debug)]
pub(super) struct ReferenceDatabase<'a> {
    local: &'a OpenAPI,
    path_item_by_file: HashMap<FileKey, PathItem>,
    parameter_by_file: HashMap<FileKey, Parameter>,
}
impl<'a> ReferenceDatabase<'a> {
    pub(super) fn new(local: &'a OpenAPI) -> Self {
        Self {
            local,
            path_item_by_file: HashMap::new(),
            parameter_by_file: HashMap::new(),
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

    pub(super) fn resolve_parameter(&mut self, reference: &str) -> Result<&Parameter> {
        let reference_with_path = FileReferenceWithPath::new(reference)?;

        match reference_with_path.reference {
            FileReference::Local => {
                let paths = reference_with_path
                    .path
                    .split('/')
                    // #/components/parameters/SomeParameter
                    //  ^skip first slash
                    .skip(1)
                    .collect::<Vec<_>>();
                dbg!(&paths);
                if paths.len() != 3 {
                    anyhow::bail!("Invalid path.");
                }
                if paths[0] != "components" {
                    anyhow::bail!("Invalid path.");
                }
                if paths[1] != "parameters" {
                    anyhow::bail!("Invalid path.");
                }

                let parameter_name = paths[2];
                let parameter = &self.local.components.as_ref().unwrap().parameters[parameter_name];
                match parameter {
                    openapiv3::ReferenceOr::Reference { reference } => {
                        // ToDo: Stop generating the same parameters recursively.
                        self.resolve_parameter(reference)
                    }
                    openapiv3::ReferenceOr::Item(item) => Ok(item),
                }
            }
            FileReference::LocalFile(file_key) => {
                let entry = self.parameter_by_file.entry(file_key);
                Ok(entry.or_insert_with_key(|reference| {
                    let file_type = reference.file_type().unwrap();
                    let file = reference.read_content();

                    let item: Parameter = match file_type {
                        SupportFileType::Json => serde_json::from_str(&file).unwrap(),
                        SupportFileType::Yaml => serde_yaml::from_str(&file).unwrap(),
                    };
                    item
                }))
            }
        }
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
