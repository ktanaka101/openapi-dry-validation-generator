mod common;

use expect_test::expect;

#[test]
fn reference_string_schema_from_local() {
    common::check_with_local_file(
        r##"
            {
                "openapi": "3.0.0",
                "info": {
                    "title": "Testing API overview",
                    "version": "1.0.0"
                },
                "paths": {
                    "/example/test": {
                        "get": {
                            "operationId": "test-example",
                            "parameters": [
                                {
                                    "in": "query",
                                    "name": "string_key",
                                    "schema": {
                                        "$ref": "#/components/schemas/StringSchema"
                                    }
                                },
                                {
                                    "in": "query",
                                    "name": "integer_key",
                                    "schema": {
                                        "$ref": "#/components/schemas/IntegerSchema"
                                    }
                                },
                                {
                                    "in": "query",
                                    "name": "boolean_key",
                                    "schema": {
                                        "$ref": "#/components/schemas/BooleanSchema"
                                    }
                                },
                                {
                                    "in": "query",
                                    "name": "array_key",
                                    "schema": {
                                        "$ref": "#/components/schemas/ArraySchema"
                                    }
                                },
                                {
                                    "in": "query",
                                    "name": "object_key",
                                    "schema": {
                                        "$ref": "#/components/schemas/ObjectSchema"
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
                },
                "components": {
                    "schemas": {
                        "StringSchema": {
                            "type": "string"
                        },
                        "IntegerSchema": {
                            "type": "integer"
                        },
                        "BooleanSchema": {
                            "type": "boolean"
                        },
                        "ArraySchema": {
                            "type": "array"
                        },
                        "ObjectSchema": {
                            "type": "object"
                        }
                    }
                }
            }
        "##,
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
fn reference_schema_by_json_from_local_file() {
    common::check_with_local_file(
        r##"
            {
                "openapi": "3.0.0",
                "info": {
                    "title": "Testing API overview",
                    "version": "1.0.0"
                },
                "paths": {
                    "/example/test": {
                        "get": {
                            "operationId": "test-example",
                            "parameters": [
                                {
                                    "$ref": "./tests/tmp/foo.json"
                                }
                            ],
                            "responses": {
                                "200": {
                                    "description": "OK"
                                }
                            }
                        }
                    }
                }
            }
            ---
            ./tests/tmp/foo.json
            {
                "in": "query",
                "name": "string_key",
                "schema": {
                    "type": "string"
                }
            }
        "##,
        expect![[r#"
            TestExample = Dry::Schema::Params do
              optional(:string_key).value(:string)
            end
        "#]],
    );
}

#[test]
fn reference_schema_by_yaml_from_local_file() {
    common::check_with_local_file(
        r##"
            {
                "openapi": "3.0.0",
                "info": {
                    "title": "Testing API overview",
                    "version": "1.0.0"
                },
                "paths": {
                    "/example/test": {
                        "get": {
                            "operationId": "test-example",
                            "parameters": [
                                {
                                    "in": "query",
                                    "name": "string_key",
                                    "schema": {
                                        "$ref": "./tests/tmp/foo.yaml"
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
                }
            }
            ---
            ./tests/tmp/foo.yaml
            type: string
        "##,
        expect![[r#"
            TestExample = Dry::Schema::Params do
              optional(:string_key).value(:string)
            end
        "#]],
    );
}

#[test]
fn reference_schema_from_server() {
    let (uri, _server) = common::once_mock_get_200(
        "/foo.json",
        r#"
            {
                "type": "string"
            }
        "#,
    );

    let openapi = common::boilerplate(&format!(
        r#"
            "/example/test": {{
                "get": {{
                    "operationId": "test-example",
                    "parameters": [
                        {{
                            "in": "query",
                            "name": "ref_string_key",
                            "schema": {{
                                "$ref": "{uri}"
                            }}
                        }}
                    ],
                    "responses": {{
                        "200": {{
                            "description": "OK"
                        }}
                    }}
                }}
            }}
        "#
    ));
    common::check(
        &openapi,
        expect![[r#"
            TestExample = Dry::Schema::Params do
              optional(:ref_string_key).value(:string)
            end
        "#]],
    );
}
