// Nel file gamebook.rs

use std::collections::HashSet;

use serde::{Deserialize};
use rand::Rng;
use rand::seq::SliceRandom;
use crate::item::Item;
use crate::Player;
use crate::player::PlayerImportData;

impl Creature {
    pub fn take_damage(&mut self, damage: i32) {
        self.health -= damage;
    }

    pub fn attack(&self) -> (String, i32) {
        let mut rng = rand::thread_rng();
        let random_attack = self.attacks.choose(&mut rng).expect("No attacks available");

        (random_attack.name.clone(), random_attack.damage)
    }
}

#[derive(Debug, Deserialize)]
pub struct GameBook {
    pub start_page: usize,
    pub pages: Vec<Page>,
    #[serde(default)]
    pub player: Player,
    pub initial: Option<PlayerImportData>,
    #[serde(default)]
    pub visited_pages: HashSet<usize>,
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
    pub attacks: Vec<Attack>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Combat {
    pub creature: Creature,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Attack {
    pub name: String,
    pub damage: i32,
}
