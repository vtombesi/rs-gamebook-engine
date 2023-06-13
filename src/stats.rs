use crate::inventory::Inventory;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Stats {
    strength: i32,
    agility: i32,
    spirit: i32,
    luck: i32,
}

impl Stats {
    pub fn new(strength: i32, agility: i32, spirit: i32, luck: i32) -> Self {
        Stats {
            strength,
            agility,
            spirit,
            luck,
        }
    }
}
