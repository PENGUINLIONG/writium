//! This module includes most of the tools you want to use *during API*
//! *development*. In case you are a user consuming APIs made by others, you
//! generally need `Writium` only.

// Api and namespace implementation use.
pub use api::{Api, ApiResult};
pub use namespace::Namespace;

// Request and response.
pub use proto::{header, Request, Response, Headers, Method, StatusCode, Uri};

// Error handling.
pub use error::{Error, Result};
