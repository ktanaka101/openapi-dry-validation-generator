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
fn reference_parameter_from_local_file() {
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
                                    "$ref": "./foo.json"
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
            ./foo.json
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
