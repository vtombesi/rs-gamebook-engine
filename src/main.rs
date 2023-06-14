mod gamebook;
mod player;
mod utils;
mod inventory;
mod effect;
mod item;
mod equipment;
mod stats;
mod logger;
mod player_actions;

use player::Player;
use utils::{load_gamebook};

use crate::{utils::{handle_loot, parse_initial_equipment}};

mod game_loop; // Import the new game_loop module

fn main() {
    let mut gamebook = load_gamebook();

    parse_initial_equipment(&mut gamebook);

    print!("{}[2J", 27 as char);

    game_loop::run(&mut gamebook);
}
