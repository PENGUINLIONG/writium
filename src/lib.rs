//! Writium Framework is the foundation of my blog generator Writium, and is
//! separated from the generator project.  
//! It's a clean framework providing fundamental functionalities we always use.
//!
//! # Why Writium Framework?
//!
//! Writium Framework is not so versatile but it does its best to fulfill most
//! of your needs, if *parts of* your web apps requires:
//!
//! * JSON ser/de;
//! * chunk-based (rather than stream-based) interaction;
//! * separation of duties;
//! * hierarchic organization.
//!
//! Writium Framework works well with all web frameworks which can provide
//! `HyperRequest`s and accept `HyperResponse`s, but itself is not a server
//! to-go. It might bring you a few more codes to write, but such design allows
//! you to separate the web engine and your API logics perfectly; it brings you
//! flexibility you always want.
//!
//! For example, after finishing your RESTful API, and you find you have to
//! write something stream-based. Then you can add it to somewhere in your same
//! application; you don't need to port codes to another web framework simply
//! because it doesn't support stream-based interaction.
pub extern crate futures;
pub extern crate hyper;
#[macro_use]
extern crate log;
extern crate serde;
extern crate serde_qs;
extern crate serde_json;

// Writium.
mod writium;

pub use writium::Writium;

// Api and namespace.
pub mod api;
pub mod namespace;

// Request flow protocol.
pub mod proto;

// Error handling.
pub mod error;

// Prelude.
pub mod prelude;
