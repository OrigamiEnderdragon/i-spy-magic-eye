//! Defining the player character, aka the "character sheet."

use std::{error::Error, fmt::Display};

use crate::{
    dice,
    encounter::{Encounter, Stat},
};

const STAT_ROLL_FACES: usize = 6;

#[derive(Copy, Clone, Debug)]
pub struct BadStatError;
impl Display for BadStatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "given stat is not in list of the encounter's allowed stats"
        )
    }
}
impl Error for BadStatError {}

/// A character.
pub struct Character {
    name: String,
    description: String,
    vim: usize,
    stat_bonuses: StatBonuses,
}
impl Character {
    /// Creates a new [`Character`].
    pub fn new(name: &str, description: &str, vim: usize, stat_bonuses: StatBonuses) -> Character {
        Character {
            name: name.to_string(),
            description: description.to_string(),
            vim,
            stat_bonuses,
        }
    }

    /// Returns `true` if the [`Encounter`] was successful; `false` otherwise.
    pub fn challenge_encounter(
        &mut self,
        chosen_stat: Stat,
        encounter: &Encounter,
    ) -> Result<bool, BadStatError> {
        if !encounter.allowed_stats.contains(&chosen_stat) {
            return Err(BadStatError);
        }

        let roll = dice::roll_die(STAT_ROLL_FACES) + self.stat_bonuses.bonus(chosen_stat);
        let outcome = roll >= encounter.difficulty;
        if !outcome {
            self.vim -= 1;
        }

        Ok(outcome)
    }
}

/// The stat bonuses a particular character has.
pub struct StatBonuses {
    slinging: usize,
    dodging: usize,
    riding: usize,
}
impl StatBonuses {
    pub fn new(plus_3: Stat, plus_2: Stat, plus_1: Stat) -> Self {
        let mut sb = Self {
            slinging: 0,
            dodging: 0,
            riding: 0,
        };
        sb.increase_stat(plus_3, 3);
        sb.increase_stat(plus_2, 2);
        sb.increase_stat(plus_1, 1);
        sb
    }

    fn increase_stat(&mut self, stat: Stat, num: usize) {
        match stat {
            Stat::Slinging => self.slinging += num,
            Stat::Dodging => self.dodging += num,
            Stat::Riding => self.riding += num,
        }
    }

    fn bonus(&self, stat: Stat) -> usize {
        match stat {
            Stat::Slinging => self.slinging,
            Stat::Dodging => self.dodging,
            Stat::Riding => self.riding,
        }
    }
}

/// Stat data type.
struct StatModifier(usize);
