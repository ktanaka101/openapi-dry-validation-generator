mod ast_builder;
mod codegen;
mod ir_builder;

use openapiv3::{OpenAPI, Operation, PathItem, ReferenceOr};

pub enum FileType {
    Json,
    Yaml,
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
