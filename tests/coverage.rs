use std::collections::BTreeSet;

fn spec_routes(markdown: &str) -> BTreeSet<(String, String)> {
    let mut routes = BTreeSet::new();
    let mut heading = "";
    for line in markdown.lines() {
        if line.starts_with('#') {
            heading = line.trim_start_matches('#').trim();
        }
        let Some(rest) = line.strip_prefix('`') else {
            continue;
        };
        let Some((method, rest)) = rest.split_once(' ') else {
            continue;
        };
        if !matches!(method, "GET" | "POST" | "PATCH" | "PUT" | "DELETE") {
            continue;
        }
        let Some((path, _)) = rest.split_once('`') else {
            continue;
        };
        let path = path
            .split(['?', '&'])
            .next()
            .unwrap_or(path)
            .trim_end_matches('/');
        if !path.starts_with("/api") {
            continue;
        }
        if heading.to_ascii_lowercase().starts_with("example") {
            continue;
        }
        routes.insert((method.to_string(), path.to_string()));
    }
    routes
}

fn doc_routes(rust: &str) -> BTreeSet<(String, String)> {
    let mut routes = BTreeSet::new();
    for line in rust.lines() {
        let line = line.trim();
        let Some(rest) = line.strip_prefix("/// `") else {
            continue;
        };
        let Some((method, rest)) = rest.split_once(' ') else {
            continue;
        };
        if !matches!(method, "GET" | "POST" | "PATCH" | "PUT" | "DELETE") {
            continue;
        }
        let Some((path, _)) = rest.split_once('`') else {
            continue;
        };
        routes.insert((method.to_string(), path.to_string()));
    }
    routes
}

/// This is a syntactic route-presence check. Endpoint contract behavior is covered
/// separately by focused request/response tests.
#[test]
fn generated_raw_route_docs_match_vendored_spec_routes() {
    let spec = spec_routes(include_str!("../docs/fleet-rest-api-main.md"));
    let api = doc_routes(include_str!("../src/endpoints/raw_api.rs"));
    assert_eq!(api, spec);
}
