use std::collections::HashSet;

use itertools::Itertools;

type StartOfPacket = (char, char, char, char);
type StartOfMessage = (char, char, char, char, char, char, char, char, char);

// TODO: Don't hard-code the window size
/*
pub fn first_marker(input: &str) -> u32 {
    // Create sliding window of 4 characters
    let windows = input.chars().tuple_windows::<StartOfPacket>();
    // Enumerate all characters
    let e = windows.enumerate();
    // Skip until there's a match in first-of-packet marker
    let mut skipped = e.skip_while(|(i, chars)| {
        let set: HashSet<char> = vec![chars.0, chars.1, chars.2, chars.3]
            .into_iter()
            .collect();
        set.iter().count() != 4
    });
    // Return the index, plus offset 4
    skipped.next().unwrap().0 as u32 + 4
}
*/

fn first_marker(input: &str, window_size: usize) -> u32 {
    let range_end = input.len() - window_size;
    let mut range = 0..range_end - 1;
    let chars: Vec<char> = input.chars().collect();
    let matched = range
    .find(|i| {
        let window = chars.iter().skip(*i).take(window_size);
        let window_set: HashSet<&char> = window.into_iter().collect();
        window_set.iter().count() == window_size
    })
    .unwrap();
    let result = matched + window_size;
    result as u32
}

fn first_start_of_packet_marker(input: &str) -> u32 {
    first_marker(&input, 4)
}

fn first_start_of_message_marker(input: &str) -> u32 {
    first_marker(&input, 14)
}

pub fn part_one(input: &str) -> Option<u32> {
    let marker = first_start_of_packet_marker(input);
    Some(marker)
}

pub fn part_two(input: &str) -> Option<u32> {
    let marker = first_start_of_message_marker(input);
    Some(marker)
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
        let marker = first_start_of_packet_marker(input);
        assert_eq!(marker, 5);
    }

    #[test]
    fn test_example_2() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        let marker = first_start_of_packet_marker(input);
        assert_eq!(marker, 6);
    }

    #[test]
    fn test_example_3() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let marker = first_start_of_packet_marker(input);
        assert_eq!(marker, 10);
    }

    #[test]
    fn test_example_4() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let marker = first_start_of_packet_marker(input);
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
