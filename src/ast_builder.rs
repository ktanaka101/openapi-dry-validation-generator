pub mod ast;

use ast::RootSchema;

use openapiv3::{
    Operation, Parameter, ParameterData, ParameterSchemaOrContent, ReferenceOr, Schema, SchemaKind,
    Type,
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
        let ty = match &param.format {
            ParameterSchemaOrContent::Schema(schema) => {
                let schema = match schema {
                    ReferenceOr::Item(schema) => schema,
                    ReferenceOr::Reference { .. } => unimplemented!(),
                };
                self.build_schema(schema, param)
            }
            ParameterSchemaOrContent::Content(_) => {
                self.add_unsupported_error("Content");
                return None;
            }
        }?;

        Some(ast::Schema {
            required: param.required,
            ty,
            name: param.name.clone(),
        })
    }

    fn build_type(&mut self, ty: &Type, ctx: &ParameterData) -> ast::Type {
        match ty {
            Type::Integer(integer) => {
                let mut validates = vec![];
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

                ast::Type::Integer { validates }
            }
            Type::String(string) => {
                let mut validates = vec![];
                if let Some(max) = string.max_length {
                    validates.push(ast::Validate::MaxLength(max));
                }
                if let Some(min) = string.min_length {
                    validates.push(ast::Validate::MinLength(min));
                }

                ast::Type::String { validates }
            }
            Type::Boolean {} => ast::Type::Boolean,
            Type::Array(array) => {
                let mut validates = vec![];
                if let Some(max) = array.max_items {
                    validates.push(ast::Validate::MaxItems(max));
                }
                if let Some(min) = array.min_items {
                    validates.push(ast::Validate::MinItems(min));
                }

                let item_ty = if let Some(item_schema) = &array.items {
                    let schema = match item_schema {
                        ReferenceOr::Item(schema) => schema,
                        ReferenceOr::Reference { .. } => unimplemented!(),
                    };
                    self.build_schema(schema, ctx)
                } else {
                    None
                };

                ast::Type::Array {
                    validates,
                    item_ty: item_ty.map(|ty| ty.into()),
                }
            }
            Type::Object(object) => {
                let mut properties = vec![];
                for property in object.properties.iter() {
                    let schema = match property.1 {
                        ReferenceOr::Item(item) => item.as_ref(),
                        ReferenceOr::Reference { .. } => unimplemented!(),
                    };
                    let ty = if let Some(ty) = self.build_schema(schema, ctx) {
                        ty
                    } else {
                        continue;
                    };

                    properties.push(ast::Property {
                        required: object.required.contains(property.0),
                        key: property.0.clone(),
                        value: ty,
                    });
                }

                ast::Type::Object {
                    validates: vec![],
                    properties,
                }
            }
            _ => unimplemented!(),
        }
    }

    fn build_schema(&mut self, schema: &Schema, ctx: &ParameterData) -> Option<ast::Type> {
        match &schema.schema_kind {
            SchemaKind::Type(ty) => Some(self.build_type(ty, ctx)),
            SchemaKind::AllOf { .. } => {
                self.add_unsupported_error_by_param("AllOf", ctx);
                None
            }
            SchemaKind::OneOf { .. } => {
                self.add_unsupported_error_by_param("OneOf", ctx);
                None
            }
            SchemaKind::AnyOf { .. } => {
                self.add_unsupported_error_by_param("AnyOf", ctx);
                None
            }
            SchemaKind::Any(_) => {
                self.add_unsupported_error_by_param("Any", ctx);
                None
            }
            SchemaKind::Not { .. } => {
                self.add_unsupported_error_by_param("Not", ctx);
                None
            }
        }
    }

    fn add_unsupported_error(&mut self, target: &str) {
        self.add_error(&format!("`{target}` is not supported"));
    }

    fn add_unsupported_error_by_param(&mut self, target: &str, ctx: &ParameterData) {
        self.add_error(&format!("`{target}` is not supported in {}", ctx.name));
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
