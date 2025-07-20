//! This module is responsible for creating and defining the encounters of the game.

use crate::dice;

/// Creates a list of the encounters for this game.
pub fn create_encounter_list() -> Vec<Encounter> {
    todo!()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Encounter {
    pub name: String,
    pub difficulty: usize,
    pub allowed_stats: Vec<Stat>,
}
impl Encounter {
    fn new(name: &str, difficulty_roll_faces: usize, allowed_stat_checks: &[Stat]) -> Self {
        Self {
            name: name.to_string(),
            difficulty: dice::roll_die(difficulty_roll_faces),
            allowed_stats: allowed_stat_checks.to_vec(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stat {
    Dodging,
    Riding,
    Slinging,
}
