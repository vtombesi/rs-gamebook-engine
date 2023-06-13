use crate::item::{Item, ItemType};
use crate::logger;
use serde::{Deserialize};

#[derive(Debug, Deserialize, Clone)]
pub struct Inventory {
    pub items: Vec<Item>,
}

impl Inventory {
    pub fn new() -> Self {
        Inventory { items: Vec::new() }
    }

    pub fn show(&self) {
        println!("--- Inventory ---");

        if self.items.is_empty() {
            println!("No items in the inventory.");
        } else {
            for (index, item) in self.items.iter().enumerate() {
                let effect = item
                    .effect
                    .as_ref()
                    .map(|effect| effect.format_effect())
                    .unwrap_or_else(|| "No effect".to_owned());

                let action = match item.item_type {
                    ItemType::Armour | ItemType::Weapon | ItemType::Shield | ItemType::Ring | ItemType::Necklace => "Equip",
                    ItemType::Usable => "Use",
                    ItemType::Quest => "",
                };

                println!("{}. {} (x{}) - {} [{}]", index + 1, item.name, item.quantity, effect, action);
            }
        }

        println!("-----------------");
        println!();
    }

    pub fn use_item(&mut self, item_index: usize) -> Result<(), String> {
        if item_index < self.items.len() {
            let item_name = self.items.remove(item_index);
            println!("You have used the item {}.", item_name);
            // Here you can add the code to handle the effect of the item
            Ok(())
        } else {
            Err(format!("The item index {} is not valid.", item_index))
        }
    }

    pub fn pickup(&mut self, item: Item) {
        let cloned_item = item.clone(); // Clone the item object
        self.items.push(cloned_item);
        if item.quantity == 1 {
            logger::log_loot_item(format!("You have picked up a {}.", item.name));
        } else {
            logger::log_loot_item(format!("You have picked up some {}s(x{}).", item.name, item.quantity));
        }
    }
}
