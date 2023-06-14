use crate::{
    gamebook::{Choice, GameBook},
    item::ItemType,
    logger,
    utils::read_user_input,
};

pub fn handle_equipment(gamebook: &mut GameBook) {
    print!("{}[2J", 27 as char);

    'equipment: loop {
        gamebook.player.equipment.show();
        println!("Select an option to continue:");
        println!("T: Return to the game");
        println!("I: Open Inventory");

        let equipment_input = read_user_input();

        match equipment_input.trim().to_uppercase().as_str() {
            "T" => {
                // Return to the game
                break 'equipment;
            }
            "I" => {
                // Go to inventory
                break;
            }
            _ => {
                println!("Invalid input. Please try again.");
            }
        }
    }
}

pub fn handle_inventory(gamebook: &mut GameBook) {
    print!("{}[2J", 27 as char);

    loop {
        gamebook.player.inventory.show();
        println!("Select an item to use or type T to return to the game.");

        let inventory_input = read_user_input();

        if inventory_input.trim().to_uppercase() == "T" {
            // Return to the game
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

                handle_inventory(gamebook);
            }
        } else {
            println!("Invalid input. Please try again.");
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
    logger::log_choice("I. Check Inventory");
    logger::log_choice("E. Check Equipment");
    logger::log_choice("S. Save");
    logger::log_choice("X. Exit game");
}
