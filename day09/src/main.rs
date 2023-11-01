use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::{env, fs};

#[derive(Debug, Eq, PartialEq, Clone, Hash, Default)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{x:{}, y:{}}}", self.x, self.y)
    }
}

fn needs_to_move(a: &Point, b: &Point) -> bool {
    (b.x - a.x).abs() >= 2 || (b.y - a.y).abs() >= 2
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl<'a> From<&'a str> for Direction {
    fn from(value: &'a str) -> Self {
        match value {
            "U" => Self::Up,
            "D" => Self::Down,
            "R" => Self::Right,
            "L" => Self::Left,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Command {
    direction: Direction,
    steps: i64,
}

impl<'a> From<&'a str> for Command {
    fn from(value: &'a str) -> Self {
        let (dir, steps) = value.split_once(" ").unwrap();

        Self {
            direction: Direction::from(dir),
            steps: steps.parse().unwrap(),
        }
    }
}

fn solve_part_one(commands: &[Command]) {
    let mut head = Point::new();
    let mut tail = Point::new();
    let mut tail_positions: HashSet<Point> = HashSet::new();
    tail_positions.insert(tail.clone());

    for command in commands {
        for _ in 0..command.steps {
            // Move the head
            match command.direction {
                Direction::Up => head.y += 1,
                Direction::Down => head.y -= 1,
                Direction::Left => head.x -= 1,
                Direction::Right => head.x += 1,
            }

            if needs_to_move(&tail, &head) {
                if tail.x < head.x {
                    tail.x += 1
                } else if tail.x > head.x {
                    tail.x -= 1
                }

                if tail.y < head.y {
                    tail.y += 1
                } else if tail.y > head.y {
                    tail.y -= 1
                }
            }

            // Add the point to the point history
            tail_positions.insert(tail.clone());
        }
    }

    println!("Part one solution: {}", tail_positions.len());
}

fn solve_part_two(commands: &[Command]) {
    let mut knots: [Point; 10] = Default::default();
    let mut tail_positions: HashSet<Point> = HashSet::new();
    tail_positions.insert(knots[9].clone());

    for command in commands {
        for _ in 0..command.steps {
            // Move the head
            match command.direction {
                Direction::Up => knots[0].y += 1,
                Direction::Down => knots[0].y -= 1,
                Direction::Left => knots[0].x -= 1,
                Direction::Right => knots[0].x += 1,
            }

            // for each knot following in the line
            for i_knot in 0..9 {
                if needs_to_move(&knots[i_knot + 1], &knots[i_knot]) {
                    if &knots[i_knot + 1].x < &knots[i_knot].x {
                        knots[i_knot + 1].x += 1
                    } else if &knots[i_knot + 1].x > &knots[i_knot].x {
                        knots[i_knot + 1].x -= 1
                    }

                    if &knots[i_knot + 1].y < &knots[i_knot].y {
                        knots[i_knot + 1].y += 1
                    } else if &knots[i_knot + 1].y > &knots[i_knot].y {
                        knots[i_knot + 1].y -= 1
                    }
                }
            }

            // Add the point to the point history
            tail_positions.insert(knots[9].clone());
        }
    }

    println!("Part one solution: {:#?}", tail_positions.len());
}

fn main() {
    let args: Vec<_> = env::args().collect();

    let file_path = if args.len() > 1 {
        args[1].clone()
    } else {
        "input".to_string()
    };

    let data: String = fs::read_to_string(&file_path).unwrap();
    let commands: Vec<Command> = data.lines().map(From::from).collect();

    solve_part_one(&commands);
    solve_part_two(&commands);
}
