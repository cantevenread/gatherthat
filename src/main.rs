mod config;
mod fields;
mod screen;
mod manager;

use crate::config::CapConfig;
use crate::fields::FieldType;

fn main() {
    let config: CapConfig = CapConfig::new();
    let current_field: FieldType = config.field.clone().parse().unwrap();
    println!("{:#?}", config);

    println!("{:#?}", current_field);

    fields::start_field(current_field, config).expect("TODO: panic message");
}

