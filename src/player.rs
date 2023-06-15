use std::mem;

use crate::inventory::Inventory;
use crate::item::Item;
use crate::equipment::Equipment;
use crate::stats::Stats;
use serde::{Deserialize};

#[derive(Debug, Deserialize, Clone)]
pub struct Player {
    pub health: i32,
    pub inventory: Inventory,
    pub stats: Stats,
    pub equipment: Equipment,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PlayerImportData {
    pub stats: Stats,
    pub equipment: Equipment,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            health: 20,
            inventory: Inventory::new(),
            stats: Stats::new(10, 10, 10, 10),
            equipment: Equipment::new(),
        }
    }
}

impl Player {
    pub fn use_item(&mut self, item: &Item) {
        self.inventory.use_item(item.clone());
    }

    pub fn pickup_item(&mut self, item: &Item) {
        self.inventory.pickup_item(item.clone());
    }

    pub fn drop_item(&mut self, item: &Item) {
        self.inventory.drop_item(item.clone());
    }

    pub fn equip_item(&mut self, item: &Item) {
        self.equipment.equip_item(item.clone());
        self.inventory.drop_item(item.clone());
    }

    pub fn unequip_item(&mut self, item: Item) {
        self.equipment.unequip_item(item.clone().item_type);
        self.inventory.pickup_item(item.clone());
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    pub fn attack(&self) -> i32 {
        let base_attack = 5; // Valore di attacco di base
        let complete_stats = self.clone().get_complete_stats();
        let strength_modifier = complete_stats.strength;

        let reference_average = 10; // Media di riferimento
        let modifier_scale = 1; // Scala del modificatore (bonus/malus ogni tre punti)

        let difference = strength_modifier - reference_average;
        let scaled_modifier = (difference / 3) * modifier_scale;

        let modified_attack = base_attack + scaled_modifier;

        modified_attack
    }

    pub fn heal(&mut self, amount: i32) {
        self.health += amount;
    }

    pub fn get_complete_stats(&mut self) -> Stats {
        let mut complete_stats = self.stats.clone();

        // Applica eventuali modificatori di equipaggiamento alle statistiche complete
        complete_stats.apply_equipment_modifiers(&self.equipment);

        complete_stats
    }
}
