mod ast_builder;
mod codegen;
mod ir_builder;

use std::{fs::File, io::Read, path::Path};

use anyhow::Result;
use openapiv3::{OpenAPI, Operation, PathItem, ReferenceOr};

pub fn generate_dry_validation_from_file<P>(path: P) -> String
where
    P: AsRef<Path>,
{
    let file_type = select_file_type(&path).unwrap();

    let file_content = {
        let mut file = File::open(path).unwrap();
        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();

        buf
    };

    match file_type {
        SupportFileType::Json => generate_dry_validation_from_json(&file_content),
        SupportFileType::Yaml => generate_dry_validation_from_yaml(&file_content),
    }
}

pub fn generate_dry_validation_from_json(text: &str) -> String {
    let openapi: OpenAPI = match serde_json::from_str(text) {
        Ok(openapi) => openapi,
        Err(err) => panic!(
            "Could not deserialize input\nerror line: `{}`\n",
            text.lines().nth(err.line()).unwrap().trim()
        ),
    };
    generate_dry_validation(&openapi)
}

pub fn generate_dry_validation_from_yaml(text: &str) -> String {
    let openapi: OpenAPI = match serde_yaml::from_str(text) {
        Ok(openapi) => openapi,
        Err(err) => panic!(
            "Could not deserialize input\nerror line: `{}`\n",
            text.lines()
                .nth(err.location().unwrap().line())
                .unwrap()
                .trim()
        ),
    };
    generate_dry_validation(&openapi)
}

enum SupportFileType {
    Json,
    Yaml,
}

fn select_file_type<P>(path: &P) -> Result<SupportFileType>
where
    P: AsRef<Path>,
{
    match path.as_ref().extension() {
        Some(extension) => match extension.to_ascii_lowercase().to_str().unwrap() {
            "json" => Ok(SupportFileType::Json),
            "yaml" | "yml" => Ok(SupportFileType::Yaml),
            ext => anyhow::bail!("Unsupported file extension.(ext: {ext})"),
        },
        None => anyhow::bail!("Unknown file extension."),
    }
}

fn generate_dry_validation(openapi: &OpenAPI) -> String {
    let mut code = String::new();

    for (pathname, item) in &openapi.paths.paths {
        let item = match item {
            ReferenceOr::Item(item) => item,
            ReferenceOr::Reference { .. } => unimplemented!(),
        };
        let operations = handling_operation(item);
        for operation in operations {
            let ast_result = ast_builder::build(pathname.clone(), operation);
            let ir_result = ir_builder::build(&ast_result.ast);

            code += &codegen::generate(&ir_result.ir);
        }
    }

    code
}

fn handling_operation(path: &PathItem) -> Vec<&Operation> {
    let mut operations = Vec::new();

    if let Some(ope) = &path.get {
        operations.push(ope);
    }

    operations
}
