// Nel file gamebook.rs

use serde::{Deserialize};
use rand::Rng;
use crate::item::Item;
use crate::Player;

impl Creature {
    pub fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
    }

    pub fn attack(&self) -> i32 {
        let damage = rand::thread_rng().gen_range(1..10);
        damage
    }
}

#[derive(Debug, Deserialize)]
pub struct GameBook {
    pub start_page: usize,
    pub pages: Vec<Page>,
    pub player: Option<Player>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Page {
    pub id: usize,
    pub text: String,
    pub loot: Option<Vec<Item>>,
    pub options: Vec<Choice>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Choice {
    pub text: String,
    pub destination: usize,
    #[serde(rename = "combat")]  // Rinomina il campo "combat" in "creature"
    pub creature: Option<Creature>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Creature {
    #[serde(rename = "name")]
    pub creature_name: String,
    pub health: i32,
    #[serde(rename = "victory_text")]
    pub victory_text: String,
    #[serde(rename = "defeat_text")]
    pub defeat_text: String,
    pub loot: Option<Vec<Item>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Combat {
    pub creature: Creature,
    pub attacks: Vec<Attack>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Attack {
    pub name: String,
    pub damage: i32,
}
