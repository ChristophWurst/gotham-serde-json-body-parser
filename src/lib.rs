extern crate failure;
extern crate futures;
extern crate gotham;
extern crate hyper;
extern crate mime;
extern crate serde;
extern crate serde_json;

use failure::Error;
use futures::{Future, Stream};
use gotham::handler::{HandlerError, IntoHandlerError};
use gotham::http::response::create_response;
use gotham::state::{FromState, State};
use hyper::{Body, Response, StatusCode};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{from_str, to_string};

pub trait JSONBody {
    fn json<'de, T: 'de>(
        self,
    ) -> Box<Future<Item = (State, T), Error = (State, HandlerError)> + 'de>
    where
        T: DeserializeOwned;
}

impl JSONBody for State {
    fn json<'de, T: 'de>(
        mut self,
    ) -> Box<Future<Item = (State, T), Error = (State, HandlerError)> + 'de>
    where
        T: DeserializeOwned,
    {
        let body = Body::take_from(&mut self);
        let f = body.concat2()
            .map_err(|err| Error::from(err))
            .then(|res| match res {
                Ok(body) => {
                    let json = String::from_utf8(body.to_vec()).unwrap();
                    match from_str(&json) {
                        Ok(parsed) => Ok((self, parsed)),
                        Err(err) => Err((self, Error::from(err))),
                    }
                }
                Err(err) => Err((self, err)),
            })
            .map_err(|(state, err)| {
                (
                    state,
                    HandlerError::with_status(
                        err.compat().into_handler_error(),
                        StatusCode::UnprocessableEntity,
                    ),
                )
            });

        Box::new(f)
    }
}

pub fn create_json_response<S: Serialize>(
    state: &State,
    status: StatusCode,
    data: &S,
) -> Result<Response, serde_json::Error> {
    to_string(data).map(|json_str| {
        create_response(
            state,
            status,
            Some((json_str.into_bytes(), mime::APPLICATION_JSON)),
        )
    })
}
