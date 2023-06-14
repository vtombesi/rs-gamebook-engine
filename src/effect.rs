use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Effect {
    pub stat: StatType,
    pub value: i32,
}

impl Effect {
    pub fn format_effect(&self) -> String {
        let sign = if self.value >= 0 { "+" } else { "-" };
        if sign == "+" {
            format!("\x1b[32m{} {}{}\x1b[0m", self.stat, sign, self.value.abs())
        } else if sign == "-" {
            format!("\x1b[31m{} {}{}\x1b[0m", self.stat, sign, self.value.abs())
        } else {
            format!("{} {}{}", self.stat, sign, self.value.abs())
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum StatType {
    Strength,
    Agility,
    Spirit,
    Luck,
}

impl fmt::Display for StatType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StatType::Strength => write!(f, "Strength"),
            StatType::Agility => write!(f, "Agility"),
            StatType::Spirit => write!(f, "Spirit"),
            StatType::Luck => write!(f, "Luck"),
        }
    }
}


