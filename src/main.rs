#![windows_subsystem = "console"]

use crate::config::{CONFIG, CURRENT_FIELD};
use crate::manager::start_manager;

mod config;
mod fields;
mod screen;
mod manager;





fn main() {
    start_manager();
}

