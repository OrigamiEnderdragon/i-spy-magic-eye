//! This module is responsible for creating and defining the encounters of the game.

use crate::dice;

/// Creates a list of the encounters for this game.
pub fn create_stage1_list() -> Vec<Encounter> {
    vec![
        Encounter::new("Stray Bolts", 6, &[Stat::Dodging]),
        Encounter::new("Heavy Rain", 4, &[Stat::Riding]),
        Encounter::new("Lightning Imp", 6, &[Stat::Slinging, Stat::Dodging]),
        Encounter::new("Dense Cloud Cover", 4, &[Stat::Riding]),
        Encounter::new(
            "Thunder, Just A Little Too Close To You",
            4,
            &[Stat::Dodging],
        ),
        Encounter::new(
            "Swarm of Birds",
            6,
            &[Stat::Slinging, Stat::Dodging, Stat::Riding],
        ),
        Encounter::new("Getting Lost", 4, &[Stat::Riding]),
        Encounter::new(
            "Cold and Wet and Miserable",
            4,
            &[Stat::Slinging, Stat::Dodging, Stat::Riding],
        ),
    ]
}
pub fn create_stage2_list() -> Vec<Encounter> {
    vec![
        Encounter::new("Raging Winds", 6, &[Stat::Riding]),
        Encounter::new("Electified Newts", 8, &[Stat::Slinging, Stat::Dodging]),
        Encounter::new("Sheet Lightning", 4, &[Stat::Dodging]),
        Encounter::new(
            "Storm-Weaver's Den",
            8,
            &[Stat::Slinging, Stat::Dodging, Stat::Riding],
        ),
        Encounter::new("More Lightning Imps", 8, &[Stat::Slinging, Stat::Dodging]),
        Encounter::new("Cloud-Hound", 8, &[Stat::Slinging, Stat::Dodging]),
        Encounter::new("Getting Lost (Again)", 4, &[Stat::Riding]),
        Encounter::new(
            "Colder and Wetter and Miserabler",
            6,
            &[Stat::Slinging, Stat::Dodging, Stat::Riding],
        ),
    ]
}

pub fn create_stage3_enviro_list() -> Vec<Encounter> {
    vec![
        Encounter::new(
            "Blinding and Deafening Lightning and Thunder",
            6,
            &[Stat::Slinging, Stat::Dodging, Stat::Riding],
        ),
        Encounter::new(
            "Conked on the Head by Hailstones",
            8,
            &[Stat::Slinging, Stat::Dodging, Stat::Riding],
        ),
        Encounter::new(
            "Tosses About by Wind and Rain (Lost, AGAIN)",
            6,
            &[Stat::Slinging, Stat::Dodging, Stat::Riding],
        ),
        Encounter::new(
            "Despair that you may never find your beloved eye in this seemingly endless storm…",
            8,
            &[Stat::Slinging, Stat::Dodging, Stat::Riding],
        ),
    ]
}

pub fn create_stage3_enemy_list() -> Vec<Encounter> {
    vec![
        Encounter::new(
            "Pack of Cloud Hounds (Very protective of their new ball.)",
            10,
            &[Stat::Slinging, Stat::Dodging, Stat::Riding],
        ),
        Encounter::new("Wyvern", 10, &[Stat::Slinging, Stat::Dodging, Stat::Riding]),
        Encounter::new(
            "(Evil) Storm Wizard (It’s Complicated.)",
            10,
            &[Stat::Slinging, Stat::Dodging, Stat::Riding],
        ),
        Encounter::new(
            "Roc Who Is Trying To Feed Your Eye To Her Babies",
            10,
            &[Stat::Slinging, Stat::Dodging, Stat::Riding],
        ),
    ]
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Encounter {
    name: String,
    difficulty: usize,
    allowed_stats: Vec<Stat>,
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
