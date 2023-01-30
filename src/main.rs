#![windows_subsystem = "console"]

use crate::manager::start_manager;

mod config;
mod fields;
mod screen;
mod manager;



// made by capthat

fn main() {
    start_manager();
}

