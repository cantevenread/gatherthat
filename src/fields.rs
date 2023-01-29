use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::config::CapConfig;
use crate::fields::FieldType::{BlueFlower, Cactus, Clover, Coconut, Dandelion, MountainTop, Mushroom, Pepper, Pine, Pumpkin, Rose, Spider, Strawberry, Stump, Sunflower};
use crate::fields::HiveSlots::{HiveSlot1, HiveSlot2, HiveSlot3, HiveSlot4, HiveSlot5, HiveSlot6};

mod dandelion;
mod sunflower;


#[derive(Debug)]
pub struct FieldGathererError {
    pub details: String,
}

impl Display for FieldGathererError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl FieldGathererError {
    pub fn new(msg: &str) -> FieldGathererError {
        FieldGathererError{details: msg.to_string()}
    }
}

impl Error for FieldGathererError {
    fn description(&self) -> &str {
        &self.details
    }
}

#[derive(Debug)]
pub struct InvalidInfoGiven {
    pub details: String,
}

impl Display for InvalidInfoGiven {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl InvalidInfoGiven {
    pub fn new(msg: &str) -> InvalidInfoGiven {
        InvalidInfoGiven{details: msg.to_string()}
    }
}

impl Error for InvalidInfoGiven {
    fn description(&self) -> &str {
        &self.details
    }
}


#[derive(Debug)]
pub enum HiveSlots {
    HiveSlot1,
    HiveSlot2,
    HiveSlot3,
    HiveSlot4,
    HiveSlot5,
    HiveSlot6,

}


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
            "Dandelion" => Ok(Dandelion),
            "Sunflower" => Ok(Sunflower),
            "Spider" => Ok(Spider),
            "Pepper" => Ok(Pepper),
            "Strawberry" => Ok(Strawberry),
            "Stump" => Ok(Stump),
            "Mushroom" => Ok(Mushroom),
            "Clover" => Ok(Clover),
            "Rose" => Ok(Rose),
            "BlueFlower" => Ok(BlueFlower),
            "Cactus" => Ok(Cactus),
            "Pumpkin" => Ok(Pumpkin),
            "Pine" => Ok(Pine),
            "MountainTop" => Ok(MountainTop),
            "Coconut" => Ok(Coconut),
            _ => Err(()),
        }
    }
}

impl std::str::FromStr for HiveSlots {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(HiveSlot1),
            "2" => Ok(HiveSlot2),
            "3" => Ok(HiveSlot3),
            "4" => Ok(HiveSlot4),
            "5" => Ok(HiveSlot5),
            "6" => Ok(HiveSlot6),
            _ => Err(())
        }
    }
}

pub fn start_field(field: &FieldType, config: &CapConfig) -> Result<(), FieldGathererError> {
    match field {
        Dandelion => Ok(dandelion::start_dandelion_gather(config)),
        _ => Err(FieldGathererError { details: "Field Doesn't Exist".to_string() })
    }
}

// "Dandelion" => Ok(Dandelion),
// "Sunflower" => Ok(Sunflower),
// "Spider" => Ok(Spider),
// "Pepper" => Ok(Pepper),
// "Strawberry" => Ok(Strawberry),
// "Stump" => Ok(Stump),
// "Mushroom" => Ok(Mushroom),
// "Clover" => Ok(Clover),
// "Rose" => Ok(Rose),
// "BlueFlower" => Ok(BlueFlower),
// "Cactus" => Ok(Cactus),
// "Pumpkin" => Ok(Pumpkin),
// "Pine" => Ok(Pine),
// "MountainTop" => Ok(MountainTop),
// "Coconut" => Ok(Coconut),
