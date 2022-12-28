use std::fs::remove_file;

use expect_test::{expect, Expect};
use openapi_dry_validation_generator::generate_dry_validation_from_root_json;

mod common;

use httptest::{matchers::*, responders::*, Expectation, Server};

fn check(actual: &str, expect: Expect) {
    let openapi = generate_dry_validation_from_root_json(actual);
    expect.assert_eq(&openapi);
}

fn check_with_local_file(actual: &str, expect: Expect) {
    let mut inputs = actual.split("---");
    let openapi = inputs.next().unwrap();
    let other_files = inputs
        .map(|input_file| {
            let mut lines = input_file.lines();
            // ignore new line
            lines.next().unwrap();
            let file_path = lines
                .next()
                .expect("Not found file path.(ex. `./tmp/example.json`)")
                .trim();
            let file_content = lines.collect::<String>();
            (file_path, file_content)
        })
        .collect::<Vec<_>>();
    for file in &other_files {
        std::fs::write(file.0, &file.1).unwrap();
    }

    let openapi = generate_dry_validation_from_root_json(openapi);
    for file in &other_files {
        remove_file(file.0).unwrap();
    }

    expect.assert_eq(&openapi);
}

#[test]
fn reference_path_item_from_local_file() {
    check_with_local_file(
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
    let server = Server::run();
    let stub_body = r#"
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
    "#;
    server.expect(
        Expectation::matching(request::method_path("GET", "/foo.json"))
            .times(1)
            .respond_with(status_code(200).body(stub_body)),
    );
    let url = server.url("/foo.json");

    let openapi = common::boilerplate(&format!(
        r#"
            "/example/test": {{
                "$ref": "{url}"
            }}
        "#
    ));
    check(
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
    let server = Server::run();
    let stub_body = r#"
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
    "#;
    server.expect(
        Expectation::matching(request::method_path("GET", "/foo.yaml"))
            .times(1)
            .respond_with(status_code(200).body(stub_body)),
    );
    let url = server.url("/foo.yaml");

    let openapi = common::boilerplate(&format!(
        r#"
            "/example/test": {{
                "$ref": "{url}"
            }}
        "#
    ));
    check(
        &openapi,
        expect![[r#"
            TestExample = Dry::Schema::Params do
              optional(:ref_string_key).value(:string)
            end
        "#]],
    );
}
