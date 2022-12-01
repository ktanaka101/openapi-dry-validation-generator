use openapiv3::OpenAPI;

pub fn generate_dry_schema(text: &str) -> String {
    let openapi = serialize(text);
    openapi.openapi
}

fn serialize(text: &str) -> OpenAPI {
    serde_json::from_str(text).expect("Could not deserialize input")
}

#[cfg(test)]
mod tests {
    use expect_test::{expect, Expect};

    use crate::generate_dry_schema;

    fn check(actual: &str, expect: Expect) {
        let schema = boilerplate(actual);
        expect.assert_eq(&generate_dry_schema(&schema));
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
    fn it_works() {
        check(
            r#"
            "/test/example": {
              "get": {
                "responses": {
                  "200": {
                    "description": "OK"
                  }
                }
              }
            }
        "#,
            expect![["3.0.0"]],
        );
    }
}
