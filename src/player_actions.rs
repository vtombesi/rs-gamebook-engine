use crate::{
    gamebook::{Choice, GameBook},
    item::ItemType,
    logger,
    utils::read_user_input,
};

pub fn handle_equipment(gamebook: &mut GameBook) {
    fn update_screen(gamebook: &mut GameBook) {
        print!("{}[2J", 27 as char);
        gamebook.player.equipment.show();
        println!("Select an option to continue:");
        logger::log_choice("(I)nventory - Re(T)urn to game");
    }

    loop {
        update_screen(gamebook);

        let equipment_input = read_user_input();

        match equipment_input.trim().to_uppercase().as_str() {
            "T" => {
                return;
            }
            "I" => {
                update_screen(gamebook);
            }
            _ => {
                println!("Invalid input. Please try again.");
            }
        }
    }
}

pub fn handle_inventory(gamebook: &mut GameBook) {
    fn update_screen(gamebook: &mut GameBook) {
        print!("{}[2J", 27 as char);
        gamebook.player.inventory.show();
        println!("Select an item to use or an option to continue:");
        logger::log_choice("(E)quipment - Re(T)urn to game");
    }

    loop {
        update_screen(gamebook);

        let inventory_input = read_user_input();

        if inventory_input.trim().to_uppercase() == "T" {
            // Return to the game
            break;
        } else if inventory_input.trim().to_uppercase() == "E" {
            update_screen(gamebook);
            break;
        } else if let Ok(item_choice) = inventory_input.trim().parse::<usize>() {
            // Try to use the chosen item

            if let Some(item) = gamebook.player.inventory.get_item(item_choice - 1).cloned() {
                println!("Item: {} - {}", item, item_choice);

                if let ItemType::Quest = item.item_type {
                    // do nothing
                } else if let ItemType::Usable = item.item_type {
                    let _ = gamebook.player.use_item(&item);
                } else {
                    gamebook.player.equip_item(&item);
                }

                update_screen(gamebook);
            }
        } else {
            println!("Invalid input. Please try again.");
        }
    }
}

pub fn handle_character(gamebook: &mut GameBook) {
    fn update_screen(gamebook: &mut GameBook) {
        print!("{}[2J", 27 as char);

        // Mostra le statistiche complete del personaggio
        let complete_stats = gamebook.player.get_complete_stats();
        println!("--- Character Info ---");
        println!("Name: {}", "Oberon");
        println!("Race: {}", "Human");
        println!("Class: {}", "Wizard");
        println!("----------------------");
        println!();

        println!("--- Character Stats ---");
        println!("Strength: {}", complete_stats.strength);
        println!("Agility: {}", complete_stats.agility);
        println!("Spirit: {}", complete_stats.spirit);
        println!("Luck: {}", complete_stats.luck);
        println!("----------------------");
        println!();

        println!("Select an option to continue:");
        logger::log_choice("(I)nventory - (E)quipment - Re(T)urn to game");

    }

    loop {
        update_screen(gamebook);

        let equipment_input = read_user_input();

        match equipment_input.trim().to_uppercase().as_str() {
            "T" => {
                return;
            }
            "I" => {
                update_screen(gamebook);
            }
            "E" => {
                handle_equipment(gamebook);
            }
            _ => {
                println!("Invalid input. Please try again.");
            }
        }
    }
}


pub fn show_options(options: &Vec<Choice>) {
    // Print the available options
    for (index, option) in options.iter().enumerate() {
        logger::log_choice(format!("{}. {}", index + 1, option.text));
    }

    // Print global options
    println!();
    logger::log_choice("(I)nventory - (E)quipment - (C)haracter - (S)ave - E(X)it game");
}
