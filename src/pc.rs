//! Defining the player character, aka the "character sheet."

/// A character.
pub struct Character {
    name: String,
    description: String,
    vim: usize,
    slinging: StatModifier,
    dodging: StatModifier,
    riding: StatModifier,
}
impl Character {
    fn new(
        name: &str,
        description: &str,
        vim: usize,
        slinging: StatModifier,
        dodging: StatModifier,
        riding: StatModifier,
    ) -> Character {
        Character {
            name: name.to_string(),
            description: description.to_string(),
            vim: vim,
            slinging: slinging,
            dodging: dodging,
            riding: riding,
        }
    }
}

/// Stat data type.
struct StatModifier(usize);
