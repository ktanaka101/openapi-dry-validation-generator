pub mod ast;

use ast::RootSchema;

use openapiv3::{
    Operation, Parameter, ParameterData, ParameterSchemaOrContent, ReferenceOr, SchemaKind, Type,
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

        let mut queries = vec![];
        for param in &self.operation.parameters {
            let param = match param {
                ReferenceOr::Item(param) => param,
                ReferenceOr::Reference { .. } => unimplemented!(),
            };

            match param {
                Parameter::Query { parameter_data, .. } => {
                    if let Some(query) = self.build_param(parameter_data) {
                        queries.push(query);
                    }
                }
                _ => unimplemented!(),
            }
        }

        AstResult {
            ast: RootSchema {
                name: ope_id,
                queries,
            },
            errors: self.errors,
        }
    }

    fn build_param(&mut self, param: &ParameterData) -> Option<ast::Schema> {
        let mut validates = vec![];

        let ty = match &param.format {
            ParameterSchemaOrContent::Schema(schema) => {
                let schema = match schema {
                    ReferenceOr::Item(schema) => schema,
                    ReferenceOr::Reference { .. } => unimplemented!(),
                };

                match &schema.schema_kind {
                    SchemaKind::Type(ty) => match ty {
                        Type::Integer(integer) => {
                            if let Some(max) = integer.maximum {
                                let max = if integer.exclusive_maximum {
                                    max - 1
                                } else {
                                    max
                                };
                                validates.push(ast::Validate::Max(max));
                            }
                            if let Some(min) = integer.minimum {
                                let min = if integer.exclusive_minimum {
                                    min + 1
                                } else {
                                    min
                                };
                                validates.push(ast::Validate::Min(min));
                            }

                            ast::Type::Integer
                        }
                        Type::String(string) => {
                            if let Some(max) = string.max_length {
                                validates.push(ast::Validate::MaxLength(max));
                            }
                            if let Some(min) = string.min_length {
                                validates.push(ast::Validate::MinLength(min));
                            }

                            ast::Type::String
                        }
                        _ => unimplemented!(),
                    },
                    SchemaKind::AllOf { .. } => {
                        self.add_unsupported_error_by_param("AllOf", param);
                        return None;
                    }
                    SchemaKind::OneOf { .. } => {
                        self.add_unsupported_error_by_param("OneOf", param);
                        return None;
                    }
                    SchemaKind::AnyOf { .. } => {
                        self.add_unsupported_error_by_param("AnyOf", param);
                        return None;
                    }
                    SchemaKind::Any(_) => {
                        self.add_unsupported_error_by_param("Any", param);
                        return None;
                    }
                    SchemaKind::Not { .. } => {
                        self.add_unsupported_error_by_param("Not", param);
                        return None;
                    }
                }
            }
            ParameterSchemaOrContent::Content(_) => {
                self.add_unsupported_error("Content");
                return None;
            }
        };

        Some(ast::Schema {
            required: param.required,
            ty,
            name: param.name.clone(),
            validates,
        })
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
