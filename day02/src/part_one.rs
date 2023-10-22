#[derive(Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl From<char> for Shape {
    fn from(value: char) -> Self {
        match value {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Turn {
    opponent: Shape,
    action: Shape,
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
            (Shape::Rock, Shape::Rock) => 3 + 1,
            (Shape::Rock, Shape::Paper) => 6 + 2,
            (Shape::Rock, Shape::Scissors) => 0 + 3,
            (Shape::Paper, Shape::Rock) => 0 + 1,
            (Shape::Paper, Shape::Paper) => 3 + 2,
            (Shape::Paper, Shape::Scissors) => 6 + 3,
            (Shape::Scissors, Shape::Rock) => 6 + 1,
            (Shape::Scissors, Shape::Paper) => 0 + 2,
            (Shape::Scissors, Shape::Scissors) => 3 + 3,
        }
    }
}

pub fn solve_part_one(data: &str) {
    let res: i32 = data
        .lines()
        .map(|line| Turn::from(line).compute_score())
        .sum();

    println!("Part one solution: {:#?}", res);
}
