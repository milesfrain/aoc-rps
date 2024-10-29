/* ----- Game Logic: hands and rankings ----- */

#[derive(PartialEq, Eq, Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn beats(&self) -> Self {
        match self {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
        }
    }
}

/* ----- Game Logic: points data ----- */

fn score(opponent: Hand, me: Hand) -> u32 {
    (match me {
        Hand::Rock => 1,
        Hand::Paper => 2,
        Hand::Scissors => 3,
    }) + match () {
        _ if opponent.beats() == me => 0, // lose
        _ if opponent == me => 3,         // draw
        _ => 6,                           // win
    }
}

/* ----- Game Logic: part 2 extension ----- */

enum Outcome {
    Lose,
    Draw,
    Win,
}

fn pick_strategy(opponent: Hand, outcome: Outcome) -> Hand {
    match outcome {
        Outcome::Draw => opponent,
        Outcome::Lose => opponent.beats(),
        Outcome::Win => opponent.beats().beats(),
    }
}

/* ----- Parsing ----- */

#[derive(Clone, Copy)]
enum Column1 {
    A,
    B,
    C,
}

impl TryFrom<char> for Column1 {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' => Ok(Column1::A),
            'B' => Ok(Column1::B),
            'C' => Ok(Column1::C),
            _ => Err("Invalid char"), // TBD if we want to note the char
        }
    }
}

#[derive(Clone, Copy)]
enum Column2 {
    X,
    Y,
    Z,
}

impl TryFrom<char> for Column2 {
    type Error = &'static str;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'X' => Ok(Column2::X),
            'Y' => Ok(Column2::Y),
            'Z' => Ok(Column2::Z),
            _ => Err("Invalid char"),
        }
    }
}

impl From<Column1> for Hand {
    fn from(c: Column1) -> Hand {
        match c {
            Column1::A => Hand::Rock,
            Column1::B => Hand::Paper,
            Column1::C => Hand::Scissors,
        }
    }
}

impl From<Column2> for Hand {
    fn from(c: Column2) -> Hand {
        match c {
            Column2::X => Hand::Rock,
            Column2::Y => Hand::Paper,
            Column2::Z => Hand::Scissors,
        }
    }
}

fn parse_line(line: &str) -> Result<(Column1, Column2), &str> {
    let mut chars = line.chars();

    let col1 = chars.next().ok_or("Missing column 1 char")?.try_into()?;

    chars
        .next()
        .filter(|&c| c == ' ')
        .ok_or("Invalid or missing space delimiter")?;

    let col2 = chars.next().ok_or("Missing column 2 char")?.try_into()?;

    if chars.next().is_some() {
        return Err("Unexpected trailing chars"); // TBD if we want to be this strict
    }

    Ok((col1, col2))
}

/* ----- Parsing: part 2 extension ----- */

impl From<Column2> for Outcome {
    fn from(c: Column2) -> Outcome {
        match c {
            Column2::X => Outcome::Lose,
            Column2::Y => Outcome::Draw,
            Column2::Z => Outcome::Win,
        }
    }
}

/* ----- Runner ----- */

fn part1_game((col1, col2): (Column1, Column2)) -> u32 {
    score(col1.into(), col2.into())
}

fn part2_game((col1, col2): (Column1, Column2)) -> u32 {
    score(col1.into(), pick_strategy(col1.into(), col2.into()))
}

fn run() -> Result<u32, String> {
    use std::io::{self, Read};
    // It may be best to convert run() to a pure function,
    // and move all IO from run() to main().
    // That might require some cumbersome modifications to
    // still capture line-by-line parsing errors.
    // Maybe returning Result<u32, vec<String>> would work.

    let mut strategy_guide = String::new();
    io::stdin()
        .read_to_string(&mut strategy_guide)
        .map_err(|e| e.to_string())?;
    let mut lines = strategy_guide.lines();
    let part = lines.next().ok_or("Missing data on first line")?;

    let strategies = lines
        .enumerate()
        .filter_map(|(n, line)| match parse_line(line) {
            Ok(t) => Some(t),
            Err(err) => {
                eprintln!("Parsing error on line {}: {}", n + 1, err);
                None
            }
        });

    match part {
        "Part 1" => Ok(strategies.map(part1_game).sum()),
        "Part 2" => Ok(strategies.map(part2_game).sum()),
        _ => Err("Malformed input".to_string()),
    }
}

fn main() {
    match run() {
        Ok(score) => println!("{score}"),
        Err(err) => eprintln!("{err}"),
    }
}

/* ----- Tests ----- */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_games() {
        assert_eq!(part1_game((Column1::A, Column2::Y)), 8); // Rock, me Paper 2, Win 6: 8
        assert_eq!(part1_game((Column1::B, Column2::X)), 1); // Paper, me Rock 1, Lose 0: 1
        assert_eq!(part1_game((Column1::C, Column2::Z)), 6); // Scissors, me Scissors 3, Draw 3: 6
    }

    #[test]
    fn test_part2_games() {
        assert_eq!(part2_game((Column1::A, Column2::Y)), 4); // Rock, Draw 3, me Rock 1: 4
        assert_eq!(part2_game((Column1::B, Column2::X)), 1); // Paper, Lose 0, me Rock 1: 1
        assert_eq!(part2_game((Column1::C, Column2::Z)), 7); // Scissors, Win 6, me Rock 1: 7
    }

    // Format to make it easier to test many games, but not taking full advantage of that yet

    static STRATS: [(Column1, Column2); 3] = [
        (Column1::A, Column2::Y),
        (Column1::B, Column2::X),
        (Column1::C, Column2::Z),
    ];

    #[test]
    fn test_part1_more_games() {
        assert_eq!(
            STRATS.into_iter().map(part1_game).collect::<Vec<_>>(),
            [8, 1, 6]
        );
    }

    #[test]
    fn test_part2_more_games() {
        assert_eq!(
            STRATS.into_iter().map(part2_game).collect::<Vec<_>>(),
            [4, 1, 7]
        );
    }
}
