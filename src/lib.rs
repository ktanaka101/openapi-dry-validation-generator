mod ast_builder;
mod codegen;
mod ir_builder;

use openapiv3::{OpenAPI, Operation, PathItem, ReferenceOr};

pub fn generate_dry_validation(text: &str) -> String {
    let mut code = String::new();

    let openapi = deserialize(text);
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

fn deserialize(text: &str) -> OpenAPI {
    match serde_json::from_str(text) {
        Ok(openapi) => openapi,
        Err(err) => panic!(
            "Could not deserialize input\nerror line: `{}`\n",
            text.lines().nth(err.line()).unwrap().trim()
        ),
    }
}
