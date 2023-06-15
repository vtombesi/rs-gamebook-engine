use serde::{Serialize, Deserialize};
use std::fmt;

use crate::effect::Effect;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    pub name: String,
    #[serde(rename = "type")]
    pub item_type: ItemType,
    pub quantity: u32,
    pub effect: Option<Effect>,
}

impl Item {
    pub fn new(name: String, item_type: ItemType, quantity: u32, effect: Option<Effect>) -> Self {
        Item {
            name,
            item_type,
            quantity,
            effect,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ItemType {
    Usable,
    Armour,
    Weapon,
    Shield,
    Necklace,
    Ring,
    Quest,
    // Aggiungi qui altri tipi di oggetti se necessario
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Nome: {}, Tipo: {:?}, Quantità: {}, Effetto: {:?}",
               self.name,
               self.item_type,
               self.quantity,
               self.effect)
    }
}
