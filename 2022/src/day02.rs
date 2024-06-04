
#[derive(Debug, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn play(&self, opponent: &Move) -> Outcome {
        if self != opponent {
            match (self, opponent) {
                (Move::Rock, Move::Scissors) |
                (Move::Paper, Move::Rock) |
                (Move::Scissors, Move::Paper) => Outcome::Win,
                _ => Outcome::Lose
            }
        } else {
            Outcome::Draw
        }
    }

    fn score(&self) -> i32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

impl Outcome {
    fn select(&self, opponent: &Move) -> Move {
        [Move::Rock, Move::Paper, Move::Scissors].into_iter().filter(
            |m| m.play(opponent) == *self).next().unwrap()
    }

    fn score(&self) -> i32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }
}

trait ParseFromChar {
    fn parse(ch: char) -> Self;
}

impl ParseFromChar for Move {
    fn parse(ch: char) -> Self {
        match ch {
            'A' | 'X' => Move::Rock,
            'B' | 'Y' => Move::Paper,
            'C' | 'Z' => Move::Scissors,
            _ => panic!("incorrect move")
        }
    }
}

impl ParseFromChar for Outcome {
    fn parse(ch: char) -> Self {
        match ch {
            'X' => Outcome::Lose,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => panic!("incorrect outcome")
        }
    }
}

#[derive(Debug)]
struct Game {
    plan1: Vec<(Move, Move)>,
    plan2: Vec<(Move, Outcome)>,
}

impl Game {
    fn parse_plan<T: ParseFromChar>(input: &str) -> Vec<(Move, T)> {
        input.lines().map(|line| {
            let chars: Vec<char> = line.split_whitespace().map(
                |s| s.chars().next().unwrap()).collect();
            (Move::parse(chars[0]), T::parse(chars[1]))
        }).collect()
    }

    fn parse(input: &str) -> Game {
        Game {
            plan1: Game::parse_plan::<Move>(input),
            plan2: Game::parse_plan::<Outcome>(input),
        }
    }

    fn score1(&self) -> i32 {
        self.plan1.iter().map(
            |(a, b)| b.score() + b.play(a).score()).sum()
    }

    fn score2(&self) -> i32 {
        self.plan2.iter().map(
            |(a, b)| b.select(a).score() + b.score()).sum()
    }
}

pub fn run(content: &str) {
    let input = Game::parse(content);
    println!("{} {}", input.score1(), input.score2())
}

#[cfg(test)]
mod tests {
    #[test]
    fn score() {
        let test = super::Game::parse("A Y\nB X\nC Z");
        assert_eq!(test.score1(), 15);
        assert_eq!(test.score2(), 12);
    }
}
