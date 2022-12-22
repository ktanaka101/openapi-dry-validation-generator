use expect_test::{expect, Expect};

use openapi_dry_validation_generator::generate_dry_validation_from_json;

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
    let debug_actual = generate_dry_validation_from_json(&actual);
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
