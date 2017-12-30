use prelude::*;

pub type ApiResult = Result<Response>;

/// # Writium API
///
/// `Api` is fundamental for all kinds of server behaviors to be achieved.
/// Writium doesn't restrict the use of certain HTTP methods, which allows you
/// to shape your API as freely as possible.
pub trait Api: 'static + Send + Sync {
    /// Name of API. It identifies an API and allow Writium to route by URL path
    /// segments.
    ///
    /// # Tricks
    ///
    /// If the name of an API is an empty slice, it can work as a "fuse" - the
    /// unnamed API will always be called. All request-routing will be short-
    /// circuited at this point.
    ///
    /// When it's placed at the end of a route chain (of a `Namespace`), the
    /// chain is then terminated; no APIs can be 'effectively' binded to that
    /// namespace, i.e., the subsequently binded APIs will never be routed. The
    /// `terminator` can provide some useful information for API users playing
    /// with curl.
    fn name(&self) -> &[&str];

    /// Route incoming request to the next level. If it's an end-point, the
    /// request is processed and a result is returned.
    fn route(&self, req: &mut Request) -> ApiResult;
}
