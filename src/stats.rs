use serde::{Deserialize};
use crate::effect::StatType;
use crate::equipment::Equipment;

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

    pub fn apply_equipment_modifiers(&mut self, equipment: &Equipment) {
        for item in equipment.get_equipped_items() {
            if let Some(effect) = item.effect {
                match effect.stat {
                    StatType::Strength => self.strength += effect.value,
                    StatType::Agility => self.agility += effect.value,
                    StatType::Spirit => self.spirit += effect.value,
                    StatType::Luck => self.luck += effect.value,
                }
            }
        }
    }
}
