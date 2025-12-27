#![forbid(unsafe_code)]

pub mod app;
pub mod components;
pub mod routes;
pub mod server;

pub use app::App;
pub use server::{login, grok_analyze, ai_analyze};
