//! This crate contains all shared UI for the workspace.

mod views;

mod navbar;
pub use navbar::Route;
pub use navbar::App;

mod markdown;
pub use markdown::Markdown;

pub mod database;
pub use database::*;

pub mod extensions;
pub use extensions::*;
