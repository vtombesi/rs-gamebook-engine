use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Effect {
    pub stat: StatType,
    pub value: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum StatType {
    Strength,
    Agility,
    Spirit,
    Luck,
}







