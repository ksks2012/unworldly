//! Demo app for egui
#![allow(clippy::missing_errors_doc)]

mod wrap_app;
mod ui;

pub use wrap_app::WrapApp;
pub use ui::*;

// ----------------------------------------------------------------------------

#[cfg(target_arch = "wasm32")]
mod web;

#[cfg(target_arch = "wasm32")]
pub use web::*;
