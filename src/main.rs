use std::fs::File;
use std::io::{self};
use std::path::Path;

mod gamebook;
use gamebook::{GameBook, Creature, Choice};

pub struct Player {
    health: i32,
}

impl Player {
    fn new() -> Self {
        Player { health: 20 }
    }

    fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
    }

    fn is_alive(&self) -> bool {
        self.health > 0
    }

    fn attack(&self) -> i32 {
        // Logic for calculating player's attack damage
        // You can modify this as per your game rules
        5
    }

    fn heal(&mut self, amount: i32) {
        self.health += amount;
    }
}

fn main() {
    // Carica il file gamebook.json
    let gamebook = load_gamebook();

    // Inizia dalla pagina di start_page
    let mut current_page_id = gamebook.start_page;

    // Crea il giocatore
    let mut player = Player::new();

// Loop principale
    loop {
        // Trova la pagina corrente
        let current_page = gamebook.pages.iter().find(|p| p.id == current_page_id);

        if let Some(mut page) = current_page.cloned() {
            // Stampa il testo della pagina corrente
            println!("{}", page.text);

            // Se non ci sono opzioni, esci dal loop
            if page.options.is_empty() {
                break;
            }

            // Stampa le opzioni disponibili
            for (index, option) in page.options.iter().enumerate() {
                println!("{}. {}", index + 1, option.text);
            }

            // Leggi l'input dell'utente
            let user_input = read_user_input();

            if let Ok(choice) = user_input.trim().parse::<usize>() {
                // Verifica se la scelta è valida
                if choice > 0 && choice <= page.options.len() {
                    if let Some(selected_option) = page.options.get(choice - 1) {
                        if let Some(mut creature) = selected_option.creature.clone() {
                            handle_combat(&mut player, &mut creature, &gamebook, selected_option, &mut current_page_id);
                        }

                        // Cambia la pagina corrente
                        current_page_id = selected_option.destination;
                    } else {
                        println!("Opzione non valida. Riprova.");
                    }
                } else {
                    println!("Scelta non valida. Riprova.");
                }
            } else {
                println!("Input non valido. Riprova.");
            }

        }

    }

}

pub fn handle_combat(player: &mut Player, creature: &mut Creature, gamebook: &GameBook, selected_option: &Choice, current_page_id: &mut usize) {
    println!("A {} appears before you:", creature.creature_name);

    println!("You: {}", player.health);
    println!("{}: {}", creature.creature_name, creature.health);

    loop {
        // Turno del giocatore
        println!("It's your turn. What do you want to do?");
        println!("1. Attack the {} with your weapon", creature.creature_name);
        println!("2. Defend and try to block the {}'s attack", creature.creature_name);
        println!("3. Heal yourself with a potion");
        println!("4. Run away");

        let user_input = read_user_input();
        let choice = user_input.trim().parse::<u32>();

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
            if let Some(loot) = &creature.loot {
                println!("Hai ottenuto: {}", loot);
                // Aggiungi qui la logica per gestire il loot
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
        println!("The {} attacks you and deals {} damage points.", creature.creature_name, damage);

        if player.health <= 0 {
            println!("{}", creature.defeat_text);
            println!("Game over.");
            break;
        }

        println!("You: {}", player.health);
        println!("{}: {}", creature.creature_name, creature.health);
    }
}

fn load_gamebook() -> GameBook {
    let path = Path::new("./data/gamebook.json");
    let file = File::open(path).expect("Failed to open file");

    serde_json::from_reader(file).expect("Failed to parse JSON")
}

fn read_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input
}
