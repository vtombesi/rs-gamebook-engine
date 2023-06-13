use crate::item::{Item, ItemType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum EquipmentSlot {
    Armour,
    Weapon,
    Shield,
    Necklace,
    Ring,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Equipment {
    pub armour: Option<Item>,
    pub weapon: Option<Item>,
    pub shield: Option<Item>,
    pub necklace: Option<Item>,
    pub ring: Option<Item>,
}

impl Equipment {
    pub fn new() -> Self {
        Equipment {
            armour: None,
            weapon: None,
            shield: None,
            necklace: None,
            ring: None,
        }
    }

    pub fn equip_item(&mut self, item: Item) -> Option<Item> {
        println!("Inizio equip_item con item: {:?}", item); // Mostra l'item all'inizio della funzione

        let slot = match item.item_type {
            ItemType::Armour => {
                println!("item è Armour");
                &mut self.armour
            }
            ItemType::Weapon => {
                println!("item è Weapon");
                &mut self.weapon
            }
            ItemType::Shield => {
                println!("item è Shield");
                &mut self.shield
            }
            ItemType::Necklace => {
                println!("item è Necklace");
                &mut self.necklace
            }
            ItemType::Ring => {
                println!("item è Ring");
                &mut self.ring
            }
            _ => {
                println!("item non è un tipo conosciuto");
                return Some(item)
            }
        };

        if let Some(replaced_item) = slot.take() {
            println!("L'item sostituisce un item esistente: {:?}", replaced_item);
            *slot = Some(item);
            Some(replaced_item)
        } else {
            println!("L'item viene equipaggiato in uno slot vuoto");
            *slot = Some(item);
            None
        }
    }


    pub fn remove_item(&mut self, item_type: ItemType) -> Option<Item> {
        let slot = match item_type {
            ItemType::Armour => &mut self.armour,
            ItemType::Weapon => &mut self.weapon,
            ItemType::Shield => &mut self.shield,
            ItemType::Necklace => &mut self.necklace,
            ItemType::Ring => &mut self.ring,
            _ => return None,
        };

        slot.take()
    }

    pub fn get_equipped_items(&self) -> Vec<Item> {
        let mut equipped_items = Vec::new();

        if let Some(armour) = &self.armour {
            equipped_items.push(armour.clone());
        }
        if let Some(weapon) = &self.weapon {
            equipped_items.push(weapon.clone());
        }
        if let Some(shield) = &self.shield {
            equipped_items.push(shield.clone());
        }
        if let Some(necklace) = &self.necklace {
            equipped_items.push(necklace.clone());
        }
        if let Some(ring) = &self.ring {
            equipped_items.push(ring.clone());
        }

        equipped_items
    }

    pub fn show(&self) {
        println!("--- Equipment ---");

        println!("[Armour] - {} - {:?}", self.armour.as_ref().map(|item| item.name.to_owned()).unwrap_or_else(|| "No armour equipped".to_owned()), self.armour.as_ref().map(|item| &item.effect));
        println!("[Weapon] - {} - {:?}", self.weapon.as_ref().map(|item| item.name.to_owned()).unwrap_or_else(|| "No weapon equipped".to_owned()), self.weapon.as_ref().map(|item| &item.effect));
        println!("[Shield] - {} - {:?}", self.shield.as_ref().map(|item| item.name.to_owned()).unwrap_or_else(|| "No shield equipped".to_owned()), self.shield.as_ref().map(|item| &item.effect));
        println!("[Ring] - {} - {:?}", self.ring.as_ref().map(|item| item.name.to_owned()).unwrap_or_else(|| "No ring equipped".to_owned()), self.ring.as_ref().map(|item| &item.effect));
        println!("[Necklace] - {} - {:?}", self.necklace.as_ref().map(|item| item.name.to_owned()).unwrap_or_else(|| "No necklace equipped".to_owned()), self.necklace.as_ref().map(|item| &item.effect));

        println!("-----------------");
        println!();
    }
}
