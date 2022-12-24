mod common;

use expect_test::{expect, Expect};

use openapi_dry_validation_generator::generate_dry_validation_from_root_json;

fn check_operation_id(actual: &str, expect: Expect) {
    let actual = common::boilerplate(&format!(
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
    let debug_actual = generate_dry_validation_from_root_json(&actual);
    expect.assert_eq(&debug_actual);
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
