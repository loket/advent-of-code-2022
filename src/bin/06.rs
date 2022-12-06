use std::collections::HashSet;

use itertools::Itertools;

// TODO: Don't hard-code the window size
pub fn first_marker(input:&str) -> u32 {
    // Create sliding window of 4 characters
    let windows = input.chars().tuple_windows::<(_,_,_,_)>();
    // Enumerate all characters
    let e = windows.enumerate();
    // Skip until there's a match in first-of-packet marker
    let mut skipped = e.skip_while(|(i, chars)| {
        let set: HashSet<char> = vec![chars.0, chars.1, chars.2, chars.3].into_iter().collect();
        set.iter().count() != 4
    });
    // Return the index, plus offset 4
    skipped.next().unwrap().0 as u32 + 4
}

pub fn part_one(input: &str) -> Option<u32> {
    let marker = first_marker(input);
    Some(marker)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let marker = first_marker(input);
        assert_eq!(marker, 5);
    }

    #[test]
    fn test_example_2() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        let marker = first_marker(input);
        assert_eq!(marker, 6);
    }

    #[test]
    fn test_example_3() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let marker = first_marker(input);
        assert_eq!(marker, 10);
    }

    #[test]
    fn test_example_4() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let marker = first_marker(input);
        assert_eq!(marker, 11);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), None);
    }
}
