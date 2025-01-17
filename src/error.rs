use hyper::{self, header};
use serde_json;

use response::Response;
use StatusCode;

/// API error format
#[derive(Debug, Default)]
pub struct Error {
    /// Error code
    pub code: StatusCode,
    /// Error message
    pub message: String,
    /// Error details
    pub details: String,
}

#[derive(Debug, Default, Serialize)]
struct Serializable {
    pub code: u16,
    pub message: String,
    pub details: String,
}

impl From<Error> for Response {
    fn from(error: Error) -> Self {
        let response = error.into();

        Response { response }
    }
}

impl From<Error> for hyper::Response {
    fn from(error: Error) -> Self {
        let serialized = serde_json::to_vec(&Serializable {
            code: error.code.as_u16(),
            message: error.message,
            details: error.details,
        })
        .expect("The serialization is infallible; qed");

        hyper::Response::new()
            .with_status(error.code)
            .with_header(header::ContentType::json())
            .with_body(serialized)
    }
}

impl Error {
    /// Internal Server Error
    pub fn internal<A: Into<String>, B: Into<String>>(message: A, details: B) -> Self {
        Error {
            code: StatusCode::InternalServerError,
            message: message.into(),
            details: details.into(),
        }
    }

    /// Generate 404 not found error.
    pub fn not_found<T: Into<String>>(details: T) -> Self {
        Error {
            code: StatusCode::NotFound,
            message: "Requested resource was not found.".to_owned(),
            details: details.into(),
        }
    }

    /// Generate 400 bad request error.
    pub fn bad_request<A: Into<String>, B: Into<String>>(message: A, details: B) -> Self {
        Error {
            code: StatusCode::BadRequest,
            message: message.into(),
            details: details.into(),
        }
    }

    /// Generate 405 method not allowed error.
    pub fn method_not_allowed<A: Into<String>, B: Into<String>>(message: A, details: B) -> Self {
        Error {
            code: StatusCode::MethodNotAllowed,
            message: message.into(),
            details: details.into(),
        }
    }
}
