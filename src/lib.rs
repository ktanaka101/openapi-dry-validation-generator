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
    fn query_string() {
        check_parameters(
            r#"
                [
                    {
                        "in": "query",
                        "name": "user_id",
                        "required": true,
                        "schema": {
                            "type": "string"
                        }
                    }
                ]
            "#,
            expect![[r#"
                TestExample = Dry::Schema::Params do
                  required(user_id).value(:string)
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
                required(user_id).value(:integer, max: 20, min: 10)
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
                  required(user_id).value(:string, max_size: 20, min_size: 10)
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
                  required(user_id).value(:array, max_size: 10, min_size: 5)
                end
            "#]],
        );
    }
}
