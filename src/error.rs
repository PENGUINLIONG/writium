use std::fmt::{Display, Formatter, Result as FormatResult};
use proto::HyperResponse;
use prelude::*;
use self::header::Header;

/// Writium error.
///
/// # Error Handling
///
/// `Error` is another form of response in Writium Framework, i.e., it itself
/// can be transformed into a response. It provides (hopefully) useful
/// information about the error occured in the current transaction. The error
/// types are customizable. Error types are defined as HTTP status codes
/// (>= 400). Every error realized by Writium will be logged.
pub struct Error {
    headers: Headers,
    status: StatusCode,
    description: &'static str,
    cause: Option<Box<::std::error::Error>>,
}
impl Error {
    pub fn new(status: StatusCode, description: &'static str) -> Error {
        Error {
            status: status,
            headers: Headers::default(),
            description: description,
            cause: None,
        }
    }
    pub fn status(&self) -> StatusCode {
        self.status
    }

    pub fn internal(description: &'static str) -> Error {
        Error::new(StatusCode::InternalServerError, description)
    }
    pub fn unauthorized(description: &'static str) -> Error {
        Error::new(StatusCode::Unauthorized, description)
    }
    pub fn bad_request(description: &'static str) -> Error {
        Error::new(StatusCode::BadRequest, description)
    }
    pub fn forbidden(description: &'static str) -> Error {
        Error::new(StatusCode::Forbidden, description)
    }
    pub fn not_found(description: &'static str) -> Error {
        Error::new(StatusCode::NotFound, description)
    }
    pub fn method_not_allowed() -> Error {
        Error::new(StatusCode::NotFound, "Method is not allowed.")
    }

    pub fn set_header<H: Header>(&mut self, header: H) {
        self.headers.set(header);
    }
    pub fn set_headers(&mut self, headers: Headers) {
        self.headers = headers;
    }
    pub fn set_cause<E>(&mut self, err: E)
        where E: 'static + ::std::error::Error {
        self.cause = Some(Box::new(err));
    }
    

    pub fn with_header<H: Header>(mut self, header: H) -> Self {
        self.headers.set(header);
        self
    }
    pub fn with_headers(mut self, headers: Headers) -> Self {
        self.headers = headers;
        self
    }
    pub fn with_cause<E>(mut self, err: E) -> Self
        where E: 'static + ::std::error::Error {
        self.cause = Some(Box::new(err));
        self
    }
}
impl ::std::fmt::Debug for Error {
    fn fmt(&self, f: &mut Formatter) -> FormatResult {
        f.write_str(self.description)
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FormatResult {
        f.write_str(self.description)
    }
}
impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        self.description
    }
    fn cause(&self) -> Option<&::std::error::Error> {
        match self.cause {
            Some(ref e) => Some(&**e),
            None => ::std::option::Option::None,
        }
    }
}
impl Into<HyperResponse> for Error {
    fn into(self) -> HyperResponse {
        use hyper::header::{ContentType, ContentLength};
        let body = format!(r#"{{"msg":"{}"}}"#, self.description).into_bytes();
        HyperResponse::new()
            .with_status(self.status)
            .with_headers(self.headers)
            .with_header(ContentType("application/json; charset=UTF-8".parse().unwrap()))
            .with_header(ContentLength(body.len() as u64))
            .with_body(body)
    }
}

pub type Result<T> = ::std::result::Result<T, Error>;
