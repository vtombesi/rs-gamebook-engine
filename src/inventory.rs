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

    pub fn use_item(&mut self, item: Item) {
        let item_position = self.items.iter().position(|i| i.name == item.name && i.item_type == item.item_type);

        if let Some(index) = item_position {
            // Get a mutable reference to the item
            let item_in_inventory = &mut self.items[index];

            // Check the quantity and either decrement it or remove the item
            if item_in_inventory.quantity > 1 {
                item_in_inventory.quantity -= 1;
                println!("You have used one {}.", item.name);
            } else {
                self.items.remove(index);
                println!("You have used the last {}.", item.name);
            }
        } else {
            println!("The item {} is not in the inventory.", item.name);
        }
    }

    pub fn drop_item(&mut self, item: Item) {
        let item_position = self.items.iter().position(|i| i.name == item.name && i.item_type == item.item_type);

        if let Some(index) = item_position {
            // Get a mutable reference to the item
            let item_in_inventory = &mut self.items[index];

            // Check the quantity and either decrement it or remove the item
            if item_in_inventory.quantity > 1 {
                item_in_inventory.quantity -= 1;
                println!("You dropped one {}.", item.name);
            } else {
                self.items.remove(index);
                println!("You dropped the last {}.", item.name);
            }
        } else {
            println!("The item {} is not in the inventory.", item.name);
        }
    }

    pub fn pickup_item(&mut self, item: Item) {
        // Cerca se l'oggetto è già presente
        if let Some(existing_item) = self.items.iter_mut().find(|i| i.name == item.name && i.item_type == item.item_type) {
            // Se esiste, incrementa la sua quantità
            existing_item.quantity += item.quantity;
        } else {
            // Altrimenti, aggiungi l'oggetto all'inventario
            self.items.push(item.clone());
        }

        if item.quantity == 1 {
            logger::log_loot_item(format!("You have picked up a {}.", item.name));
        } else {
            logger::log_loot_item(format!("You have picked up some {}s(x{}).", item.name, item.quantity));
        }
    }

    pub fn get_item(&mut self, item_index: usize) -> Option<&mut Item> {
        self.items.get_mut(item_index)
    }

}
