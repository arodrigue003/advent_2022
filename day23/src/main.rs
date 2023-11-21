use clap::Parser;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug, Eq, PartialEq, Clone)]
struct Cli {
    /// Enable verbose display
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// File to parse
    #[arg(default_value = "input")]
    path: PathBuf,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    #[inline(always)]
    fn can_move(&self, neighbors: u8) -> bool {
        match self {
            Direction::North => (neighbors & 0b11100000) == 0,
            Direction::South => (neighbors & 0b00001110) == 0,
            Direction::West => (neighbors & 0b10000011) == 0,
            Direction::East => (neighbors & 0b00111000) == 0,
        }
    }

    #[inline(always)]
    fn iter(&self) -> &'static [Direction] {
        match self {
            Direction::North => &[
                Direction::North,
                Direction::South,
                Direction::West,
                Direction::East,
            ],
            Direction::South => &[
                Direction::South,
                Direction::West,
                Direction::East,
                Direction::North,
            ],
            Direction::West => &[
                Direction::West,
                Direction::East,
                Direction::North,
                Direction::South,
            ],
            Direction::East => &[
                Direction::East,
                Direction::North,
                Direction::South,
                Direction::West,
            ],
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Elf {
    line: i64,
    column: i64,
}

impl Elf {
    fn move_elf(&self, direction: &Direction) -> Elf {
        match direction {
            Direction::North => Elf {
                line: self.line - 1,
                column: self.column,
            },
            Direction::South => Elf {
                line: self.line + 1,
                column: self.column,
            },
            Direction::West => Elf {
                line: self.line,
                column: self.column - 1,
            },
            Direction::East => Elf {
                line: self.line,
                column: self.column + 1,
            },
        }
    }

    pub fn compute_new_position(
        &self,
        starting_direction: &Direction,
        current: &HashSet<Elf>,
    ) -> Elf {
        // Build elf scout array
        let neighbors = (current.contains(&Elf {
            line: self.line - 1,
            column: self.column - 1,
        }) as u8)
            << 7
            | (current.contains(&Elf {
                line: self.line - 1,
                column: self.column,
            }) as u8)
                << 6
            | (current.contains(&Elf {
                line: self.line - 1,
                column: self.column + 1,
            }) as u8)
                << 5
            | (current.contains(&Elf {
                line: self.line,
                column: self.column + 1,
            }) as u8)
                << 4
            | (current.contains(&Elf {
                line: self.line + 1,
                column: self.column + 1,
            }) as u8)
                << 3
            | (current.contains(&Elf {
                line: self.line + 1,
                column: self.column,
            }) as u8)
                << 2
            | (current.contains(&Elf {
                line: self.line + 1,
                column: self.column - 1,
            }) as u8)
                << 1
            | (current.contains(&Elf {
                line: self.line,
                column: self.column - 1,
            }) as u8);

        // If the elf is alone, do nothing
        if neighbors == 0 {
            return self.clone();
        }

        for direction in starting_direction.iter() {
            if direction.can_move(neighbors) {
                return self.move_elf(direction);
            }
        }

        self.clone()
    }
}

fn get_elfs_bounding_box(elfs: &HashSet<Elf>) -> (i64, i64, i64, i64) {
    let line_min = elfs.iter().map(|elf| elf.line).min().unwrap();
    let line_max = elfs.iter().map(|elf| elf.line).max().unwrap();
    let column_min = elfs.iter().map(|elf| elf.column).min().unwrap();
    let column_max = elfs.iter().map(|elf| elf.column).max().unwrap();
    (line_min, line_max, column_min, column_max)
}

pub fn print_elfs(elfs: &HashSet<Elf>) {
    let (line_min, line_max, column_min, column_max) = get_elfs_bounding_box(elfs);

    for line in line_min..=line_max {
        for column in column_min..=column_max {
            if elfs.contains(&Elf { line, column }) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn solve(elfs: &HashSet<Elf>, verbose: bool) {
    let mut current: HashSet<Elf> = elfs.clone();
    let mut next: HashSet<Elf> = HashSet::new();
    let mut movement: HashMap<Elf, Elf> = HashMap::new();
    let mut occupation: HashMap<Elf, usize> = HashMap::new();
    let mut round: usize = 0;
    let mut movement_count: usize = 0;

    for starting_direction in Direction::North.iter().iter().cycle() {
        // Print the grid for verbose debugging
        if verbose {
            println!("Game field after round: {}", round);
            print_elfs(&current);
        }

        // Fill the movement and and occupation hashmaps
        for elf in &current {
            let elf_new_position = elf.compute_new_position(starting_direction, &current);
            // Add the elf to the movement list
            movement.insert(elf.clone(), elf_new_position.clone());
            // Add the elf to the occupation hashmap
            *occupation.entry(elf_new_position).or_default() += 1;
        }

        // Update elf position in the next hashset if they can move
        for elf in &current {
            let elf_new_pos = &movement[elf];

            // Move it only if the target position is free
            if occupation[elf_new_pos] == 1 {
                next.insert(elf_new_pos.clone());

                // Increase movement_count if the elf moved
                if elf_new_pos != elf {
                    movement_count += 1;
                }
            } else {
                next.insert(elf.clone());
            }
        }

        // purge movement and occupation for the next round
        movement.drain();
        occupation.drain();

        // put next in current
        current = next;
        next = HashSet::new();

        // Increment round count
        round += 1;

        // If we reached the target for part 01, print the solution
        if round == 10 {
            let (line_min, line_max, column_min, column_max) = get_elfs_bounding_box(&current);
            let res =
                (line_max - line_min + 1) * (column_max - column_min + 1) - current.len() as i64;

            println!("Part one solution: {}", res);
        }

        // If we reached the target for part 02, print the solution and exit
        if movement_count == 0 {
            println!("Part two solution: {}", round);
            break;
        }

        // Reset movement count fot eh next round
        movement_count = 0;
    }
}

fn main() {
    let args = Cli::parse();

    let data: String = fs::read_to_string(&args.path).unwrap();
    let elfs: HashSet<Elf> = data
        .lines()
        .enumerate()
        .flat_map(|(i_line, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(i_column, tile)| match tile {
                    '.' => None,
                    '#' => Some(Elf {
                        line: i_line as i64,
                        column: i_column as i64,
                    }),
                    _ => unreachable!(),
                })
        })
        .collect();

    solve(&elfs, args.verbose);
}
