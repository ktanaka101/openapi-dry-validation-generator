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
                    },
                    {
                        "in": "query",
                        "name": "object_key",
                        "schema": {
                            "type": "object"
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
                  optional(:object_key).value(:hash)
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
                    },
                    {
                        "in": "query",
                        "name": "required_object_key",
                        "required": true,
                        "schema": {
                            "type": "object"
                        }
                    },
                    {
                        "in": "query",
                        "name": "optional_object_key",
                        "schema": {
                            "type": "object"
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
                  required(:required_object_key).value(:hash)
                  optional(:optional_object_key).value(:hash)
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
                  required(:user_id).value(:integer, min: 10, max: 20)
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
                  required(:user_id).value(:string, min_size: 10, max_size: 20)
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
                  required(:user_id).value(:array, min_size: 5, max_size: 10)
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
                    },
                    {
                        "in": "query",
                        "name": "object_item",
                        "schema": {
                            "type": "array",
                            "items": {
                                "type": "object"
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
                  optional(:object_item).value(:array).each(:hash?)
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
                    },
                    {
                        "in": "query",
                        "name": "object_item",
                        "schema": {
                            "type": "array",
                            "minItems": 15,
                            "maxItems": 16,
                            "items": {
                                "type": "object"
                            }
                        }
                    }
                ]
            "#,
            expect![[r#"
                TestExample = Dry::Schema::Params do
                  optional(:integer_item).value(:array, min_size: 1, max_size: 2).each(:int?, min: 3, max: 4)
                  optional(:string_item).value(:array, min_size: 5, max_size: 6).each(:str?, min_size: 7, max_size: 8)
                  optional(:boolean_item).value(:array, min_size: 9, max_size: 10).each(:bool?)
                  optional(:array_item).value(:array, min_size: 11, max_size: 12).each(:array?, min_size: 13, max_size: 14)
                  optional(:object_item).value(:array, min_size: 15, max_size: 16).each(:hash?)
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
                    },
                    {
                        "in": "query",
                        "name": "nested_object",
                        "required": true,
                        "schema": {
                            "type": "array",
                            "items": {
                                "type": "array",
                                "items": {
                                    "type": "array",
                                    "items": {
                                        "type": "object"
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
                  required(:nested_object).value(:array).each(:array?) do
                    schema(:array?).each(:array?) do
                      schema(:array?).each(:hash?)
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
                    },
                    {
                        "in": "query",
                        "name": "nested_object",
                        "required": true,
                        "schema": {
                            "type": "array",
                            "minItems": 31,
                            "maxItems": 32,
                            "items": {
                                "type": "array",
                                "minItems": 33,
                                "maxItems": 34,
                                "items": {
                                    "type": "array",
                                    "minItems": 35,
                                    "maxItems": 36,
                                    "items": {
                                        "type": "object"
                                    }
                                }
                            }
                        }
                    }
                ]
            "#,
            expect![[r#"
                TestExample = Dry::Schema::Params do
                  required(:nested_integer).value(:array, min_size: 1, max_size: 2).each(:array?, min_size: 3, max_size: 4) do
                    schema(:array?).each(:array?, min_size: 5, max_size: 6) do
                      schema(:array?).each(:int?, min: 7, max: 8)
                    end
                  end
                  required(:nested_string).value(:array, min_size: 9, max_size: 10).each(:array?, min_size: 11, max_size: 12) do
                    schema(:array?).each(:array?, min_size: 13, max_size: 14) do
                      schema(:array?).each(:str?, min_size: 15, max_size: 16)
                    end
                  end
                  required(:nested_boolean).value(:array, min_size: 17, max_size: 18).each(:array?, min_size: 19, max_size: 20) do
                    schema(:array?).each(:array?, min_size: 21, max_size: 22) do
                      schema(:array?).each(:bool?)
                    end
                  end
                  required(:nested_array).value(:array, min_size: 23, max_size: 24).each(:array?, min_size: 25, max_size: 26) do
                    schema(:array?).each(:array?, min_size: 27, max_size: 28) do
                      schema(:array?).each(:array?, min_size: 29, max_size: 30)
                    end
                  end
                  required(:nested_object).value(:array, min_size: 31, max_size: 32).each(:array?, min_size: 33, max_size: 34) do
                    schema(:array?).each(:array?, min_size: 35, max_size: 36) do
                      schema(:array?).each(:hash?)
                    end
                  end
                end
            "#]],
        );
    }

    #[test]
    fn query_property_types_in_hash() {
        check_parameters(
            r#"
                [
                    {
                        "in": "query",
                        "name": "integer_property",
                        "schema": {
                            "type": "object",
                            "properties": {
                                "integer_prop": {
                                    "type": "integer"
                                },
                                "string_prop": {
                                    "type": "string"
                                },
                                "boolean_prop": {
                                    "type": "boolean"
                                },
                                "array_prop": {
                                    "type": "array"
                                },
                                "object_prop": {
                                    "type": "object"
                                }
                            }
                        }
                    }
                ]
            "#,
            expect![[r#"
                TestExample = Dry::Schema::Params do
                  optional(:integer_property).value(:hash) do
                    optional(:integer_prop).value(:integer)
                    optional(:string_prop).value(:string)
                    optional(:boolean_prop).value(:boolean)
                    optional(:array_prop).value(:array)
                    optional(:object_prop).value(:hash)
                  end
                end
            "#]],
        );
    }

    #[test]
    fn query_property_types_with_validation_in_object() {
        check_parameters(
            r#"
                [
                    {
                        "in": "query",
                        "name": "property_types",
                        "schema": {
                            "type": "object",
                            "properties": {
                                "integer_prop": {
                                    "type": "integer",
                                    "minimum": 1,
                                    "maximum": 2
                                },
                                "string_prop": {
                                    "type": "string",
                                    "minLength": 3,
                                    "maxLength": 4
                                },
                                "boolean_prop": {
                                    "type": "boolean"
                                },
                                "array_prop": {
                                    "type": "array",
                                    "minItems": 5,
                                    "maxItems": 6
                                },
                                "object_prop": {
                                    "type": "object"
                                }
                            }
                        }
                    }
                ]
            "#,
            expect![[r#"
                TestExample = Dry::Schema::Params do
                  optional(:property_types).value(:hash) do
                    optional(:integer_prop).value(:integer, min: 1, max: 2)
                    optional(:string_prop).value(:string, min_size: 3, max_size: 4)
                    optional(:boolean_prop).value(:boolean)
                    optional(:array_prop).value(:array, min_size: 5, max_size: 6)
                    optional(:object_prop).value(:hash)
                  end
                end
            "#]],
        );
    }

    #[test]
    fn query_nested_object() {
        check_parameters(
            r#"
                [
                    {
                        "in": "query",
                        "name": "nested_object",
                        "required": true,
                        "schema": {
                            "type": "object",
                            "properties": {
                                "nested_1": {
                                    "type": "object",
                                    "properties": {
                                        "nested_2": {
                                            "type": "object",
                                            "properties": {
                                                "nested_3_1": {
                                                    "type": "integer"
                                                },
                                                "nested_3_2": {
                                                    "type": "string"
                                                },
                                                "nested_3_3": {
                                                    "type": "boolean"
                                                },
                                                "nested_3_4": {
                                                    "type": "array"
                                                },
                                                "nested_3_5": {
                                                    "type": "object"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                ]
            "#,
            expect![[r#"
                TestExample = Dry::Schema::Params do
                  required(:nested_object).value(:hash) do
                    optional(:nested_1).value(:hash) do
                      optional(:nested_2).value(:hash) do
                        optional(:nested_3_1).value(:integer)
                        optional(:nested_3_2).value(:string)
                        optional(:nested_3_3).value(:boolean)
                        optional(:nested_3_4).value(:array)
                        optional(:nested_3_5).value(:hash)
                      end
                    end
                  end
                end
            "#]],
        );
    }

    #[test]
    fn query_nested_object_with_validation() {
        check_parameters(
            r#"
                [
                    {
                        "in": "query",
                        "name": "nested_object",
                        "required": true,
                        "schema": {
                            "type": "object",
                            "properties": {
                                "nested_1": {
                                    "type": "object",
                                    "properties": {
                                        "nested_2": {
                                            "type": "object",
                                            "properties": {
                                                "nested_3_1": {
                                                    "type": "integer",
                                                    "minimum": 1,
                                                    "maximum": 2
                                                },
                                                "nested_3_2": {
                                                    "type": "string",
                                                    "minLength": 3,
                                                    "maxLength": 4
                                                },
                                                "nested_3_3": {
                                                    "type": "boolean"
                                                },
                                                "nested_3_4": {
                                                    "type": "array",
                                                    "minItems": 5,
                                                    "maxItems": 6
                                                },
                                                "nested_3_5": {
                                                    "type": "object"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                ]
            "#,
            expect![[r#"
                TestExample = Dry::Schema::Params do
                  required(:nested_object).value(:hash) do
                    optional(:nested_1).value(:hash) do
                      optional(:nested_2).value(:hash) do
                        optional(:nested_3_1).value(:integer, min: 1, max: 2)
                        optional(:nested_3_2).value(:string, min_size: 3, max_size: 4)
                        optional(:nested_3_3).value(:boolean)
                        optional(:nested_3_4).value(:array, min_size: 5, max_size: 6)
                        optional(:nested_3_5).value(:hash)
                      end
                    end
                  end
                end
            "#]],
        );
    }

    #[test]
    fn query_nested_object_with_required() {
        check_parameters(
            r#"
                [
                    {
                        "in": "query",
                        "name": "nested_object",
                        "required": true,
                        "schema": {
                            "type": "object",
                            "properties": {
                                "nested_1": {
                                    "type": "object",
                                    "properties": {
                                        "nested_2": {
                                            "type": "object",
                                            "properties": {
                                                "nested_3_1": {
                                                    "type": "integer",
                                                    "minimum": 1,
                                                    "maximum": 2
                                                },
                                                "nested_3_2": {
                                                    "type": "string",
                                                    "minLength": 3,
                                                    "maxLength": 4
                                                },
                                                "nested_3_3": {
                                                    "type": "boolean"
                                                }
                                            },
                                            "required": ["nested_3_1", "nested_3_3"]
                                        }
                                    },
                                    "required": ["nested_2"]
                                }
                            },
                            "required": ["nested_1"]
                        }
                    }
                ]
            "#,
            expect![[r#"
                TestExample = Dry::Schema::Params do
                  required(:nested_object).value(:hash) do
                    required(:nested_1).value(:hash) do
                      required(:nested_2).value(:hash) do
                        required(:nested_3_1).value(:integer, min: 1, max: 2)
                        optional(:nested_3_2).value(:string, min_size: 3, max_size: 4)
                        required(:nested_3_3).value(:boolean)
                      end
                    end
                  end
                end
            "#]],
        );
    }

    #[test]
    fn query_object_required_with_the_same_name() {
        check_parameters(
            r#"
                [
                    {
                        "in": "query",
                        "name": "nested_object",
                        "required": true,
                        "schema": {
                            "type": "object",
                            "properties": {
                                "same_key": {
                                    "type": "object",
                                    "properties": {
                                        "same_key": {
                                            "type": "boolean"
                                        }
                                    }
                                }
                            },
                            "required": ["same_key"]
                        }
                    }
                ]
            "#,
            expect![[r#"
                TestExample = Dry::Schema::Params do
                  required(:nested_object).value(:hash) do
                    required(:same_key).value(:hash) do
                      optional(:same_key).value(:boolean)
                    end
                  end
                end
            "#]],
        );
    }
}
