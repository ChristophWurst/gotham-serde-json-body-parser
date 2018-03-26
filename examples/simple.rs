extern crate futures;
extern crate gotham;
extern crate gotham_serde_json_body_parser;
extern crate hyper;
extern crate mime;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use futures::Future;
use gotham::handler::HandlerFuture;
use gotham::http::response::create_response;
use gotham::state::State;
use gotham_serde_json_body_parser::JSONBody;
use hyper::{Response, StatusCode};

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

pub fn main() {
    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, || Ok(say_hello))
}
