use crate::{utils::{handle_combat, read_user_input, save_game}, logger, handle_loot};
use crate::gamebook::GameBook;
use crate::player_actions;

pub fn run(gamebook: &mut GameBook) {
    let mut current_page_id = gamebook.start_page;

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

            player_actions::show_options(&page.options);

            // Read user input
            let user_input = read_user_input();

            // Check if the user input is a string
            if user_input.trim().to_uppercase() == "X" {
                std::process::exit(0);
            } else if user_input.trim().to_uppercase() == "S" {
                save_game(&mut gamebook.player, current_page_id);
                continue;
            } else if user_input.trim().to_uppercase() == "E" {
                player_actions::handle_equipment(gamebook);
                continue;
            } else if user_input.trim().to_uppercase() == "I" {
                player_actions::handle_inventory(gamebook);
                continue;
            }

            if let Ok(choice) = user_input.trim().parse::<usize>() {
                // Check if the choice is valid
                if choice > 0 && choice <= page.options.len() {
                    if let Some(selected_option) = page.options.get(choice - 1) {

                        if !gamebook.visited_pages.contains(&current_page_id) {
                            if let Some(mut creature) = selected_option.creature.clone() {
                                handle_combat(gamebook, &mut creature, selected_option, &mut current_page_id);
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
