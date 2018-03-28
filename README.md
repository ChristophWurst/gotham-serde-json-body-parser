# gotham-serde-json-body-parser
JSON body parser for the Gotham web framework.

[![Crates.io](https://img.shields.io/crates/v/gotham_serde_json_body_parser.svg)](https://crates.io/crates/gotham_serde_json_body_parser)

This is a simple integration of `serde_json` crate to eliminate the boilerplate code of parsing a request body. If parsing fails, a HTTP 422 (Unprocessable entity) is returned. This crate also provides a convenience function to create JSON responses.

```rust
use gotham_serde_json_body_parser::{create_json_response, JSONBody};

#[derive(Debug, Deserialize, Serialize)]
struct Person {
    name: String,
}

pub fn json_echo(state: State) -> Box<HandlerFuture> {
    Box::new(state.json::<Person>().and_then(|(state, person)| {
        let res = create_json_response(&state, StatusCode::Ok, &person).unwrap();
        Ok((state, res))
    }))
}
```
