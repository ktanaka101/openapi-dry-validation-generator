mod common;

use expect_test::{expect, Expect};

use openapi_dry_validation_generator::generate_dry_validation_from_root_json;

fn check_operations(actual: &str, expect: Expect) {
    let actual = common::boilerplate(&format!(
        r#"
            "/test/example": {actual}
        "#
    ));
    let debug_actual = generate_dry_validation_from_root_json(&actual);
    expect.assert_eq(&debug_actual);
}

#[test]
fn operations() {
    check_operations(
        r#"
            {
                "get": {
                    "operationId": "get-test",
                    "parameters": [
                        {
                            "in": "query",
                            "name": "get_key",
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
                },
                "post": {
                    "operationId": "post-test",
                    "parameters": [
                        {
                            "in": "query",
                            "name": "post_key",
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
                },
                "patch": {
                    "operationId": "patch-test",
                    "parameters": [
                        {
                            "in": "query",
                            "name": "patch_key",
                            "schema": {
                                "type": "boolean"
                            }
                        }
                    ],
                    "responses": {
                        "200": {
                            "description": "OK"
                        }
                    }
                },
                "put": {
                    "operationId": "put-test",
                    "parameters": [
                        {
                            "in": "query",
                            "name": "put_key",
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
                },
                "delete": {
                    "operationId": "delete-test",
                    "parameters": [
                        {
                            "in": "query",
                            "name": "delete_key",
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
            GetTest = Dry::Schema::Params do
              optional(:get_key).value(:string)
            end
            PostTest = Dry::Schema::Params do
              optional(:post_key).value(:integer)
            end
            PatchTest = Dry::Schema::Params do
              optional(:patch_key).value(:boolean)
            end
            PutTest = Dry::Schema::Params do
              optional(:put_key).value(:string)
            end
            DeleteTest = Dry::Schema::Params do
              optional(:delete_key).value(:integer)
            end
        "#]],
    );
}
