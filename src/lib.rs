#[macro_use]
extern crate conrod;

mod gui;
mod config;

pub use gui::{run, run_config};
pub use config::Config;
