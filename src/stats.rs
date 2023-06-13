use serde::{Deserialize};

#[derive(Debug, Deserialize, Clone)]
pub struct Stats {
    pub strength: i32,
    pub agility: i32,
    pub spirit: i32,
    pub luck: i32,
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
