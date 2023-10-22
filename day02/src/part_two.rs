#[derive(Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl From<char> for Shape {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::Rock,
            'B' => Self::Paper,
            'C' => Self::Scissors,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum Action {
    Lose,
    Draw,
    Win,
}

impl From<char> for Action {
    fn from(value: char) -> Self {
        match value {
            'X' => Self::Lose,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Turn {
    opponent: Shape,
    action: Action,
}

impl<'a> From<&'a str> for Turn {
    fn from(value: &'a str) -> Self {
        let chars: Vec<_> = value.chars().collect();

        Turn {
            opponent: From::from(chars[0]),
            action: From::from(chars[2]),
        }
    }
}

impl Turn {
    fn compute_score(&self) -> i32 {
        match (&self.opponent, &self.action) {
            // result + shape value
            (Shape::Rock, Action::Lose) => 0 + 3, // scissors loses to rock
            (Shape::Rock, Action::Draw) => 3 + 1, // rock draws to rock
            (Shape::Rock, Action::Win) => 6 + 2,  // paper wins to rock
            (Shape::Paper, Action::Lose) => 0 + 1, // rock loses to paper
            (Shape::Paper, Action::Draw) => 3 + 2, // paper draws to paper
            (Shape::Paper, Action::Win) => 6 + 3, // scissors wins to paper
            (Shape::Scissors, Action::Lose) => 0 + 2, // paper loses to scissors
            (Shape::Scissors, Action::Draw) => 3 + 3, // scissors draws to scissors
            (Shape::Scissors, Action::Win) => 6 + 1, // rock wins to scissors
        }
    }
}

pub fn solve_part_two(data: &str) {
    let res: i32 = data
        .lines()
        .map(|line| Turn::from(line).compute_score())
        .sum();

    println!("Part two solution: {:#?}", res);
}
