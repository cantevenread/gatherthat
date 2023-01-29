use crate::config::CapConfig;

mod dandelion;


#[derive(Debug)]
pub struct FieldGathererError {}


#[derive(Debug)]
pub enum FieldType {
    Sunflower,
    Dandelion,
    Spider,
    Pepper,
    Strawberry,
    Stump,
    Mushroom,
    Clover,
    Rose,
    BlueFlower,
    Cactus,
    Pumpkin,
    Pine,
    MountainTop,
    Coconut
}

impl std::str::FromStr for FieldType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Dandelion" => Ok(FieldType::Dandelion),
            "Sunflower" => Ok(FieldType::Sunflower),
            _ => Err(()),
        }
    }
}

pub fn start_field(field: FieldType, config: CapConfig) -> Result<(), FieldGathererError> {
    match field {
        FieldType::Dandelion => Ok(dandelion::start_dandelion_gather(config)),
        _ => Err(FieldGathererError {})
    }
}
