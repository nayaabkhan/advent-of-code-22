use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissor,
}

enum RoundResult {
    Won,
    Lost,
    Tie,
}

impl Hand {
    fn points(&self) -> u32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissor => 3,
        }
    }

    fn against(&self, opponent: &Hand) -> RoundResult {
        match self {
            Hand::Rock => match opponent {
                Hand::Rock => RoundResult::Tie,
                Hand::Paper => RoundResult::Lost,
                Hand::Scissor => RoundResult::Won,
            },
            Hand::Paper => match opponent {
                Hand::Rock => RoundResult::Won,
                Hand::Paper => RoundResult::Tie,
                Hand::Scissor => RoundResult::Lost,
            },
            Hand::Scissor => match opponent {
                Hand::Rock => RoundResult::Lost,
                Hand::Paper => RoundResult::Won,
                Hand::Scissor => RoundResult::Tie,
            },
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseHandError;

impl FromStr for Hand {
    type Err = ParseHandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Hand::Rock),
            "B" | "Y" => Ok(Hand::Paper),
            "C" | "Z" => Ok(Hand::Scissor),
            _ => Err(ParseHandError),
        }
    }
}

fn main() {
    let mut score = 0;
    let rounds = include_str!("../input.txt").lines();

    for round in rounds {
        let index_of_space = round.find(' ').unwrap();
        let (elf_hand, my_hand) = round.split_at(index_of_space);

        let elf_hand: Hand = elf_hand.trim().parse().unwrap();
        let my_hand: Hand = my_hand.trim().parse().unwrap();

        score += my_hand.points();

        match my_hand.against(&elf_hand) {
            RoundResult::Won => score += 6,
            RoundResult::Tie => score += 3,
            _ => (),
        }
    }

    println!("{}", score);
    assert!(score == 13052);
}
