use enigo::*;
use std::{thread, time};
use crate::config::CapConfig;

pub fn start_dandelion_gather(config: CapConfig) {
    let mut enigo = Enigo::new();

    enigo.mouse_move_to(950, 840);
    thread::sleep(time::Duration::from_millis(1000));
    enigo.mouse_click(MouseButton::Left);
    thread::sleep(time::Duration::from_millis(1000));
    enigo.key_click(Key::Escape);
    enigo.key_click(Key::Layout('r'));
    thread::sleep(time::Duration::from_millis(800));
    enigo.key_click(Key::Return);

    println!("Test")
}