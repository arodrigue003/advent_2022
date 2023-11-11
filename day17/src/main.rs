use lazy_static::lazy_static;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::ops::{Index, IndexMut};
use std::{env, fs};

static GRID_HEIGHT: usize = 50_000;

struct Shape {
    shape: Vec<Point>,
    height: usize,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct FrontLine {
    start: usize,
    low: usize,
    end: usize,
}

lazy_static! {
    static ref SHAPES: [Shape; 5] = [
        Shape {
            shape: vec![
                Point { x: 0, y: 0 },
                Point { x: 1, y: 0 },
                Point { x: 2, y: 0 },
                Point { x: 3, y: 0 },
            ],
            height: 1
        },
        Shape {
            shape: vec![
                Point { x: 1, y: 0 },
                Point { x: 0, y: 1 },
                Point { x: 1, y: 1 },
                Point { x: 2, y: 1 },
                Point { x: 1, y: 2 },
            ],
            height: 3
        },
        Shape {
            shape: vec![
                Point { x: 0, y: 0 },
                Point { x: 1, y: 0 },
                Point { x: 2, y: 0 },
                Point { x: 2, y: 1 },
                Point { x: 2, y: 2 },
            ],
            height: 3
        },
        Shape {
            shape: vec![
                Point { x: 0, y: 0 },
                Point { x: 0, y: 1 },
                Point { x: 0, y: 2 },
                Point { x: 0, y: 3 },
            ],
            height: 4
        },
        Shape {
            shape: vec![
                Point { x: 0, y: 0 },
                Point { x: 1, y: 0 },
                Point { x: 0, y: 1 },
                Point { x: 1, y: 1 },
            ],
            height: 2
        }
    ];
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum FrontLineDirection {
    Left,
    Top,
    Right,
    Bot,
}

impl FrontLineDirection {
    fn get_left_and_front_positions(&self, position: &Point) -> (Option<Point>, Option<Point>) {
        match self {
            FrontLineDirection::Left => {
                let left = if position.y > 0 {
                    Some(Point {
                        x: position.x,
                        y: position.y - 1,
                    })
                } else {
                    None
                };
                let front = if position.x > 0 {
                    Some(Point {
                        x: position.x - 1,
                        y: position.y,
                    })
                } else {
                    None
                };
                (left, front)
            }
            FrontLineDirection::Top => {
                let left = if position.x > 0 {
                    Some(Point {
                        x: position.x - 1,
                        y: position.y,
                    })
                } else {
                    None
                };
                (
                    left,
                    Some(Point {
                        x: position.x,
                        y: position.y + 1,
                    }),
                )
            }
            FrontLineDirection::Right => {
                let front = if position.x < 6 {
                    Some(Point {
                        x: position.x + 1,
                        y: position.y,
                    })
                } else {
                    None
                };
                (
                    Some(Point {
                        x: position.x,
                        y: position.y + 1,
                    }),
                    front,
                )
            }
            FrontLineDirection::Bot => {
                let left = if position.x < 6 {
                    Some(Point {
                        x: position.x + 1,
                        y: position.y,
                    })
                } else {
                    None
                };
                let front = if position.y > 0 {
                    Some(Point {
                        x: position.x,
                        y: position.y - 1,
                    })
                } else {
                    None
                };
                (left, front)
            }
        }
    }

    fn turn_left(&self) -> Self {
        match self {
            FrontLineDirection::Left => FrontLineDirection::Bot,
            FrontLineDirection::Top => FrontLineDirection::Left,
            FrontLineDirection::Right => FrontLineDirection::Top,
            FrontLineDirection::Bot => FrontLineDirection::Right,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            FrontLineDirection::Left => FrontLineDirection::Top,
            FrontLineDirection::Top => FrontLineDirection::Right,
            FrontLineDirection::Right => FrontLineDirection::Bot,
            FrontLineDirection::Bot => FrontLineDirection::Left,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
enum GridState {
    Air,
    Rock,
}

struct Grid {
    grid: Vec<Vec<GridState>>,
    max_height: usize,
    y_offset: usize,
}

impl Grid {
    fn new() -> Self {
        let mut grid = vec![vec![GridState::Air; 7]; GRID_HEIGHT];
        for i in 0..7 {
            grid[0][i] = GridState::Rock
        }
        Self {
            grid,
            max_height: 1,
            y_offset: 0,
        }
    }

    fn pretty_print(&self, full: bool) {
        let starting_line = if full {
            0
        } else {
            if self.max_height - self.y_offset > 45 {
                self.max_height - self.y_offset - 45
            } else {
                0
            }
        };

        for i_line in (starting_line..self.max_height - self.y_offset + 5).rev() {
            let line = &self.grid[i_line];
            if i_line % 5 == 0 {
                print!("{:>6} ", i_line + self.y_offset);
            } else {
                print!("       ")
            }
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

    fn shift_grid(&mut self) {
        let front_line = self.compute_front_line();

        // If we cannot remove elements from the grid, we are fucked
        if front_line.start - self.y_offset == 0 || front_line.end - self.y_offset == 1 {
            // We can fix this by having the grid height being dynamic and just allocating
            // new lines without removing old ones.
            // We can also solve this issue by increasing the base height
            unimplemented!()
        }

        // Front line is the height of the start and end line in the grid.
        // We need to convert them to grid offset in order to perform our operation
        // We remove everything bellow the front_line
        let to_remove = front_line.start.min(front_line.end).min(front_line.low) - self.y_offset;
        self.grid.drain(0..to_remove);

        // We now add new lines above the front line
        self.grid
            .append(&mut vec![vec![GridState::Air; 7]; to_remove]);

        // We now increase the y_offset with the number of lines we removed
        self.y_offset += to_remove;
    }

    fn push(&self, shape: &Shape, shape_position: &mut Point, direction: Direction) {
        // special condition where the shape is already located at the left
        if shape_position.x == 0 && direction == Direction::Left {
            return;
        }

        for part in &shape.shape {
            let x_pos = match direction {
                Direction::Left => part.x + shape_position.x - 1,
                Direction::Right => part.x + shape_position.x + 1,
            };
            let y_pos = part.y + shape_position.y;
            if x_pos == 7 || self[(x_pos, y_pos)] == GridState::Rock {
                return;
            }
        }

        // Every check was ok, update shape position
        match direction {
            Direction::Left => shape_position.x -= 1,
            Direction::Right => shape_position.x += 1,
        }
    }

    fn fall(&mut self, shape: &Shape, shape_position: &mut Point) -> bool {
        for part in &shape.shape {
            let x_pos = part.x + shape_position.x;
            let y_pos = part.y + shape_position.y - 1;
            if self[(x_pos, y_pos)] == GridState::Rock {
                // Store the shape in the grid and return true
                for part in &shape.shape {
                    let x_pos = part.x + shape_position.x;
                    let y_pos = part.y + shape_position.y;
                    self[(x_pos, y_pos)] = GridState::Rock;
                }

                // Compute the new max height
                self.max_height = self.max_height.max(shape_position.y + shape.height);

                // Allocate more cases if necessary
                if GRID_HEIGHT + self.y_offset - self.max_height < 20 {
                    self.shift_grid();
                }

                return true;
            }
        }

        // Every check was ok, update shape position
        shape_position.y -= 1;
        false
    }

    fn compute_front_line(&self) -> FrontLine {
        // Get the position of the first element of the front line
        let mut starting = Point {
            x: 0,
            y: self.max_height,
        };
        // Get it done until we find a rock
        loop {
            if self[(starting.x, starting.y)] == GridState::Rock {
                break;
            }
            starting.y -= 1
        }

        // Keep the information about the front line min height and the path
        let mut path: Vec<Point> = vec![];

        // Try to reach the right wall
        let mut current = starting.clone();
        let mut direction = FrontLineDirection::Right;
        while current.x != 6 {
            let (left, front) = direction.get_left_and_front_positions(&current);

            // If we can go left, turn left and update the position and the direction
            if let Some(left) = left {
                if self[(left.x, left.y)] == GridState::Rock {
                    // If we are going back, remove the last entry from the path
                    if path.len() >= 2 && path[path.len() - 2] == current {
                        path.pop();
                    } else {
                        path.push(current);
                    }

                    current = left;
                    direction = direction.turn_left();
                    continue;
                }
            }

            // If we can go front, advance of one case
            if let Some(front) = front {
                if self[(front.x, front.y)] == GridState::Rock {
                    // If we are going back, remove the last entry from the path
                    if path.len() >= 2 && path[path.len() - 2] == current {
                        path.pop();
                    } else {
                        path.push(current);
                    }

                    current = front;
                    continue;
                }
            }

            // If we couldn't move, turn right
            direction = direction.turn_right();
        }

        FrontLine {
            start: starting.y,
            low: path.iter().map(|point| point.y).min().unwrap(),
            end: current.y,
        }
    }

    fn get_front_line_hash(&self) -> u64 {
        let front_line = self.compute_front_line();

        // Front line is the height of the start and end line in the grid.
        // We need to convert them to grid offset in order to perform our operation
        let min_height = front_line.start.min(front_line.end).min(front_line.low) - self.y_offset;
        let max_height = self.max_height - self.y_offset;
        let mut hasher = DefaultHasher::new();
        for y in min_height..max_height {
            self.grid[y].hash(&mut hasher);
        }
        hasher.finish()
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = GridState;

    #[inline(always)]
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.grid[index.1 - self.y_offset][index.0]
    }
}

impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.grid[index.1 - self.y_offset][index.0]
    }
}

fn solve_part_one(directions: &[Direction]) {
    let mut grid = Grid::new();
    let mut shape_iterator = SHAPES.iter().cycle();
    let mut fallen_shapes = 0;

    // Initialize shape and it's position
    let mut current_shape = shape_iterator.next().unwrap();
    let mut current_shape_pos = Point {
        x: 2,
        y: grid.max_height + 3,
    };

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

    grid.pretty_print(false);

    println!("Part one solution: {}", grid.max_height - 1);
}

fn solve_part_two(directions: &[Direction]) {
    let target = 1_000_000_000_000;

    let mut grid = Grid::new();
    let mut shape_iterator = SHAPES.iter().cycle();
    let mut fallen_shapes = 0;

    // Initialize shape and it's position
    let mut current_shape = shape_iterator.next().unwrap();
    let mut current_shape_pos = Point {
        x: 2,
        y: grid.max_height + 3,
    };

    let direction_len = directions.len();
    let mut step = 0;
    let shape_len = SHAPES.len();
    let mut step_shape = 0;

    let mut to_check: HashMap<(usize, usize, u64), (usize, usize)> = HashMap::new();
    let mut test = true;

    for direction in directions.iter().cycle() {
        // Push the shape with the wind
        grid.push(current_shape, &mut current_shape_pos, *direction);
        let is_stopped = grid.fall(current_shape, &mut current_shape_pos);
        if is_stopped {
            // Increase the count of fallen shapes
            fallen_shapes += 1;

            // If we reached the target, break the loop
            if fallen_shapes == target {
                break;
            }

            // If we reach a synchronisation point, print it
            if test {
                let new_check = (step, step_shape, grid.get_front_line_hash());
                if let Some((last_fallen_shapes, last_max_height)) = to_check.get(&new_check) {
                    let shape_skip = fallen_shapes - last_fallen_shapes;
                    let max_height_skip = grid.max_height - last_max_height;

                    // Compute the number of skip we can perform
                    let skip_count = (target - fallen_shapes) / shape_skip;

                    // Do the skip
                    fallen_shapes += skip_count * shape_skip;
                    grid.max_height += skip_count * max_height_skip;
                    grid.y_offset += skip_count * max_height_skip;
                    test = false;
                }
                to_check.insert(new_check, (fallen_shapes, grid.max_height));
            }

            // the shape reached a stop position, spawn a new one
            current_shape = shape_iterator.next().unwrap();
            step_shape = (step_shape + 1) % shape_len;
            current_shape_pos = Point {
                x: 2,
                y: grid.max_height + 3,
            };
        }

        step = (step + 1) % direction_len;
    }

    // grid.pretty_print(false);

    println!("Part two solution: {}", grid.max_height - 1);
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
    solve_part_two(&directions);
}
