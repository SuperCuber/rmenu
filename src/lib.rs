#[macro_use]
extern crate conrod;

mod gui;
mod config;
mod gui_result;
mod reasonable_main;

pub use gui::{run, run_config};
pub use config::Config;
pub use gui_result::GuiResult;
pub use reasonable_main::{reasonable_main, Command};
pub use conrod::color;
