mod common;

use expect_test::expect;

#[test]
fn reference_path_item_from_local_file() {
    common::check_with_local_file(
        r#"
            {
                "openapi": "3.0.0",
                "info": {
                    "title": "Testing API overview",
                    "version": "1.0.0"
                },
                "paths": {
                    "/example/test": {
                        "$ref": "./tests/tmp/aaa.json"
                    }
                }
            }
            ---
            ./tests/tmp/aaa.json
            {
                "get": {
                    "operationId": "testExample",
                    "parameters": [
                        {
                            "in": "query",
                            "name": "ref_string_key",
                            "schema": {
                                "type": "string"
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
          optional(:ref_string_key).value(:string)
        end
    "#]],
    );
}

#[test]
fn reference_path_item_by_json_from_server() {
    let (uri, _server) = common::once_mock_get_200(
        "/foo.json",
        r#"
            {
                "get": {
                    "operationId": "testExample",
                    "parameters": [
                        {
                            "in": "query",
                            "name": "ref_string_key",
                            "schema": {
                                "type": "string"
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
    );

    let openapi = common::boilerplate(&format!(
        r#"
            "/example/test": {{
                "$ref": "{uri}"
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

#[test]
fn reference_path_item_by_yaml_from_server() {
    let (uri, _server) = common::once_mock_get_200(
        "/foo.yaml",
        r#"
            get:
                operationId: testExample
                parameters:
                    - in: query
                      name: ref_string_key
                      schema:
                          type: string
                responses:
                    200:
                        description: OK
        "#,
    );

    let openapi = common::boilerplate(&format!(
        r#"
            "/example/test": {{
                "$ref": "{uri}"
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
