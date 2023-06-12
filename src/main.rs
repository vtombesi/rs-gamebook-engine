mod gamebook;
mod player;
mod utils;
mod inventory;
mod effect;
mod item;
mod logger;

use player::Player;
use utils::{load_gamebook, handle_combat, read_user_input, save_game};

fn main() {
    // Carica il file gamebook.json
    let gamebook = load_gamebook();

    // Inizia dalla pagina di start_page
    let mut current_page_id = gamebook.start_page;

    // Crea il giocatore
    let mut player = Player::new();

    print!("{}[2J", 27 as char);

// Loop principale
    loop {
        // Svuota il terminale
        print!("{}[2J", 27 as char);

        // Trova la pagina corrente
        let current_page = gamebook.pages.iter().find(|p| p.id == current_page_id);

        if let Some(page) = current_page.cloned() {
            // Stampa il testo della pagina corrente
            logger::log_narration(format!("{}", page.text));
            println!();
            println!();

            // Se non ci sono opzioni, esci dal loop
            if page.options.is_empty() {
                break;
            }

            // Stampa le opzioni disponibili
            for (index, option) in page.options.iter().enumerate() {
                logger::log_choice(format!("{}. {}", index + 1, option.text));
            }

            // Stampa le opzioni globali
            logger::log_choice("X. Esci dal gioco");
            logger::log_choice("S. Salva posizione");
            logger::log_choice("I. Guarda l'inventario");

            // Leggi l'input dell'utente
            let user_input = read_user_input();

            // Verifica se l'input dell'utente è una stringa
            if user_input.trim().to_uppercase() == "X" {
                std::process::exit(0);
            } else if user_input.trim().to_uppercase() == "S" {
                save_game(&player, current_page_id);
                continue;
            } else if user_input.trim().to_uppercase() == "I" {
                loop {
                    player.inventory.show();
                    println!("Seleziona un oggetto da usare o digita T per tornare al gioco.");

                    let inventory_input = read_user_input();

                    if inventory_input.trim().to_uppercase() == "T" {
                        // Torna al gioco
                        break;
                    } else if let Ok(item_choice) = inventory_input.trim().parse::<usize>() {
                        // Tenta di usare l'oggetto scelto
                        if let Err(e) = player.inventory.use_item(item_choice) {
                            println!("Errore nell'uso dell'oggetto: {}", e);
                        }
                    } else {
                        println!("Input non valido. Riprova.");
                    }
                }
                continue;
            }

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
