pub mod ast;

use ast::RootSchema;

use openapiv3::{
    Operation, ParameterData, ParameterSchemaOrContent, ReferenceOr, SchemaKind, Type,
};

pub fn build(pathname: String, operation: &Operation) -> AstResult {
    let builder = AstBuilder::new(pathname, operation);
    builder.build()
}

pub struct AstResult {
    pub ast: ast::RootSchema,
    pub errors: Vec<String>,
}

struct AstBuilder<'a> {
    pathname: String,
    operation: &'a Operation,
    errors: Vec<String>,
}

impl<'a> AstBuilder<'a> {
    fn new(pathname: String, operation: &'a Operation) -> Self {
        Self {
            pathname,
            operation,
            errors: Vec::new(),
        }
    }

    fn build(mut self) -> AstResult {
        let ope_id = if let Some(id) = &self.operation.operation_id {
            Some(id.clone())
        } else {
            self.errors.push("operation_id is not found".to_string());
            None
        };

        let mut params = vec![];
        for param in &self.operation.parameters {
            let param = match param {
                ReferenceOr::Item(param) => param.parameter_data_ref(),
                ReferenceOr::Reference { .. } => unimplemented!(),
            };

            let param = self.build_param(param);
            params.push(param);
        }

        AstResult {
            ast: RootSchema {
                name: ope_id,
                parameters: params,
            },
            errors: self.errors,
        }
    }

    fn build_param(&mut self, param: &ParameterData) -> ast::Schema {
        let ty = ast::Type::Integer;
        let validates = vec![];

        match &param.format {
            ParameterSchemaOrContent::Schema(schema) => {
                let schema = match schema {
                    ReferenceOr::Item(schema) => schema,
                    ReferenceOr::Reference { .. } => unimplemented!(),
                };

                match &schema.schema_kind {
                    SchemaKind::Type(ty) => match ty {
                        Type::Integer(_) => (),
                        _ => unimplemented!(),
                    },
                    SchemaKind::AllOf { .. } => {
                        self.add_unsupported_error_by_param("AllOf", param);
                    }
                    SchemaKind::OneOf { .. } => {
                        self.add_unsupported_error_by_param("OneOf", param);
                    }
                    SchemaKind::AnyOf { .. } => {
                        self.add_unsupported_error_by_param("AnyOf", param);
                    }
                    SchemaKind::Any(_) => {
                        self.add_unsupported_error_by_param("Any", param);
                    }
                    SchemaKind::Not { .. } => {
                        self.add_unsupported_error_by_param("Not", param);
                    }
                }
            }
            ParameterSchemaOrContent::Content(_) => {
                self.add_unsupported_error("Content");
            }
        }

        ast::Schema {
            required: param.required,
            ty,
            name: param.name.clone(),
            validates,
        }
    }

    fn add_unsupported_error(&mut self, target: &str) {
        self.add_error(&format!("`{target}` is not supported"));
    }

    fn add_unsupported_error_by_param(&mut self, target: &str, param: &ParameterData) {
        self.add_error(&format!("`{target}` is not supported in {}", param.name));
    }

    fn add_error(&mut self, message: &str) {
        if let Some(operation_id) = &self.operation.operation_id {
            self.errors
                .push(format!("{message} in {} {operation_id}", self.pathname));
        } else {
            self.errors.push(format!("{message} in {}", self.pathname));
        }
    }
}
