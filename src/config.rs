
use serde::{Serialize, Deserialize};
use std::io::Error as IoError;
use std::fs;
use once_cell::sync::Lazy;
use std::clone::Clone;
use crate::fields::{FieldType, HiveSlots};

#[derive(Serialize, Deserialize, Debug)]
pub struct CapConfigStructure {
    pub info: Option<CapConfigInfo>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CapConfigInfo {
    pub bees: Option<String>,
    pub hive_slot: Option<String>,
    pub field: Option<String>
}

#[derive(Serialize, Deserialize,Debug)]
pub struct CapConfig {
    #[serde(default = "deafult_bees")]
    pub bees: String,
    #[serde(default = "deafult_hive_slot")]
    pub hive_slot: String,

    pub field: String
}

impl CapConfig {
    pub fn new() -> Self {
        let config_path: [&str; 2] = [
            "./Config.toml",
            "./config.toml"
        ];

        let mut content: String = "".to_owned();

        for path in config_path {
            let result: Result<String, IoError> = fs::read_to_string(path);

            if result.is_ok() {
                content = result.unwrap();
                break;
            }
        }

        let config_toml: CapConfigStructure = toml::from_str(&content).unwrap_or_else(|_| {
            println!("Failed to create ConfigToml Object out of config file.");
            CapConfigStructure{
                info: None
            }
        });

        let (bees, hive_slot, field): (String, String, String) = match config_toml.info {
            Some(info) => {
                let info_bees: String = info.bees.unwrap_or_else(|| {
                    println!("Missing field username in table database.");
                    "unknown".to_owned()

                });

                let info_hive_slot: String = info.hive_slot.unwrap_or_else(|| {
                    println!("Missing field username in table database.");
                    "unknown".to_owned()

                });

                let info_field: String = info.field.unwrap_or_else(|| {
                    println!("Missing field in table info.");
                    "unknown".to_owned()

                });

                (info_bees, info_hive_slot, info_field)
            },
            None => {
                println!("Missing table database.");
                ("unknown".to_owned(),"unknown".to_owned(),"unknown".to_owned())
            },


        };


        CapConfig {
            bees,
            field,
            hive_slot
        }
    }
}

fn deafult_bees() -> String {
    "35".to_string()
}
fn deafult_hive_slot() -> String {
    "1".to_string()
}

pub static CONFIG: Lazy<CapConfig> = Lazy::new(CapConfig::new);
pub static CURRENT_FIELD: Lazy<FieldType> = Lazy::new(|| CONFIG.field.clone().parse().expect("INVALID>FIELD"));
pub static CURRENT_HIVE_SLOT: Lazy<HiveSlots> = Lazy::new(|| CONFIG.hive_slot.clone().parse().expect("INVALID>HIVE_SLOT"));

// USE &* to get value