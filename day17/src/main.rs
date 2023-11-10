use lazy_static::lazy_static;
use std::{env, fs};

lazy_static! {
    static ref SHAPES: [Vec<Point>; 5] = [
        vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 2, y: 0 },
            Point { x: 3, y: 0 },
        ],
        vec![
            Point { x: 1, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 },
            Point { x: 2, y: 1 },
            Point { x: 1, y: 2 },
        ],
        vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 2, y: 0 },
            Point { x: 2, y: 1 },
            Point { x: 2, y: 2 },
        ],
        vec![
            Point { x: 0, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 0, y: 2 },
            Point { x: 0, y: 3 },
        ],
        vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 },
        ],
    ];
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum GridState {
    Air,
    Rock,
}

struct Grid {
    grid: Vec<Vec<GridState>>,
    max_height: i32,
}

impl Grid {
    fn new() -> Self {
        Self {
            grid: vec![vec![GridState::Air; 7]; 10_000],
            max_height: 0,
        }
    }

    fn pretty_print(&self) {
        for i_line in (0..self.max_height + 5).rev() {
            let line = &self.grid[i_line as usize];
            for column in line {
                print!(
                    "{}",
                    match column {
                        GridState::Air => ".",
                        GridState::Rock => "#",
                    }
                );
            }
            println!()
        }
    }

    fn push(&self, shape: &Vec<Point>, shape_position: &mut Point, direction: Direction) {
        // Depending of the direction, check is the shape will be locked or not,
        let offset = match direction {
            Direction::Left => -1,
            Direction::Right => 1,
        };

        for part in shape {
            let x_pos = part.x + shape_position.x + offset;
            let y_pos = part.y + shape_position.y;
            if x_pos == -1
                || x_pos == 7
                || self.grid[y_pos as usize][x_pos as usize] == GridState::Rock
            {
                return;
            }
        }

        // Every check was ok, update shape position
        shape_position.x += offset;
    }

    fn fall(&mut self, shape: &Vec<Point>, shape_position: &mut Point) -> bool {
        for part in shape {
            let x_pos = part.x + shape_position.x;
            let y_pos = part.y + shape_position.y - 1;
            if y_pos == -1 || self.grid[y_pos as usize][x_pos as usize] == GridState::Rock {
                // Store the shape in the grid and return true
                for part in shape {
                    let x_pos = part.x + shape_position.x;
                    let y_pos = part.y + shape_position.y;
                    self.grid[y_pos as usize][x_pos as usize] = GridState::Rock;
                }

                // Compute the new max height
                self.max_height = self.max_height.max(
                    shape
                        .iter()
                        .map(|part| part.y + shape_position.y + 1)
                        .max()
                        .unwrap(),
                );

                return true;
            }
        }

        // Every check was ok, update shape position
        shape_position.y -= 1;
        false
    }
}

fn solve_part_one(directions: &[Direction]) {
    let mut grid = Grid::new();
    let mut shape_iterator = SHAPES.iter().cycle();
    let mut fallen_shapes = 0;

    // Initialize shape and it's position
    let mut current_shape = shape_iterator.next().unwrap();
    let mut current_shape_pos = Point { x: 2, y: 3 };

    for direction in directions.iter().cycle() {
        // Push the shape with the wind
        grid.push(current_shape, &mut current_shape_pos, *direction);
        let is_stopped = grid.fall(current_shape, &mut current_shape_pos);
        if is_stopped {
            // Increase the count of fallen shapes
            fallen_shapes += 1;

            // If we reached the target, break the loop
            if fallen_shapes == 2022 {
                break;
            }

            // the shape reached a stop position, spawn a new one
            current_shape = shape_iterator.next().unwrap();
            current_shape_pos = Point {
                x: 2,
                y: grid.max_height + 3,
            }
        }
    }

    // grid.pretty_print();

    println!("Part one solution: {}", grid.max_height);
}

fn solve_part_two(data: &str) {
    println!("Part two solution:");
}

fn main() {
    let args: Vec<_> = env::args().collect();

    let file_path = if args.len() > 1 {
        args[1].clone()
    } else {
        "input".to_string()
    };

    let data = fs::read_to_string(&file_path).unwrap();
    let directions: Vec<_> = data
        .chars()
        .filter_map(|char| match char {
            '<' => Some(Direction::Left),
            '>' => Some(Direction::Right),
            '\n' | '\r' => None,
            _ => unreachable!(),
        })
        .collect();

    solve_part_one(&directions);
    solve_part_two(&data);
}
