mod ast_builder;
mod codegen;
mod ir_builder;

use openapiv3::{OpenAPI, Operation, PathItem, ReferenceOr};

pub fn generate_dry_schema(text: &str) -> String {
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

#[cfg(test)]
mod tests {
    use expect_test::{expect, Expect};

    use crate::generate_dry_schema;

    fn check_parameters(actual: &str, expect: Expect) {
        let actual = boilerplate(&format!(
            r#"
                "/test/example": {{
                    "get": {{
                        "operationId": "testExample",
                        "parameters": {actual},
                        "responses": {{
                            "200": {{
                                "description": "OK"
                            }}
                        }}
                    }}
                }}
            "#
        ));
        let debug_actual = generate_dry_schema(&actual);
        expect.assert_eq(&debug_actual);
    }

    fn check_operation_id(actual: &str, expect: Expect) {
        let actual = boilerplate(&format!(
            r#"
                "/test/example": {{
                    "get": {{
                        "operationId": "{actual}",
                        "responses": {{
                            "200": {{
                                "description": "OK"
                            }}
                        }}
                    }}
                }}
            "#
        ));
        let debug_actual = generate_dry_schema(&actual);
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
    fn defined_name_is_pascal() {
        check_operation_id(
            "testExample",
            expect![[r#"
              TestExample = Dry::Schema::Params do
              end
            "#]],
        );

        check_operation_id(
            "test-example",
            expect![[r#"
              TestExample = Dry::Schema::Params do
              end
            "#]],
        );

        check_operation_id(
            "test_example",
            expect![[r#"
              TestExample = Dry::Schema::Params do
              end
            "#]],
        );
    }

    #[test]
    fn query_types() {
        check_parameters(
            r#"
                [
                    {
                        "in": "query",
                        "name": "string_key",
                        "schema": {
                            "type": "string"
                        }
                    },
                    {
                        "in": "query",
                        "name": "integer_key",
                        "schema": {
                            "type": "integer"
                        }
                    },
                    {
                        "in": "query",
                        "name": "boolean_key",
                        "schema": {
                            "type": "boolean"
                        }
                    },
                    {
                        "in": "query",
                        "name": "array_key",
                        "schema": {
                            "type": "array"
                        }
                    }
                ]
            "#,
            expect![[r#"
                TestExample = Dry::Schema::Params do
                  optional(:string_key).value(:string)
                  optional(:integer_key).value(:integer)
                  optional(:boolean_key).value(:boolean)
                  optional(:array_key).value(:array)
                end
            "#]],
        );
    }

    #[test]
    fn query_required_and_optional() {
        check_parameters(
            r#"
                [
                    {
                        "in": "query",
                        "name": "required_key",
                        "require": true,
                        "schema": {
                            "type": "string"
                        }
                    },
                    {
                        "in": "query",
                        "name": "optional_key",
                        "schema": {
                            "type": "string"
                        }
                    }
                ]
            "#,
            expect![[r#"
                TestExample = Dry::Schema::Params do
                  optional(:required_key).value(:string)
                  optional(:optional_key).value(:string)
                end
            "#]],
        );
    }

    #[test]
    fn query_validates_integer() {
        check_parameters(
            r#"
                [
                    {
                        "in": "query",
                        "name": "user_id",
                        "required": true,
                        "schema": {
                            "type": "integer",
                            "maximum": 20,
                            "minimum": 10
                        }
                    }
                ]
            "#,
            expect![[r#"
                TestExample = Dry::Schema::Params do
                  required(:user_id).value(:integer, max: 20, min: 10)
                end
            "#]],
        );
    }

    #[test]
    fn query_validates_string() {
        check_parameters(
            r#"
                [
                    {
                        "in": "query",
                        "name": "user_id",
                        "required": true,
                        "schema": {
                            "type": "string",
                            "maxLength": 20,
                            "minLength": 10
                        }
                    }
                ]
            "#,
            expect![[r#"
                TestExample = Dry::Schema::Params do
                  required(:user_id).value(:string, max_size: 20, min_size: 10)
                end
            "#]],
        );
    }

    #[test]
    fn query_validates_array() {
        check_parameters(
            r#"
                [
                    {
                        "in": "query",
                        "name": "user_id",
                        "required": true,
                        "schema": {
                            "type": "array",
                            "maxItems": 10,
                            "minItems": 5,
                            "items": {}
                        }
                    }
                ]
            "#,
            expect![[r#"
                TestExample = Dry::Schema::Params do
                  required(:user_id).value(:array, max_size: 10, min_size: 5)
                end
            "#]],
        );
    }

    #[test]
    fn query_item_schema_in_array() {
        check_parameters(
            r#"
                [
                    {
                        "in": "query",
                        "name": "user_id",
                        "required": true,
                        "schema": {
                            "type": "array",
                            "maxItems": 3,
                            "minItems": 1,
                            "items": {
                                "type": "string",
                                "maxLength": 10,
                                "minLength": 5
                            }
                        }
                    }
                ]
            "#,
            expect![[r#"
                TestExample = Dry::Schema::Params do
                  required(:user_id).value(:array, max_size: 3, min_size: 1).each(:str?, max_size: 10, min_size: 5)
                end
            "#]],
        );
    }

    #[test]
    fn query_nested_array() {
        check_parameters(
            r#"
                [
                    {
                        "in": "query",
                        "name": "user_id",
                        "required": true,
                        "schema": {
                            "type": "array",
                            "maxItems": 3,
                            "minItems": 1,
                            "items": {
                                "type": "array",
                                "maxItems": 6,
                                "minItems": 2,
                                "items": {
                                    "type": "string",
                                    "maxLength": 20,
                                    "minLength": 15
                                }
                            }
                        }
                    }
                ]
            "#,
            expect![[r#"
                TestExample = Dry::Schema::Params do
                  required(:user_id).value(:array, max_size: 3, min_size: 1).each(:array?, max_size: 6, min_size: 2) do
                    schema(:array?).each(:str?, max_size: 20, min_size: 15)
                  end
                end
            "#]],
        );
    }

    #[test]
    fn query_no_item_array() {
        check_parameters(
            r#"
                [
                    {
                        "in": "query",
                        "name": "user_id",
                        "required": true,
                        "schema": {
                            "type": "array",
                            "maxItems": 3,
                            "minItems": 1,
                            "items": {
                                "type": "array",
                                "maxItems": 6,
                                "minItems": 2
                            }
                        }
                    }
                ]
            "#,
            expect![[r#"
                TestExample = Dry::Schema::Params do
                  required(:user_id).value(:array, max_size: 3, min_size: 1).each(:array?, max_size: 6, min_size: 2)
                end
            "#]],
        );
    }

    #[test]
    fn query_nested_array_no_validation() {
        check_parameters(
            r#"
                [
                    {
                        "in": "query",
                        "name": "user_id",
                        "required": true,
                        "schema": {
                            "type": "array",
                            "items": {
                                "type": "array",
                                "items": {
                                    "type": "array",
                                    "items": {
                                        "type": "string"
                                    }
                                }
                            }
                        }
                    }
                ]
            "#,
            expect![[r#"
                TestExample = Dry::Schema::Params do
                  required(:user_id).value(:array).each(:array?) do
                    schema(:array?).each(:array?) do
                      schema(:array?).each(:str?)
                    end
                  end
                end
            "#]],
        );
    }

    #[test]
    fn query_nested_array_with_validation() {
        check_parameters(
            r#"
                [
                    {
                        "in": "query",
                        "name": "user_id",
                        "required": true,
                        "schema": {
                            "type": "array",
                            "maxItems": 2,
                            "minItems": 1,
                            "items": {
                                "type": "array",
                                "maxItems": 4,
                                "minItems": 3,
                                "items": {
                                    "type": "array",
                                    "maxItems": 6,
                                    "minItems": 5,
                                    "items": {
                                        "type": "string",
                                        "maxLength": 8,
                                        "minLength": 7
                                    }
                                }
                            }
                        }
                    }
                ]
            "#,
            expect![[r#"
                TestExample = Dry::Schema::Params do
                  required(:user_id).value(:array, max_size: 2, min_size: 1).each(:array?, max_size: 4, min_size: 3) do
                    schema(:array?).each(:array?, max_size: 6, min_size: 5) do
                      schema(:array?).each(:str?, max_size: 8, min_size: 7)
                    end
                  end
                end
            "#]],
        );
    }
}
