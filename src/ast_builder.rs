pub mod ast;
mod reference_db;

use ast::RootSchema;
use reference_db::ReferenceDatabase;

use openapiv3::{
    OpenAPI, Operation, Parameter, ParameterData, ParameterSchemaOrContent, PathItem, Paths,
    ReferenceOr, Schema, SchemaKind, Type,
};

pub fn build(openapi: &OpenAPI) -> AstResult {
    let builder = AstBuilder::new(openapi);
    builder.build()
}

pub struct AstResult {
    pub ast: ast::RootSchema,
    pub errors: Vec<String>,
}

struct AstBuilder<'a> {
    openapi: &'a OpenAPI,
    errors: Vec<String>,
    db: ReferenceDatabase<'a>,
}

impl<'a> AstBuilder<'a> {
    fn new(openapi: &'a OpenAPI) -> Self {
        Self {
            openapi,
            errors: Vec::new(),
            db: ReferenceDatabase::new(openapi),
        }
    }

    fn handling_operation(path: &PathItem) -> Vec<&Operation> {
        let mut operations = Vec::new();

        if let Some(ope) = &path.get {
            operations.push(ope);
        }
        if let Some(ope) = &path.post {
            operations.push(ope);
        }
        if let Some(ope) = &path.patch {
            operations.push(ope);
        }
        if let Some(ope) = &path.put {
            operations.push(ope);
        }
        if let Some(ope) = &path.delete {
            operations.push(ope);
        }

        operations
    }

    fn build(mut self) -> AstResult {
        let path_items = self.build_paths(&self.openapi.paths);

        AstResult {
            ast: RootSchema { path_items },
            errors: self.errors,
        }
    }

    fn build_paths(&mut self, paths: &Paths) -> Vec<ast::PathItem> {
        let mut path_items = vec![];

        for (path_name, item) in paths.iter() {
            let item = match item {
                ReferenceOr::Item(item) => item.clone(),
                ReferenceOr::Reference { reference } => {
                    if let Ok(item) = self.db.resolve_path_item(reference) {
                        item.clone()
                    } else {
                        self.add_error(format!(
                            "Failed to resolve reference. reference: {reference}"
                        ));
                        continue;
                    }
                }
            };
            let operations = Self::handling_operation(&item);

            path_items.push(ast::PathItem {
                url: path_name.clone(),
                operations: operations
                    .iter()
                    .map(|ope| self.build_operation(ope))
                    .collect::<Vec<_>>(),
            });
        }

        path_items
    }

    fn build_operation(&mut self, operation: &Operation) -> ast::Operation {
        let ope_id = if let Some(id) = &operation.operation_id {
            Some(id.clone())
        } else {
            self.errors.push("operation_id is not found".to_string());
            None
        };

        let mut queries = vec![];
        for param in &operation.parameters {
            let param = match param {
                ReferenceOr::Item(param) => param.clone(),
                ReferenceOr::Reference { reference } => {
                    self.db.resolve_parameter(reference).unwrap().clone()
                }
            };

            match param {
                Parameter::Query { parameter_data, .. } => {
                    if let Some(query) = self.build_param(&parameter_data) {
                        queries.push(query);
                    }
                }
                _ => unimplemented!(),
            }
        }

        ast::Operation {
            id: ope_id,
            queries,
        }
    }

    fn build_param(&mut self, param: &ParameterData) -> Option<ast::Schema> {
        let ty = match &param.format {
            ParameterSchemaOrContent::Schema(schema) => {
                let schema = match schema {
                    ReferenceOr::Item(schema) => schema.clone(),
                    ReferenceOr::Reference { reference } => {
                        self.db.resolve_schema(reference).unwrap().clone()
                    }
                };
                self.build_schema(&schema, param)
            }
            ParameterSchemaOrContent::Content(_) => {
                self.add_unsupported_error_by_param("Content", param);
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
            Type::Number(_) => ast::Type::Number,
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
                        ReferenceOr::Item(schema) => *schema.clone(),
                        ReferenceOr::Reference { reference } => {
                            self.db.resolve_schema(reference).unwrap().clone()
                        }
                    };
                    self.build_schema(&schema, ctx)
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
                        ReferenceOr::Item(item) => *item.clone(),
                        ReferenceOr::Reference { reference } => {
                            self.db.resolve_schema(reference).unwrap().clone()
                        }
                    };
                    let ty = if let Some(ty) = self.build_schema(&schema, ctx) {
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

    fn add_unsupported_error_by_param(&mut self, target: &str, param: &ParameterData) {
        self.add_error(format!("`{target}` is not supported in {}", param.name));
    }

    fn add_error(&mut self, message: String) {
        self.errors.push(message);
    }
}
