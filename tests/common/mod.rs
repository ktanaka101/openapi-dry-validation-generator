use std::fs::remove_file;

use expect_test::Expect;
use httptest::{http::Uri, matchers::request, responders::status_code, Expectation, Server};
use openapi_dry_validation_generator::generate_dry_validation_from_root_json;

#[allow(dead_code)]
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
pub fn once_mock_get_200(path: &'static str, stub_body: &'static str) -> (Uri, Server) {
    let server = Server::run();
    server.expect(
        Expectation::matching(request::method_path("GET", path))
            .times(1)
            .respond_with(status_code(200).body(stub_body)),
    );
    let uri = server.url(path);

    (uri, server)
}

#[allow(dead_code)]
pub fn check(actual: &str, expect: Expect) {
    let openapi = generate_dry_validation_from_root_json(actual);
    expect.assert_eq(&openapi);
}

#[allow(dead_code)]
pub fn check_with_local_file(actual: &str, expect: Expect) {
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
            let file_content = lines.collect::<Vec<&str>>().join("\n");

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
