# gotham-serde-json-body-parser
JSON body parser for the Gotham web framework.

[![Crates.io](https://img.shields.io/crates/v/gotham_serde_json_body_parser.svg)](https://crates.io/crates/gotham_serde_json_body_parser)

This is a simple integration of `serde_json` crate to eliminate the boilerplate code of parsing a request body. If parsing fails, a HTTP 422 (Unprocessable entity) is returned.

```rust
use gotham_serde_json_body_parser::JSONBody;

#[derive(Debug, Deserialize)]
struct Person {
    name: String,
}

pub fn say_hello(state: State) -> Box<HandlerFuture> {
    Box::new(state.json::<Person>().and_then(|(state, person)| {
        let res = create_response(
            &state,
            StatusCode::Ok,
            Some((
                format!("Hello, {}!", person.name).into_bytes(),
                mime::TEXT_PLAIN,
            )),
        );

        Ok((state, res))
    }))
}
```
