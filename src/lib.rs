use openapiv3::{OpenAPI, Operation, ParameterSchemaOrContent, PathItem, SchemaKind, Type};

pub fn generate_dry_schema(text: &str) -> CodeGenResult {
    let openapi = serialize(text);
    let builder = CodeBuilder::new(&openapi);
    builder.build()
}

pub struct CodeGenResult {
    pub code: String,
    pub errors: Vec<String>,
}

struct CodeBuilder<'a> {
    openapi: &'a OpenAPI,
    errors: Vec<String>,
}

impl<'a> CodeBuilder<'a> {
    fn new(openapi: &'a OpenAPI) -> Self {
        Self {
            openapi,
            errors: Vec::new(),
        }
    }

    fn build(mut self) -> CodeGenResult {
        let mut paths = self.openapi.paths.paths.keys().collect::<Vec<_>>();
        paths.sort();

        let mut code = "".to_string();

        for pathname in paths {
            let path = self.openapi.paths.paths.get(pathname).unwrap();
            if let Some(item) = path.as_item() {
                let operations = handling_operation(item);
                for ope in operations {
                    if ope.parameters.is_empty() {
                        continue;
                    }

                    let id = if let Some(id) = &ope.operation_id {
                        id
                    } else {
                        self.errors
                            .push(format!("operation_id is not found in {}", pathname));
                        continue;
                    };

                    code.push_str(&format!("{} = Dry::Schema.Params do\n", id));

                    for param in &ope.parameters {
                        let param = if let Some(param) = param.as_item() {
                            param.parameter_data_ref()
                        } else {
                            continue;
                        };

                        let mut tmp_code = if param.required {
                            format!("  required(:{}).value(", param.name)
                        } else {
                            format!("  optional({}).value(", param.name)
                        };

                        match &param.format {
                            ParameterSchemaOrContent::Schema(schema) => {
                                let schema = if let Some(schema) = schema.as_item() {
                                    schema
                                } else {
                                    continue;
                                };

                                match &schema.schema_kind {
                                    SchemaKind::Type(ty) => match ty {
                                        Type::Integer(_) => {
                                            tmp_code.push_str(":integer)");
                                        }
                                        _ => unimplemented!(),
                                    },
                                    SchemaKind::AllOf { .. } => {
                                        self.errors.push(format!(
                                            "AllOf is not supported in {}",
                                            param.name
                                        ));
                                        continue;
                                    }
                                    SchemaKind::OneOf { .. } => {
                                        self.errors.push(format!(
                                            "OneOf is not supported in {}",
                                            param.name
                                        ));
                                        continue;
                                    }
                                    SchemaKind::AnyOf { .. } => {
                                        self.errors.push(format!(
                                            "AnyOf is not supported in {}",
                                            param.name
                                        ));
                                        continue;
                                    }
                                    SchemaKind::Any(_) => {
                                        self.errors.push(format!(
                                            "Any is not supported in {}",
                                            param.name
                                        ));
                                        continue;
                                    }
                                    SchemaKind::Not { .. } => {
                                        self.errors.push(format!(
                                            "Not is not supported in {}",
                                            param.name
                                        ));
                                        continue;
                                    }
                                }
                            }
                            ParameterSchemaOrContent::Content(_) => {
                                self.errors.push(format!(
                                    "Content is not supported in {} {}",
                                    pathname, id
                                ));
                                continue;
                            }
                        }

                        code.push_str(&format!("{}\n", tmp_code));
                    }

                    code.push_str("end\n");
                }
            } else {
                continue;
            }
        }

        CodeGenResult {
            code,
            errors: self.errors,
        }
    }
}

fn handling_operation(path: &PathItem) -> Vec<&Operation> {
    let mut operations = Vec::new();

    if let Some(ope) = &path.get {
        operations.push(ope);
    }

    operations
}

fn serialize(text: &str) -> OpenAPI {
    serde_json::from_str(text).expect("Could not deserialize input")
}

#[cfg(test)]
mod tests {
    use expect_test::{expect, Expect};

    use crate::generate_dry_schema;

    fn check(actual: &str, expect: Expect) {
        let schema = boilerplate(actual);
        let result = generate_dry_schema(&schema);

        let mut debug_actual = result.code;
        debug_actual.push_str("\n---\n");
        for err in result.errors {
            debug_actual.push_str(&format!("{}\n", err));
        }

        expect.assert_eq(&debug_actual);
    }

    fn boilerplate(input: &str) -> String {
        format!(
            r#"
            {{
              "openapi": "3.0.0",
              "info": {{
                "title": "Testing API overview",
                "version": "1.0.0"
              }},
              "paths": {{
                {}
              }}
            }}
          "#,
            input
        )
    }

    #[test]
    fn it_works() {
        check(
            r#"
            "/test/example": {
              "get": {
                "operationId": "testExample",
                "parameters": [
                    {
                        "in": "query",
                        "name": "user_id",
                        "required": true,
                        "schema": {
                            "type": "integer"
                        }
                    }
                ],
                "responses": {
                  "200": {
                    "description": "OK"
                  }
                }
              }
            }
        "#,
            expect![[r#"
                testExample = Dry::Schema.Params do
                  required(:user_id).value(:integer)
                end

                ---
            "#]],
        );
    }
}
