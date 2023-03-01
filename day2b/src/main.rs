use std::{io, str::FromStr};

#[derive(Debug, PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissor,
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Hand::Rock),
            "B" | "Y" => Ok(Hand::Paper),
            "C" | "Z" => Ok(Hand::Scissor),
            _ => Err(()),
        }
    }
}

impl Hand {
    fn points(&self) -> u32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissor => 3,
        }
    }

    fn desired_hand(opponent: &Hand, result: &RoundResult) -> Self {
        match result {
            RoundResult::Win => match opponent {
                Hand::Rock => Hand::Paper,
                Hand::Paper => Hand::Scissor,
                Hand::Scissor => Hand::Rock,
            },
            RoundResult::Lose => match opponent {
                Hand::Rock => Hand::Scissor,
                Hand::Paper => Hand::Rock,
                Hand::Scissor => Hand::Paper,
            },
            RoundResult::Tie => match opponent {
                Hand::Rock => Hand::Rock,
                Hand::Paper => Hand::Paper,
                Hand::Scissor => Hand::Scissor,
            },
        }
    }
}

#[derive(Debug, PartialEq)]
enum RoundResult {
    Win,
    Lose,
    Tie,
}

impl FromStr for RoundResult {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(RoundResult::Lose),
            "Y" => Ok(RoundResult::Tie),
            "Z" => Ok(RoundResult::Win),
            _ => Err(()),
        }
    }
}

fn main() {
    let mut score = 0;
    let lines = io::stdin().lines();

    for line in lines {
        let line = line.unwrap();

        let index_of_space = line.find(' ').unwrap();
        let (elf_hand, desired_result) = line.split_at(index_of_space);

        let elf_hand: Hand = elf_hand.trim().parse().unwrap();
        let desired_result: RoundResult = desired_result.trim().parse().unwrap();

        let my_hand = Hand::desired_hand(&elf_hand, &desired_result);

        score += my_hand.points();
        match desired_result {
            RoundResult::Win => score += 6,
            RoundResult::Tie => score += 3,
            _ => (),
        }
    }

    println!("{}", score);
}
