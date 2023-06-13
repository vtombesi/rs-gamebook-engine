use std::fs::File;
use std::io::{self};
use std::path::Path;
use std::fs::write;
use serde_json::{json, Value};
use crate::inventory::Inventory;
use crate::item::{Item, ItemType};
use crate::{Equipment, logger, Stats};

use super::player::Player;
use super::gamebook::{GameBook, Creature, Choice};
use std::fs::read_to_string;

pub fn load_gamebook() -> GameBook {
    let path = Path::new("./data/gamebook.json");
    let file = File::open(path).expect("Failed to open file");

    let mut gamebook: GameBook = serde_json::from_reader(file).expect("Failed to parse JSON");

    gamebook
}

pub fn handle_combat(player: &mut Player, creature: &mut Creature, gamebook: &GameBook, selected_option: &Choice, current_page_id: &mut usize) {
    // print!("{}[2J", 27 as char);

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
                    creature.take_damage(10);
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
                handle_loot(player, loot);
            }

            let current_page = gamebook.pages.iter().find(|p| p.id == selected_option.destination);
            if let Some(page) = current_page {
                // println!("{}", page.text);
                *current_page_id = selected_option.destination;
            } else {
                println!("End of the game. You won!");
            }

            break;
        }

        // Creature's turn
        let damage = creature.attack();
        player.take_damage(damage);
        logger::log_damage(format!("The {} attacks you and deals {} damage points.", creature.creature_name, damage));

        if player.health <= 0 {
            println!("{}", creature.defeat_text);
            println!("Game over.");
            std::process::exit(0);
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
    println!("Select a save slot (1, 2, or 3):");
    let mut save_slot = String::new();
    std::io::stdin().read_line(&mut save_slot).expect("Error reading input");

    let save_slot = save_slot.trim();
    let save_file_name = match save_slot {
        "1" => "saves/save1.json",
        "2" => "saves/save2.json",
        "3" => "saves/save3.json",
        _ => {
            println!("Invalid save slot. The game will be saved in 'savegame.json'.");
            "saves/savegame.json"
        }
    };

    let save_data = json!({
        "health": player.health,
        "inventory": player.inventory.items,
        "page_id": page_id,
    });

    write(save_file_name, save_data.to_string()).expect("Unable to write save file");
}

// pub fn load_game(slot: usize) -> Option<(Player, usize)> {
//     let save_file_name = format!("save{}.json", slot);
//
//     if let Ok(save_data) = read_to_string(&save_file_name) {
//         if let Ok(json_data) = serde_json::from_str::<Value>(&save_data) {
//             let health = json_data["health"].as_i64().unwrap_or(0) as i32;
//
//             let inventory_items = json_data["inventory"]
//                 .as_array()
//                 .unwrap_or(&vec![])
//                 .iter()
//                 .map(|item| {
//                     let item_name = item.as_str().unwrap_or("");
//                     Item::new(item_name.into(), ItemType::SomeType, 1, None)
//                 })
//                 .collect();
//
//             let page_id = json_data["page_id"].as_u64().unwrap_or(0) as usize;
//
//             let player = Player {
//                 health,
//                 inventory: Inventory { items: inventory_items },
//
//             };
//
//             return Some((player, page_id));
//         }
//     }
//
//     None
// }

pub fn handle_loot(player: &mut Player, loot: &[Item]) {
    for item in loot {
        player.pickup(item.clone());
    }
}

pub fn parse_initial_equipment(gamebook: &mut GameBook) {
    if let Some(player_data) = &mut gamebook.player {

        let stats_data = player_data.stats; // stats_data è di tipo `Stats`
        player_data.stats = stats_data;

        if let Some(armour_data) = &player_data.equipment.armour {
            // ora armour_data è di tipo `Item`
            // il tuo codice qui...
        }

        if let Some(weapon_data) = &player_data.equipment.weapon {
            // ora weapon_data è di tipo `Item`
            // il tuo codice qui...
        }

    }
}


