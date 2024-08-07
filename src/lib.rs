#![allow(clippy::missing_errors_doc)]

mod app;

pub use app::MyApp;

#[cfg(target_arch = "wasm32")]
mod web;

#[cfg(target_arch = "wasm32")]
pub use web::*;
