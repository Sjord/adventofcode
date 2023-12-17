use std::fs;
use std::env;
use std::str::FromStr;

fn main() {
    let fname = env::args().nth(1).unwrap();
    let contents = fs::read_to_string(fname).unwrap();
    let moves : Vec<Match> = contents.lines().map(|l| l.parse().unwrap()).collect();
    let score : i32 = moves.iter().map(|m| m.total_score()).sum();
    println!("{}", score);
}

#[derive(Copy, Clone)]
enum Hand {
    Rock,
    Paper,
    Scissors
}

enum Outcome {
    Lose,
    Draw,
    Win
}

impl Hand {
    fn beats(&self, other: &Hand) -> bool {
        match (self, other) {
            (Hand::Rock, Hand::Scissors) 
            | (Hand::Scissors, Hand::Paper)
            | (Hand::Paper, Hand::Rock) => true,
            _ => false
        }
    }

    fn score(&self) -> i32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3
        }
    }

    fn get_hand_for_outcome(&self, outcome: &Outcome) -> Hand {
        let hands = [Hand::Rock, Hand::Scissors, Hand::Paper];
        let beating_hand = hands.iter().filter(|h| h.beats(self)).next().unwrap();
        let losing_hand = hands.iter().filter(|h| self.beats(h)).next().unwrap();
        let draw_hand = self;
        match outcome {
            Outcome::Win => beating_hand.clone(),
            Outcome::Draw => draw_hand.clone(),
            Outcome::Lose => losing_hand.clone()
        }
    }
}

struct Match {
    me: Hand,
    opponent: Hand,
}

impl Match {
    fn beat_score(&self) -> i32 {
        if (self.me.beats(&self.opponent)) {
            return 6;
        }
        if (self.opponent.beats(&self.me)) {
            return 0;
        }
        return 3;
    }

    fn total_score(&self) -> i32 {
        return &self.me.score() + &self.beat_score();
    }
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Hand::Rock),
            "B" => Ok(Hand::Paper),
            "C" => Ok(Hand::Scissors),
            _ => Err(())
        }
    }
}

impl FromStr for Outcome {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(())
        }
    }
}

impl FromStr for Match {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_ascii_whitespace().collect::<Vec<&str>>();
        let opp = Hand::from_str(parts[0]).unwrap();
        let outcome = Outcome::from_str(parts[1]).unwrap();
        let me = opp.get_hand_for_outcome(&outcome);

        return Ok(Self {
            me: me,
            opponent: opp,
        })
    }
}
