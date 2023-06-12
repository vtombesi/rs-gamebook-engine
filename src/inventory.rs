pub struct Inventory {
    pub items: Vec<String>,
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
            println!("Hai usato l'oggetto {}.", item_name);
            // Qui puoi aggiungere il codice per gestire l'effetto dell'oggetto
            Ok(())
        } else {
            Err(format!("L'indice dell'oggetto {} non è valido.", item_index))
        }
    }

    pub fn pickup(&mut self, item_name: String) {
        self.items.push(item_name.clone());
        println!("Hai raccolto l'oggetto {}.", item_name);
    }
}
