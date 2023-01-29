use enigo::*;
use std::{thread, time};

pub fn start_dandelion_gather() {
    let mut enigo = Enigo::new();

    enigo.mouse_move_to(785, 840);
    thread::sleep(time::Duration::from_millis(1000));
    enigo.mouse_click(MouseButton::Left);
    thread::sleep(time::Duration::from_millis(1000));
    enigo.mouse_move_to(785, 775);
    thread::sleep(time::Duration::from_millis(1000));
    enigo.mouse_click(MouseButton::Left);
    thread::sleep(time::Duration::from_millis(1000));
    enigo.key_sequence_parse(" hello nigga");
    thread::sleep(time::Duration::from_millis(1000));
    enigo.key_click(Key::Return);
    println!("Test")
}