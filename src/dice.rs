//! This module is responsible for dice.

/// The function `roll_die` takes in a single `usize` as an argument.
/// That `usize` represents how many faces the dice has.
/// It returns a single `usize` that represents the number that was rolled.
#[must_use]
pub fn roll_die(num_faces: usize) -> usize {
    // Set the "result" variable (which has data type "usize") to a random number from 1 to "num_faces"
    // (inclusive)
    let result: usize = rand::random_range(1..=num_faces);
    // Return the random number you rolled!
    result
}
