#[cfg(test)]
mod tests {
    use expect_test::{expect, Expect};

    use openapi_dry_validation_generator::generate_dry_validation;

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
        let debug_actual = generate_dry_validation(&actual);
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
        let debug_actual = generate_dry_validation(&actual);
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
                        "name": "required_integer_key",
                        "required": true,
                        "schema": {
                            "type": "integer"
                        }
                    },
                    {
                        "in": "query",
                        "name": "optional_integer_key",
                        "schema": {
                            "type": "integer"
                        }
                    },
                    {
                        "in": "query",
                        "name": "required_string_key",
                        "required": true,
                        "schema": {
                            "type": "string"
                        }
                    },
                    {
                        "in": "query",
                        "name": "optional_string_key",
                        "schema": {
                            "type": "string"
                        }
                    },
                    {
                        "in": "query",
                        "name": "required_boolean_key",
                        "required": true,
                        "schema": {
                            "type": "boolean"
                        }
                    },
                    {
                        "in": "query",
                        "name": "optional_boolean_key",
                        "schema": {
                            "type": "boolean"
                        }
                    },
                    {
                        "in": "query",
                        "name": "required_array_key",
                        "required": true,
                        "schema": {
                            "type": "array"
                        }
                    },
                    {
                        "in": "query",
                        "name": "optional_array_key",
                        "schema": {
                            "type": "array"
                        }
                    }
                ]
            "#,
            expect![[r#"
                TestExample = Dry::Schema::Params do
                  required(:required_integer_key).value(:integer)
                  optional(:optional_integer_key).value(:integer)
                  required(:required_string_key).value(:string)
                  optional(:optional_string_key).value(:string)
                  required(:required_boolean_key).value(:boolean)
                  optional(:optional_boolean_key).value(:boolean)
                  required(:required_array_key).value(:array)
                  optional(:optional_array_key).value(:array)
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
                            "minimum": 10,
                            "maximum": 20
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
                            "minLength": 10,
                            "maxLength": 20
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
                            "minItems": 5,
                            "maxItems": 10,
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
    fn query_item_types_in_array() {
        check_parameters(
            r#"
                [
                    {
                        "in": "query",
                        "name": "integer_item",
                        "schema": {
                            "type": "array",
                            "items": {
                                "type": "integer"
                            }
                        }
                    },
                    {
                        "in": "query",
                        "name": "string_item",
                        "schema": {
                            "type": "array",
                            "items": {
                                "type": "string"
                            }
                        }
                    },
                    {
                        "in": "query",
                        "name": "boolean_item",
                        "schema": {
                            "type": "array",
                            "items": {
                                "type": "boolean"
                            }
                        }
                    },
                    {
                        "in": "query",
                        "name": "array_item",
                        "schema": {
                            "type": "array",
                            "items": {
                                "type": "array"
                            }
                        }
                    }
                ]
            "#,
            expect![[r#"
                TestExample = Dry::Schema::Params do
                  optional(:integer_item).value(:array).each(:int?)
                  optional(:string_item).value(:array).each(:str?)
                  optional(:boolean_item).value(:array).each(:bool?)
                  optional(:array_item).value(:array).each(:array?)
                end
            "#]],
        );
    }

    #[test]
    fn query_item_types_with_validation_in_array() {
        check_parameters(
            r#"
                [
                    {
                        "in": "query",
                        "name": "integer_item",
                        "schema": {
                            "type": "array",
                            "minItems": 1,
                            "maxItems": 2,
                            "items": {
                                "type": "integer",
                                "minimum": 3,
                                "maximum": 4
                            }
                        }
                    },
                    {
                        "in": "query",
                        "name": "string_item",
                        "schema": {
                            "type": "array",
                            "minItems": 5,
                            "maxItems": 6,
                            "items": {
                                "type": "string",
                                "minLength": 7,
                                "maxLength": 8
                            }
                        }
                    },
                    {
                        "in": "query",
                        "name": "boolean_item",
                        "schema": {
                            "type": "array",
                            "minItems": 9,
                            "maxItems": 10,
                            "items": {
                                "type": "boolean"
                            }
                        }
                    },
                    {
                        "in": "query",
                        "name": "array_item",
                        "schema": {
                            "type": "array",
                            "minItems": 11,
                            "maxItems": 12,
                            "items": {
                                "type": "array",
                                "minItems": 13,
                                "maxItems": 14
                            }
                        }
                    }
                ]
            "#,
            expect![[r#"
                TestExample = Dry::Schema::Params do
                  optional(:integer_item).value(:array, max_size: 2, min_size: 1).each(:int?, max: 4, min: 3)
                  optional(:string_item).value(:array, max_size: 6, min_size: 5).each(:str?, max_size: 8, min_size: 7)
                  optional(:boolean_item).value(:array, max_size: 10, min_size: 9).each(:bool?)
                  optional(:array_item).value(:array, max_size: 12, min_size: 11).each(:array?, max_size: 14, min_size: 13)
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
                        "name": "nested_integer",
                        "required": true,
                        "schema": {
                            "type": "array",
                            "items": {
                                "type": "array",
                                "items": {
                                    "type": "array",
                                    "items": {
                                        "type": "integer"
                                    }
                                }
                            }
                        }
                    },
                    {
                        "in": "query",
                        "name": "nested_string",
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
                    },
                    {
                        "in": "query",
                        "name": "nested_boolean",
                        "required": true,
                        "schema": {
                            "type": "array",
                            "items": {
                                "type": "array",
                                "items": {
                                    "type": "array",
                                    "items": {
                                        "type": "boolean"
                                    }
                                }
                            }
                        }
                    },
                    {
                        "in": "query",
                        "name": "nested_array",
                        "required": true,
                        "schema": {
                            "type": "array",
                            "items": {
                                "type": "array",
                                "items": {
                                    "type": "array",
                                    "items": {
                                        "type": "array"
                                    }
                                }
                            }
                        }
                    }
                ]
            "#,
            expect![[r#"
                TestExample = Dry::Schema::Params do
                  required(:nested_integer).value(:array).each(:array?) do
                    schema(:array?).each(:array?) do
                      schema(:array?).each(:int?)
                    end
                  end
                  required(:nested_string).value(:array).each(:array?) do
                    schema(:array?).each(:array?) do
                      schema(:array?).each(:str?)
                    end
                  end
                  required(:nested_boolean).value(:array).each(:array?) do
                    schema(:array?).each(:array?) do
                      schema(:array?).each(:bool?)
                    end
                  end
                  required(:nested_array).value(:array).each(:array?) do
                    schema(:array?).each(:array?) do
                      schema(:array?).each(:array?)
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
                        "name": "nested_integer",
                        "required": true,
                        "schema": {
                            "type": "array",
                            "minItems": 1,
                            "maxItems": 2,
                            "items": {
                                "type": "array",
                                "minItems": 3,
                                "maxItems": 4,
                                "items": {
                                    "type": "array",
                                    "minItems": 5,
                                    "maxItems": 6,
                                    "items": {
                                        "type": "integer",
                                        "minimum": 7,
                                        "maximum": 8
                                    }
                                }
                            }
                        }
                    },
                    {
                        "in": "query",
                        "name": "nested_string",
                        "required": true,
                        "schema": {
                            "type": "array",
                            "minItems": 9,
                            "maxItems": 10,
                            "items": {
                                "type": "array",
                                "minItems": 11,
                                "maxItems": 12,
                                "items": {
                                    "type": "array",
                                    "minItems": 13,
                                    "maxItems": 14,
                                    "items": {
                                        "type": "string",
                                        "minLength": 15,
                                        "maxLength": 16
                                    }
                                }
                            }
                        }
                    },
                    {
                        "in": "query",
                        "name": "nested_boolean",
                        "required": true,
                        "schema": {
                            "type": "array",
                            "minItems": 17,
                            "maxItems": 18,
                            "items": {
                                "type": "array",
                                "minItems": 19,
                                "maxItems": 20,
                                "items": {
                                    "type": "array",
                                    "minItems": 21,
                                    "maxItems": 22,
                                    "items": {
                                        "type": "boolean"
                                    }
                                }
                            }
                        }
                    },
                    {
                        "in": "query",
                        "name": "nested_array",
                        "required": true,
                        "schema": {
                            "type": "array",
                            "minItems": 23,
                            "maxItems": 24,
                            "items": {
                                "type": "array",
                                "minItems": 25,
                                "maxItems": 26,
                                "items": {
                                    "type": "array",
                                    "minItems": 27,
                                    "maxItems": 28,
                                    "items": {
                                        "type": "array",
                                        "minItems": 29,
                                        "maxItems": 30
                                    }
                                }
                            }
                        }
                    }
                ]
            "#,
            expect![[r#"
                TestExample = Dry::Schema::Params do
                  required(:nested_integer).value(:array, max_size: 2, min_size: 1).each(:array?, max_size: 4, min_size: 3) do
                    schema(:array?).each(:array?, max_size: 6, min_size: 5) do
                      schema(:array?).each(:int?, max: 8, min: 7)
                    end
                  end
                  required(:nested_string).value(:array, max_size: 10, min_size: 9).each(:array?, max_size: 12, min_size: 11) do
                    schema(:array?).each(:array?, max_size: 14, min_size: 13) do
                      schema(:array?).each(:str?, max_size: 16, min_size: 15)
                    end
                  end
                  required(:nested_boolean).value(:array, max_size: 18, min_size: 17).each(:array?, max_size: 20, min_size: 19) do
                    schema(:array?).each(:array?, max_size: 22, min_size: 21) do
                      schema(:array?).each(:bool?)
                    end
                  end
                  required(:nested_array).value(:array, max_size: 24, min_size: 23).each(:array?, max_size: 26, min_size: 25) do
                    schema(:array?).each(:array?, max_size: 28, min_size: 27) do
                      schema(:array?).each(:array?, max_size: 30, min_size: 29)
                    end
                  end
                end
            "#]],
        );
    }
}
