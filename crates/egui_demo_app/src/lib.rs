//! Demo app for egui
#![allow(clippy::missing_errors_doc)]

mod wrap_app;
mod ui;
mod demo;

pub use {
    ui::DemoWindows, wrap_app::WrapApp, demo::MiscDemoWindow,
};

// ----------------------------------------------------------------------------

#[cfg(target_arch = "wasm32")]
mod web;

#[cfg(target_arch = "wasm32")]
pub use web::*;
