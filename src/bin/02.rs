use advent_of_code::helpers::split_lines;

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

    pub fn from_result(result: &Result, lhs: &Hand) -> Hand {
        match lhs {
            Hand::Rock => match result {
                Result::Win => Hand::Paper,
                Result::Loss => Hand::Scissors,
                _ => Hand::Rock,
            },
            Hand::Paper => match result {
                Result::Win => Hand::Scissors,
                Result::Loss => Hand::Rock,
                _ => Hand::Paper,
            },
            Hand::Scissors => match result {
                Result::Win => Hand::Rock,
                Result::Loss => Hand::Paper,
                _ => Hand::Scissors,
            },
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

    fn from_input(input: &str) -> Result {
        match input {
            "X" => Result::Loss,
            "Y" => Result::Draw,
            "Z" => Result::Win,
            _ => panic!("Unknown input: {}", input),
        }
    }

    fn from_round(lhs: &Hand, rhs: &Hand) -> Result {
        match lhs {
            Hand::Rock => match rhs {
                Hand::Paper => Result::Win,
                Hand::Scissors => Result::Loss,
                _ => Result::Draw,
            },
            Hand::Paper => match rhs {
                Hand::Rock => Result::Loss,
                Hand::Scissors => Result::Win,
                _ => Result::Draw,
            },
            Hand::Scissors => match rhs {
                Hand::Rock => Result::Win,
                Hand::Paper => Result::Loss,
                _ => Result::Draw,
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

    pub fn from_round_input(input: &str) -> Game {
        let rounds: Vec<Round> = split_lines(input)
            .map(|line| {
                let hands: Vec<Hand> = line
                    .split_whitespace() // Split each line by space
                    .map(Hand::from_input) // Map each character as a "hand"
                    .collect();
                Round {
                    lhs: hands[0],
                    rhs: hands[1],
                } // Return one round with two hands
            })
            .collect();
        Game { rounds }
    }

    pub fn from_results_input(input: &str) -> Game {
        let rounds = split_lines(input)
            .map(|line| {
                let chars: Vec<&str> = line.split_whitespace().collect();
                let lhs = Hand::from_input(chars[0]);
                let result = Result::from_input(chars[1]);
                let rhs = Hand::from_result(&result, &lhs);
                Round { lhs, rhs }
            })
            .collect();
        Game { rounds }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let game = Game::from_round_input(input);
    Some(game.score())
}

pub fn part_two(input: &str) -> Option<u32> {
    let game = Game::from_results_input(input);
    Some(game.score())
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
        assert_eq!(part_two(&input), Some(12));
    }
}
