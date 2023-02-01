use enigo::Enigo;
use once_cell::sync::Lazy;
use sysinfo::{System, SystemExt};

pub static mut SYS: Lazy<System> = Lazy::new(||{System::new_all()});

pub fn current_mouse_location() -> (i32, i32) {
    Enigo::mouse_location()
}

pub fn current_display_size() -> (usize, usize) {
    Enigo::main_display_size()
}
