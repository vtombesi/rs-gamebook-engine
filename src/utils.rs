use std::fs::File;
use std::io::{self};
use std::path::Path;
use std::fs::write;
use serde_json::json;
use crate::logger;

use super::player::Player;
use super::gamebook::{GameBook, Creature, Choice};

pub fn load_gamebook() -> GameBook {
    let path = Path::new("./data/gamebook.json");
    let file = File::open(path).expect("Failed to open file");

    serde_json::from_reader(file).expect("Failed to parse JSON")
}

pub fn handle_combat(player: &mut Player, creature: &mut Creature, gamebook: &GameBook, selected_option: &Choice, current_page_id: &mut usize) {
    print!("{}[2J", 27 as char);

    logger::log_monster_name(format!("A {} appears before you:", creature.creature_name));

    println!("You: {}", player.health);
    println!("{}: {}", creature.creature_name, creature.health);

    loop {
        println!();
        logger::log_narration("It's your turn. What do you want to do?");
        println!();
        logger::log_choice("1. Attack");
        logger::log_choice("2. Defend");
        logger::log_choice("3. Use an item");
        logger::log_choice("4. Flee");

        let user_input = read_user_input();
        let choice = user_input.trim().parse::<u32>();

        print!("{}[2J", 27 as char);

        match choice {
            Ok(choice) => match choice {
                1 => {
                    let damage = player.attack();
                    creature.take_damage(damage);
                    println!("You attack the {} and damage it for {} damage points.", creature.creature_name, damage);
                }
                2 => {
                    println!("You try to block the {}'s attack.", creature.creature_name);
                    // Add your defense logic here
                }
                3 => {
                    let healing = player.heal(10);
                    println!("You heal yourself for {:?} points.", healing);
                }
                4 => {
                    println!("You decide to run away from the {}.", creature.creature_name);
                    break;
                }
                _ => {
                    println!("Invalid choice. Try again.");
                }
            },
            Err(_) => {
                println!("Invalid input. Try again.");
            }
        }

        if creature.health <= 0 {
            println!("{}", creature.victory_text);
            if let Some(loot) = selected_option.creature.as_ref().and_then(|creature| creature.loot.as_ref()) {
                for item in loot {
                    player.pickup(item.clone());
                }
            }

            let current_page = gamebook.pages.iter().find(|p| p.id == selected_option.destination);
            if let Some(page) = current_page {
                println!("{}", page.text);
                *current_page_id = selected_option.destination;
            } else {
                println!("Fine del gioco. Hai vinto!");
            }

            break;
        }

        // Turno della creatura
        let damage = creature.attack();
        player.take_damage(damage);
        logger::log_damage(format!("The {} attacks you and deals {} damage points.", creature.creature_name, damage));

        if player.health <= 0 {
            println!("{}", creature.defeat_text);
            println!("Game over.");
            break;
        }

        println!("You: {}", player.health);
        println!("{}: {}", creature.creature_name, creature.health);
    }
}

pub fn read_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input
}

pub fn save_game(player: &Player, page_id: usize) {
    let save_data = json!({
        "health": player.health,
        "inventory": player.inventory.items,
        "page_id": page_id,
    });

    write("savegame.json", save_data.to_string()).expect("Unable to write save game file");
}
