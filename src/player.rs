use crate::inventory::Inventory;
use crate::item::Item;

pub struct Player {
    pub health: i32,
    pub inventory: Inventory,
}

impl Player {
    pub fn new() -> Self {
        Player {
            health: 20,
            inventory: Inventory::new(),
        }
    }

    pub fn use_item(&mut self, item_index: usize) -> Result<(), String> {
        self.inventory.use_item(item_index)
            .map_err(|e| format!("Errore nell'uso dell'oggetto: {}", e))
        // Dopo aver utilizzato l'oggetto, potresti voler aggiungere effetti specifici dell'oggetto, come curare il giocatore
    }

    pub fn pickup(&mut self, item: Item) {
        self.inventory.pickup(item);
    }
    pub fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    pub fn attack(&self) -> i32 {
        // Logic for calculating player's attack damage
        // You can modify this as per your game rules
        5
    }

    pub fn heal(&mut self, amount: i32) {
        self.health += amount;
    }
}
