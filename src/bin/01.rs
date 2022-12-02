fn calories_from_input(input: &str) -> Box<dyn Iterator<Item = u32>> {
    let mut calories = vec![0u32];
    // Temporarily store lines as Some(u32)
    input
        .split('\n') // Split string on newline
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
    Box::new(calories.into_iter())
}

pub fn part_one(input: &str) -> Option<u32> {
    calories_from_input(input).max()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sorted = calories_from_input(input).collect::<Vec<u32>>();
    // There has to be a better solution than this?
    sorted.sort();
    sorted.reverse();
    // Take the top three values, and add them together
    let top_three = &sorted[..3];
    Option::Some(top_three.iter().sum::<u32>())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
