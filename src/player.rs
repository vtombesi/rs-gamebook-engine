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
    // pub fn new() -> Self {
    //     Player {
    //         health: 20,
    //         inventory: Inventory::new(),
    //         stats: Stats::new(10, 10, 10, 10),
    //         equipment: Equipment::new(),
    //     }
    // }

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
    }

    pub fn unequip_item(&mut self, item: Item) {
        self.equipment.unequip_item(item.item_type);
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    pub fn attack(&self) -> i32 {
        5
    }

    pub fn heal(&mut self, amount: i32) {
        self.health += amount;
    }
}
