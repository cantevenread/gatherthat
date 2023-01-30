use crate::config::{CONFIG, CURRENT_FIELD};
use crate::fields::start_field;
use crate::manager::start_manager;

mod config;
mod fields;
mod screen;
mod manager;




fn main() {

    println!("{:#?}", &*config::CONFIG);

    println!("{:#?}", &*config::CURRENT_FIELD);

    println!("{:#?}", &*config::CURRENT_HIVE_SLOT);

    start_manager();

}

