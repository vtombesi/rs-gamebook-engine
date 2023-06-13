use crate::item::{Item};

pub struct Inventory {
    pub items: Vec<Item>,
}

impl Inventory {
    pub fn new() -> Self {
        Inventory { items: Vec::new() }
    }

    pub fn show(&self) {
        for (index, item) in self.items.iter().enumerate() {
            println!("{}. {}", index + 1, item);
        }
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
        println!("You have picked up the item {}.", item.name);
    }
}
