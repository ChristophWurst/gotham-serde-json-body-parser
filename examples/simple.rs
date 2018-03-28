extern crate futures;
extern crate gotham;
extern crate gotham_serde_json_body_parser;
extern crate hyper;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use futures::Future;
use gotham::handler::HandlerFuture;
use gotham::state::State;
use gotham_serde_json_body_parser::{create_json_response, JSONBody};
use hyper::StatusCode;

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

pub fn main() {
    let addr = "127.0.0.1:7878";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, || Ok(json_echo))
}
