use enigo::Enigo;

pub fn current_mouse_location() -> (i32, i32) {
    Enigo::mouse_location()
}

pub fn current_display_size() -> (usize, usize) {
    Enigo::main_display_size()
}

