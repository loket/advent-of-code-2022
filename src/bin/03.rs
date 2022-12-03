use std::collections::HashSet;

use advent_of_code::helpers::split_lines;
use advent_of_code::helpers::str_as_chars;

struct Backpack {
    left: Vec<char>,
    right: Vec<char>,
}

impl Backpack {
    fn intersection(&self) -> Vec<&char> {
        // Convert vectors to sets
        let left_set: HashSet<&char> = self.left.iter().collect();
        let right_set: HashSet<&char> = self.right.iter().collect();
        left_set
            .intersection(&right_set) // Find intersection between sets
            .copied() // Copy all elements
            .collect()
    }

    /// Find the priority of the item which exists in both compartments in the backpack (intersection)
    /// Lowercase item types a through z have priorities 1 through 26.
    /// Uppercase item types A through Z have priorities 27 through 52.
    /// ASCII characters a through z have values 97 through 122
    /// ASCII characters A through Z have values 65 through 90
    fn priority(&self) -> u32 {
        let intersection = self.intersection()[0];
        let char_number = *intersection as u32;
        if intersection.is_lowercase() {
            return char_number - 97 + 1; // a-z: Priority 1-26, ASCII 97-122
        }
        if intersection.is_uppercase() {
            return char_number - 65 + 27; // A-Z: Priority 27-52, ASCII 65-90
        }
        panic!("Intersecting value is not a character: {}", intersection)
    }

    fn from_str(input: &str) -> Backpack {
        let length = input.len();
        let left = str_as_chars(&input[..length / 2]);
        let right = str_as_chars(&input[length / 2..]);
        Backpack { left, right }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let priority: u32 = split_lines(input)
        .map(Backpack::from_str)
        .map(|backpack| backpack.priority())
        .sum();
    Some(priority)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}
