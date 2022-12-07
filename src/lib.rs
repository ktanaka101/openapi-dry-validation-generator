mod ast_builder;
mod codegen;
mod ir_builder;

use openapiv3::{OpenAPI, Operation, PathItem, ReferenceOr};

pub fn generate_dry_schema(text: &str) -> String {
    let mut code = String::new();

    let openapi = serialize(text);
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

fn serialize(text: &str) -> OpenAPI {
    serde_json::from_str(text).expect("Could not deserialize input")
}

#[cfg(test)]
mod tests {
    use expect_test::{expect, Expect};

    use crate::generate_dry_schema;

    fn check(actual: &str, expect: Expect) {
        let schema = boilerplate(actual);
        let debug_actual = generate_dry_schema(&schema);

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
                TestExample = Dry::Schema::Params do
                  required(user_id).value(:integer)
                end
            "#]],
        );
    }

    #[test]
    fn defined_name_is_pascal() {
        check(
            r#"
            "/test/example": {
              "get": {
                "operationId": "testExample",
                "responses": {
                  "200": {
                    "description": "OK"
                  }
                }
              }
            }
        "#,
            expect![[r#"
                TestExample = Dry::Schema::Params do
                end
            "#]],
        );

        check(
            r#"
            "/test/example": {
              "get": {
                "operationId": "test-example",
                "responses": {
                  "200": {
                    "description": "OK"
                  }
                }
              }
            }
        "#,
            expect![[r#"
                TestExample = Dry::Schema::Params do
                end
            "#]],
        );

        check(
            r#"
            "/test/example": {
              "get": {
                "operationId": "test_example",
                "responses": {
                  "200": {
                    "description": "OK"
                  }
                }
              }
            }
        "#,
            expect![[r#"
                TestExample = Dry::Schema::Params do
                end
            "#]],
        );
    }
}
