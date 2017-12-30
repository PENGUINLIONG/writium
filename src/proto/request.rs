use std::any::Any;
use std::collections::BTreeMap;
use serde::Serialize;
use serde::de::{Deserialize, DeserializeOwned};
use hyper::header::Header;
use super::{Headers, Method, Uri};
use error::{Result, Error};

pub use hyper::Request as HyperRequest;

/// Wrapped HTTP protocol items (request, response and message components) for
/// common needs in web service development.
///
/// # Request
///
/// Unlike `HyperRequest`, `Request` contains a body of buffer rather than a
/// async Stream. That's what makes it unsuitable for stream-based
/// interactions. But the simplified representation of request allows us to
/// write concise codes, before we can return traits without boxing.
///
/// ## Extra
///
/// Writium Framework allows low hierarchy to provide data for high hierarchy
/// via extras, so that boilerplates can be saved. For example, the API
/// `[books]` parses the second segment of `/books/978-1-107-63682-8/content`
/// into ISBN, maps the ISBN to an object, then passes the reference of the
/// data model of that book to `[content] sub-API`.
///
/// When a piece of data is put into a request, it cannot be withdrawed. API
/// implementations should stay sane using extras to prevent unnecessary use of
/// resources.
#[derive(Debug)]
pub struct Request {
    pub(crate) method: Method,
    pub(crate) query: String,
    pub(crate) path_segs: Vec<String>,
    pub(crate) headers: Headers,
    pub(crate) body: Vec<u8>,
    pub(crate) extra: BTreeMap<String, Box<Any>>,
}
impl Request {
    pub fn new(method: Method) -> Request {
        Request {
            method: method, 
            query: String::new(),
            path_segs: Vec::new(),
            headers: Headers::new(),
            body: Vec::new(),
            extra: BTreeMap::new(),
        }
    }
    /// Get the HTTP method of the current request.
    pub fn method(&self) -> Method {
        self.method.clone()
    }
    /// Take the query part of URI and deserialize it into a structure, or a
    /// generic container.
    pub fn to_param<'de, T: Deserialize<'de>>(&'de self) -> Result<T> {
        ::serde_qs::from_str(&self.query)
            .map_err(|e| Error::internal("Unable to deserialize URI query.").with_cause(e))
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

    /// Set path segments and query string using given URI.
    pub fn set_uri(&mut self, uri: &Uri) {
        fn collect_path_segs(path: &str) -> Vec<String> {
            let mut raw_rv = Vec::new();
            if path.len() != 0 && path.as_bytes()[0] == b'/' {
                let chars = if cfg!(windows) {
                    path[1..].replace("\\", "/")
                } else {
                    path[1..].to_owned()
                };
                let iter = chars.split('/')
                        .map(|x| x.to_string());
                raw_rv.extend(iter);
            }
            // Protection for path traversal attack.
            let mut rv = Vec::new();
            for seg in raw_rv {
                if seg == ".." {
                    rv.pop();
                } else if seg == "." {
                } else {
                    rv.push(seg);
                }
            }
            rv
        }
        self.path_segs = collect_path_segs(uri.path());
        self.query = uri.query().unwrap_or_default().to_owned();
    }
    /// Set query string.
    pub fn set_query(&mut self, query: &str) {
        self.query = query.to_owned();
    }
    /// Set path segments.
    pub fn set_path_segs(&mut self, path_segs: &[&str]) {
        self.path_segs = path_segs.into_iter()
            .map(|x| (*x).to_owned())
            .collect();
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

    /// Set path segments and query string using given URI.
    pub fn with_uri(mut self, uri: &Uri) -> Self {
        self.set_uri(uri);
        self
    }
    /// Set query string.
    pub fn with_query(mut self, query: &str) -> Self {
        self.query = query.to_owned();
        self
    }
    /// Set path segments.
    pub fn with_path_segs(mut self, path_segs: &[&str]) -> Self {
        self.set_path_segs(path_segs);
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

    /// Extra data derived by lower path hierarchy of APIs.
    pub fn extra<T: 'static>(&self, key: &str) -> Option<&T> {
        self.extra.get(key)
            .and_then(|boxed| boxed.downcast_ref())
    }
    /// Put extra data into the current request.
    pub fn set_extra<T: 'static + Any + Sized>(&mut self, key: &str, val: T) {
        self.extra.insert(key.to_string(), Box::new(val));
    }

    /// Get the reference to internal path segment record.
    pub fn path_segs(&self) -> &[String] {
        &self.path_segs[..]
    }

    /// Match a segment of path. If the preceding segment in the current request
    /// is matching the given segment string, the segment is removed from
    /// internal record and the matched segment is then returned. Otherwise,
    /// `false` is returned.
    pub fn match_seg(&mut self, seg: &str) -> bool {
        if self.path_segs.len() > 0 &&
            &self.path_segs[0] == seg {
            self.path_segs.remove(0);
            true
        } else {
            false
        }
    }
    /// Match several segments of path. It matches when and only when all the
    /// segments are matching. See `match_segs()`.
    pub fn match_segs(&mut self, segs: &[&str]) -> bool {
        // The incoming segments should be fewer than the segments the current
        // request can provide.
        if self.path_segs.len() < segs.len() {
            return false
        }
        if self.path_segs.iter()
            .zip(segs.iter())
            .all(|(req_seg, api_seg)| req_seg == api_seg) {
            self.path_segs.drain(..segs.len());
            true
        } else {
            false
        }
    }
    /// Match a segment of path. If the preceding segment in the current request
    /// is matching the given condition, the segment is removed from internal
    /// record and the matched segment is then returned. Otherwise, `None` is
    /// returned.
    pub fn match_seg_if<F>(&mut self, pred: F) -> Option<String>
        where F: Fn(&str) -> bool {
        if self.path_segs.len() > 0 && pred(&self.path_segs[0]) {
            Some(self.path_segs.remove(0))
        } else {
            None
        }
    }
    /// Match all the segments of path while the given condition is satisfied.
    /// See `match_seg()`.
    pub fn match_segs_while<F>(&mut self, pred: F) -> Vec<String>
        where F: Fn(&str) -> bool {
        use std::iter::FromIterator;
        if let Some(false_point) = self.path_segs.iter()
            .position(|x| !pred(x)) {
            Vec::from_iter(self.path_segs.drain(..false_point))
        } else {
            Vec::from_iter(self.path_segs.drain(..))
        }
    }
}
