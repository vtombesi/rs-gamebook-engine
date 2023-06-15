use std::fs::{File, read_to_string};
use std::io::{self};
use std::path::Path;
use std::fs::write;
use std::fs;
use serde_json::{json, Map, Value};
use crate::item::{Item, ItemType};
use crate::{logger};
use crate::equipment::{Equipment, EquipmentSlot};
use crate::inventory::Inventory;
use crate::player::PlayerImportData;
use crate::stats::Stats;

use super::player::Player;
use super::gamebook::{GameBook, Creature, Choice};

pub fn load_gamebook() -> GameBook {
    let path = Path::new("./data/gamebook.json");
    let file = File::open(path).expect("Failed to open file");

    let gamebook: GameBook = serde_json::from_reader(file).expect("Failed to parse JSON");

    gamebook
}

pub fn handle_combat(gamebook: &mut GameBook, creature: &mut Creature, selected_option: &Choice, current_page_id: &mut usize) {
    // print!("{}[2J", 27 as char);

    let mut player: Player = gamebook.player.clone();

    let mut blocking: bool = false;

    logger::log_monster_name(format!("A {} appears before you:", creature.creature_name));

    println!("You: {}", player.health);
    println!("{}: {}", creature.creature_name, creature.health);

    loop {
        blocking = false;

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
                    blocking = true;
                    // Add your defense logic here
                }
                3 => {
                    let healing = player.heal(10);
                    logger::log_narration(format!("You heal yourself for {:?} points.", healing));
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
                handle_loot(&mut player, loot);
            }

            let current_page = gamebook.pages.iter().find(|p| p.id == selected_option.destination);
            if let Some(_page) = current_page {
                // println!("{}", page.text);
                *current_page_id = selected_option.destination;
            } else {
                println!("End of the game. You won!");
            }

            break;
        }

        // Creature's turn
        let (attack_name, damage) = creature.attack();
        player.take_damage(damage);
        logger::log_damage(format!("The {} attacks you with {} and deals {} damage points.", creature.creature_name, attack_name, damage));

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

pub fn save_game(gamebook: &GameBook, page_id: usize) {
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

    let player: Player = gamebook.player.clone();

    let save_data = json!({
        "health": player.health,
        "inventory": player.inventory.items,
        "equipment": player.equipment,
        "visited_pages": gamebook.visited_pages,
        "page_id": page_id,
    });

    write(save_file_name, save_data.to_string()).expect("Unable to write save file");
}

