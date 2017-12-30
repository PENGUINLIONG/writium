use std::sync::Arc;
use proto::{HyperRequest, HyperResponse};
use futures::Future;
use prelude::*;

/// The element Writium.
///
/// Writium holds all the APIs and transform `hyper` data types into what
/// Writium APIs can utilize. Itself can be considered a `Namespace` interfacing
/// `hyper`-variant frameworks.
pub struct Writium {
    ns: Arc<Namespace>,
}
impl Writium {
    /// Create a new instance of `Writium`.
    pub fn new() -> Writium {
        Writium {
            ns: Arc::new(Namespace::new(&[])),
        }
    }

    /// Route a `HyperRequest` to target API and return a `Future` of
    /// `HyperResponse`.
    pub fn route(&self, req: HyperRequest)
        -> Box<Future<Item=HyperResponse, Error=::hyper::Error>> {

        // TODO: Deal with that path segments are not parsed.
        let (method, uri, _version, headers, body) = req.deconstruct();
        use futures::Stream;
        let ns = self.ns.clone();
        let f_res = body
            .concat2()
            .map(move |body| {
                let mut req = Request::new(method);
                req.set_headers(headers);
                req.set_body(body.to_vec());
                req.set_uri(&uri);
                // No need to check namespace name. Safe to route
                // directly.
                match ns.route(&mut req) {
                    Ok(res) => res.into(),
                    Err(err) => {
                        // Log if error occurred.
                        let mut log = Vec::<String>::new();
                        if err.status().is_server_error() {
                            log.push(format!("Unexpected error occured: {}", err));
                            let mut err: &::std::error::Error = &err;
                            loop {
                                if let Some(cause) = err.cause() {
                                    log.push(format!("\tBy: {}", cause));
                                    err = cause;
                                } else {
                                    break
                                }
                            }
                            warn!("{}", log.join("\n"));
                        } else if err.status().is_client_error() {
                            warn!("Bad request induced an error: {}", err);
                        }
                        err.into()
                    },
                }
            });
        Box::new(f_res)
    }

    /// Bind an API to the root namespace. See `Namespace`'s `bind()` for more
    /// information.
    pub fn bind<A: Api + 'static>(&mut self, api: A) {
        Arc::make_mut(&mut self.ns).bind(api)
    }
}
