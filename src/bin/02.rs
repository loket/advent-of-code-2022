#[derive(Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    pub fn from_input(input: &str) -> Hand {
        match input {
            "A" | "X" => Hand::Rock,
            "B" | "Y" => Hand::Paper,
            "C" | "Z" => Hand::Scissors,
            _ => panic!("Unknown input: {}", input),
        }
    }

    // TODO: Use smaller data type, e.g. u8
    pub fn score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

struct Round {
    lhs: Hand,
    rhs: Hand,
}

enum Result {
    Win,
    Draw,
    Loss,
}

impl Result {
    fn score(&self) -> u32 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Loss => 0,
        }
    }

    fn from_round(lhs: &Hand, rhs: &Hand) -> Result {
        match lhs {
            Hand::Rock => match rhs {
                Hand::Rock => Result::Draw,
                Hand::Paper => Result::Win,
                Hand::Scissors => Result::Loss,
            },
            Hand::Paper => match rhs {
                Hand::Rock => Result::Loss,
                Hand::Paper => Result::Draw,
                Hand::Scissors => Result::Win,
            },
            Hand::Scissors => match rhs {
                Hand::Rock => Result::Win,
                Hand::Paper => Result::Loss,
                Hand::Scissors => Result::Draw,
            },
        }
    }
}

impl Round {
    fn score(&self) -> u32 {
        let result = Result::from_round(&self.lhs, &self.rhs);
        result.score() + self.rhs.score()
    }
}

struct Game {
    rounds: Vec<Round>,
}

impl Game {
    fn score(&self) -> u32 {
        self.rounds.iter().map(|round| round.score()).sum()
    }
}

fn input_as_game(input: &str) -> Game {
    let rounds: Vec<Round> = input
        .split("\n") // Split by line
        .filter(|line| !line.is_empty()) // Remove empty lines
        .into_iter()
        .map(|line| {
            let hands: Vec<Hand> = line
                .split(" ") // Split each line by space
                .map(|char| Hand::from_input(char)) // Map each character as a "hand"
                .collect();
            Round {
                lhs: hands[0],
                rhs: hands[1],
            } // Return one round with two hands
        })
        .collect();
    Game { rounds: rounds }
}

pub fn part_one(input: &str) -> Option<u32> {
    let game = input_as_game(input);
    Some(game.score())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
