mod common;

use expect_test::{expect, Expect};
use httptest::{matchers::*, responders::*, Expectation, Server};
use openapi_dry_validation_generator::generate_dry_validation_from_root_json;

fn check(actual: &str, expect: Expect) {
    let openapi = generate_dry_validation_from_root_json(actual);
    expect.assert_eq(&openapi);
}

#[test]
fn reference_path_item_from_server() {
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
