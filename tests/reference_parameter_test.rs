mod common;

use expect_test::expect;

#[test]
fn reference_parameter_from_local() {
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
                                    "$ref": "#/components/parameters/StringKeyParam"
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
                    "parameters": {
                        "StringKeyParam": {
                            "in": "query",
                            "name": "string_key",
                            "schema": {
                                "type": "string"
                            }
                        }
                    }
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
fn reference_parameter_by_json_from_local_file() {
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
fn reference_parameter_by_yaml_from_local_file() {
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
                                    "$ref": "./tests/tmp/foo.yaml"
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
            in: query
            name: string_key
            schema:
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
fn reference_parameter_from_server() {
    let (uri, _server) = common::once_mock_get_200(
        "/foo.json",
        r#"
            {
                "in": "query",
                "name": "ref_string_key",
                "schema": {
                    "type": "string"
                }
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
                            "$ref": "{uri}"
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