pub fn load_game(gamebook: &mut GameBook) -> Option<usize> {
    println!("Select a save slot:");
    let save_files = match fs::read_dir("saves") {
        Ok(files) => files
            .filter_map(|file| {
                file.ok().and_then(|entry| {
                    entry
                        .file_name()
                        .into_string()
                        .ok()
                        .and_then(|file_name| file_name.strip_suffix(".json").map(|name| name.to_owned()))
                })
            })
            .collect::<Vec<String>>(),
        Err(_) => {
            println!("No save files found. Unable to load the game.");
            return None;
        }
    };

    for (index, file_name) in save_files.iter().enumerate() {
        println!("{}. {}", index + 1, file_name);
    }

    let mut slot = String::new();
    std::io::stdin().read_line(&mut slot).expect("Error reading input");
    let slot = slot.trim().parse::<usize>().unwrap_or(0);

    if slot > 0 && slot <= save_files.len() {
        let selected_slot = slot - 1;
        let save_file_name = format!("saves/{}.json", save_files[selected_slot]);

        if let Ok(save_data) = read_to_string(&save_file_name) {
            if let Ok(json_data) = serde_json::from_str::<Value>(&save_data) {
                let health = json_data["health"].as_i64().unwrap_or(0) as i32;

                let inventory_items = json_data["inventory"]
                    .as_array()
                    .unwrap_or(&vec![])
                    .iter()
                    .map(|item| {
                        let item_name = item.as_str().unwrap_or("");
                        let item_type = match item["type"].as_str() {
                            Some("Usable") => ItemType::Usable,
                            Some("Armour") => ItemType::Armour,
                            Some("Weapon") => ItemType::Weapon,
                            Some("Shield") => ItemType::Shield,
                            Some("Necklace") => ItemType::Necklace,
                            Some("Ring") => ItemType::Ring,
                            Some("Quest") => ItemType::Quest,
                            _ => {
                                // Gesto dell'errore: tipo di oggetto non valido
                                panic!("Invalid item type");
                            }
                        };
                        Item::new(item_name.into(), item_type, 1, None)
                    })
                    .collect();

                let page_id = json_data["page_id"].as_u64().unwrap_or(0) as usize;


                let default_equipment = serde_json::Map::new();
                let equipment = json_data["equipment"]
                    .as_object()
                    .unwrap_or(&default_equipment);

                let equipped_items: Vec<Item> = equipment
                    .iter()
                    .filter_map(|(slot_name, item_value)| {
                        let item_name = item_value["name"].as_str().unwrap_or("");
                        let item_type = match slot_name.as_str() {
                            "armour" => ItemType::Armour,
                            "weapon" => ItemType::Weapon,
                            "shield" => ItemType::Shield,
                            "necklace" => ItemType::Necklace,
                            "ring" => ItemType::Ring,
                            _ => return None,
                        };
                        Some(Item::new(item_name.into(), item_type, 1, None))
                    })
                    .collect();

                gamebook.player.health = health;
                gamebook.player.inventory.items = inventory_items;
                gamebook.player.equipment = Equipment {
                    armour: equipped_items
                        .iter()
                        .find(|item| item.item_type == ItemType::Armour)
                        .cloned(),
                    weapon: equipped_items
                        .iter()
                        .find(|item| item.item_type == ItemType::Weapon)
                        .cloned(),
                    shield: equipped_items
                        .iter()
                        .find(|item| item.item_type == ItemType::Shield)
                        .cloned(),
                    necklace: equipped_items
                        .iter()
                        .find(|item| item.item_type == ItemType::Necklace)
                        .cloned(),
                    ring: equipped_items
                        .iter()
                        .find(|item| item.item_type == ItemType::Ring)
                        .cloned(),
                };

                gamebook.visited_pages = json_data["visited_pages"]
                    .as_array()
                    .unwrap_or(&vec![])
                    .iter()
                    .map(|page| page.as_u64().unwrap_or(0) as usize)
                    .collect();

                return Some(page_id);
            }
        }
    } else {
        println!("Invalid save slot. Unable to load the game.");
    }

    None
}



pub fn handle_loot(player: &mut Player, loot: &[Item]) {
    for item in loot {
        player.pickup_item(item);
    }
}

pub fn parse_initial_equipment(gamebook: &mut GameBook) {
    let initial: Option<PlayerImportData> = gamebook.initial.clone();

    if let Some(initial_options) = initial {
        gamebook.player.stats = Stats {
            strength: initial_options.stats.strength,
            agility: initial_options.stats.agility,
            spirit: initial_options.stats.spirit,
            luck: initial_options.stats.luck,
        };

        if let Some(armour_item) = initial_options.equipment.armour {
            gamebook.player.equipment.equip_item(armour_item);
        }

        if let Some(weapon_item) = initial_options.equipment.weapon {
            gamebook.player.equipment.equip_item(weapon_item);
        }

        if let Some(shield_item) = initial_options.equipment.shield {
            gamebook.player.equipment.equip_item(shield_item);
        }

        if let Some(ring_item) = initial_options.equipment.ring {
            gamebook.player.equipment.equip_item(ring_item);
        }

        if let Some(necklace_item) = initial_options.equipment.necklace {
            gamebook.player.equipment.equip_item(necklace_item);
        }
    }
}




