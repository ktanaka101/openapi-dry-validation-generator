use expect_test::Expect;
use openapi_dry_validation_generator::generate_dry_validation_from_root_json;

pub fn boilerplate(input: &str) -> String {
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

#[allow(dead_code)]
pub fn check(actual: &str, expect: Expect) {
    let openapi = generate_dry_validation_from_root_json(actual);
    expect.assert_eq(&openapi);
}
