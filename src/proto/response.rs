use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use hyper::header::Header;
use super::{Headers, StatusCode};
use error::{Error, Result};

pub use hyper::Response as HyperResponse;

/// # Response
///
/// `Response` is similar to `HyperResponse`, but it provides useful interfaces
/// for convenience.
#[derive(Debug)]
pub struct Response {
    status: StatusCode,
    headers: Headers,
    body: Vec<u8>,
}
impl Response {
    /// Create a new `Response` instance.
    pub fn new() -> Self {
        Response {
            status: StatusCode::Ok,
            headers: Headers::new(),
            body: Vec::new(),
        }
    }
    /// Get the status code.
    pub fn status(&self) -> StatusCode {
        self.status.clone()
    }
    /// Get the status code.
    pub fn header<H: Header>(&self) -> Option<&H> {
        self.headers.get::<H>()
    }
    /// Get the status code.
    pub fn body(&self) -> &[u8] {
        &self.body
    }
    
    /// Get a reference of to request body parsed as `str`. See documentation of
    /// `body()` for more information.
    pub fn to_str(&self) -> Result<&str> {
        ::std::str::from_utf8(&self.body)
            .map_err(|e| Error::internal("Unable to parse body as string.").with_cause(e))
    }
    /// Deserialize the body into a structure, or a generic container, only if
    /// the `Content-Type` is of type `application/json`.
    pub fn to_json<T: 'static + DeserializeOwned>(&self) -> Result<T> {
        use hyper::header::ContentType;
        if let Some(&ContentType(ref ty)) = self.headers.get::<ContentType>() {
            if ty.type_() != "application" || ty.subtype() != "json" {
                let err = Error::bad_request("Content should be of type `application/json`.");
                return Err(err)
            }
        }
        ::serde_json::from_slice::<T>(&self.body)
            .map_err(|e| Error::internal("Unable to deserialize body as JSON.").with_cause(e))
    }
    /// Deserialize the body into a structure, or a generic container, only if
    /// the `Content-Type` is of type `application/x-www-form-urlencoded`.
    pub fn to_form<'de, T: Deserialize<'de>>(&'de self) -> Result<T> {
        use hyper::header::ContentType;
        if let Some(&ContentType(ref ty)) = self.headers.get::<ContentType>() {
            if ty.type_() != "application" || ty.subtype() != "x-www-form-urlencoded" {
                let err = Error::bad_request("Content should be of type `application/x-www-form-urlencoded`.");
                return Err(err)
            }
        }
        ::serde_qs::from_bytes(&self.body)
            .map_err(|e| Error::internal("Unable to parse body as form data.").with_cause(e))
    }

    /// Set a status code.
    pub fn set_status(&mut self, status: StatusCode) {
        self.status = status;
    }
    /// Set a specific header.
    pub fn set_header<H: Header>(&mut self, header: H) {
        self.headers.set(header);
    }
    /// Set all the headers.
    pub fn set_headers(&mut self, headers: Headers) {
        self.headers = headers;
    }
    /// Set response content.
    pub fn set_body<B>(&mut self, body: B) where B: Into<Vec<u8>> {
        self.body = body.into();
    }
    /// Set response content serialized from json. The content type will be set
    /// to `application/json`.
    pub fn set_json<T: Serialize>(&mut self, json: &T) -> Result<()> {
        use hyper::header::ContentType;
        ::serde_json::to_vec(&json)
            .map(|json| {
                self.body = json;
                self.headers.set(ContentType("application/json; charset=UTF-8".parse().unwrap()));
            })
            .map_err(|err| Error::internal("Unable to serialize data into JSON.").with_cause(err))
    }

    /// Set a status code. Useful for builder pattern.
    pub fn with_status(mut self, status: StatusCode) -> Self {
        self.status = status;
        self
    }
    /// Set a specific header. Useful for builder pattern.
    pub fn with_header<H: Header>(mut self, header: H) -> Self {
        self.headers.set(header);
        self
    }
    /// Set all the headers. Useful for builder pattern.
    pub fn with_headers(mut self, headers: Headers) -> Self {
        self.headers = headers;
        self
    }
    /// Set response content. Useful for builder pattern.
    pub fn with_body<B>(mut self, body: B) -> Self
        where B: Into<Vec<u8>> {
        self.body = body.into();
        self
    }
    /// Set response content serialized from json. Useful for builder pattern.
    /// The content type will be set to `application/json`.
    pub fn with_json<T: Serialize>(mut self, json: &T) -> Result<Self> {
        self.set_json(json)
            .map(|_| { self })
    }
}
impl From<Response> for HyperResponse {
    fn from(res: Response) -> HyperResponse {
        let Response { status, headers, body } = res;
        HyperResponse::new()
            .with_status(status)
            .with_headers(headers)
            .with_body(body)
            // TODO: Update according to the version in request later?
    }
}
