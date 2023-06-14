mod gamebook;
mod player;
mod utils;
mod inventory;
mod effect;
mod item;
mod equipment;
mod stats;
mod logger;

use player::Player;
use utils::{load_gamebook, handle_combat, read_user_input, save_game};
use crate::{utils::{handle_loot, parse_initial_equipment}, item::ItemType};

fn main() {
    // Load the gamebook.json file
    let mut gamebook = load_gamebook();

    // Start from the start_page page
    let mut current_page_id = gamebook.start_page;

    parse_initial_equipment(&mut gamebook);

    print!("{}[2J", 27 as char);

// Main loop
    loop {
        // Clear the terminal
        // print!("{}[2J", 27 as char);

        // Find the current page
        let current_page = gamebook.pages.iter().find(|p| p.id == current_page_id);

        if let Some(page) = current_page.cloned() {
            // Print the text of the current page
            logger::log_narration(format!("{}", page.text));
            println!();
            println!();

            if !gamebook.visited_pages.contains(&current_page_id) {
                if let Some(loot) = page.loot.as_ref() {
                    handle_loot(&mut gamebook.player, loot);
                    println!();
                }
            }

            // If there are no options, exit the loop
            if page.options.is_empty() {
                break;
            }

            // Print the available options
            for (index, option) in page.options.iter().enumerate() {
                logger::log_choice(format!("{}. {}", index + 1, option.text));
            }

            // Print global options
            println!();
            logger::log_choice("I. Check Inventory");
            logger::log_choice("E. Check Equipment");
            logger::log_choice("S. Save");
            logger::log_choice("X. Exit game");

            // Read user input
            let user_input = read_user_input();

            // Check if the user input is a string
            if user_input.trim().to_uppercase() == "X" {
                std::process::exit(0);
            } else if user_input.trim().to_uppercase() == "S" {
                save_game(&mut gamebook.player, current_page_id);
                continue;
            } else if user_input.trim().to_uppercase() == "E" {
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
            } else if user_input.trim().to_uppercase() == "I" {
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
                        }
                    } else {
                        println!("Invalid input. Please try again.");
                    }
                }
                continue;
            }

            if let Ok(choice) = user_input.trim().parse::<usize>() {
                // Check if the choice is valid
                if choice > 0 && choice <= page.options.len() {
                    if let Some(selected_option) = page.options.get(choice - 1) {
                        
                        if !gamebook.visited_pages.contains(&current_page_id) {
                            if let Some(mut creature) = selected_option.creature.clone() {
                                handle_combat(&mut gamebook, &mut creature, selected_option, &mut current_page_id);
                            }
                        }

                        gamebook.visited_pages.insert(current_page_id);

                        // Change the current page
                        current_page_id = selected_option.destination;
                    } else {
                        println!("Invalid option. Please try again.");
                    }
                } else {
                    println!("Invalid choice. Please try again.");
                }
            } else {
                println!("Invalid input. Please try again.");
            }

        }

    }

}
