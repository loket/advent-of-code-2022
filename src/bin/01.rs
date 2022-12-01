use std::ops::Add;

pub fn part_one(input: &str) -> Option<u32> {
    let mut calories = vec![0u32];
    // Temporarily store lines as Some(u32)
    input.split("\n") // Split string on newline
    .for_each(|line| {
        if line.is_empty() {
            // Add a new entry to calories if the line is empty
            calories.push(0);
        } else {
            // Parse the line as a number, and add it to the last calory in the vector
            let index = calories.len() - 1;
            calories[index] += line.parse::<u32>().unwrap();
        }
    });
    calories.into_iter().max()
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &aoc::read_file("inputs", 1);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
