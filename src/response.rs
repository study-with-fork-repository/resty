use hyper::{self, header};
use serde;
use serde_json;

use error::Error;

/// Resty response wrapper.
#[derive(Debug, Default)]
pub struct Response {
    pub(crate) response: hyper::Response,
}

impl From<Response> for hyper::Response {
    fn from(val: Response) -> Self {
        val.response
    }
}

impl<T: serde::Serialize> From<T> for Response {
    fn from(val: T) -> Self {
        let serialized = serde_json::to_vec(&val);
        match serialized {
            Ok(serialized) => {
                let response = hyper::Response::new()
                    .with_status(hyper::StatusCode::Ok)
                    .with_header(header::ContentType::json())
                    .with_body(serialized);
                Response { response }
            }
            Err(error) => {
                Error::internal("Unable to serialize response.", format!("{:?}", error)).into()
            }
        }
    }
}
