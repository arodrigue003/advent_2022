use crate::parser;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

impl Direction {
    pub fn rotate(&self, rotation: Rotation) -> Self {
        match rotation {
            Rotation::Left => match self {
                Direction::Top => Direction::Left,
                Direction::Right => Direction::Top,
                Direction::Bottom => Direction::Right,
                Direction::Left => Direction::Bottom,
            },
            Rotation::Right => match self {
                Direction::Top => Direction::Right,
                Direction::Right => Direction::Bottom,
                Direction::Bottom => Direction::Left,
                Direction::Left => Direction::Top,
            },
        }
    }

    pub fn value(&self) -> usize {
        match self {
            Direction::Top => 3,
            Direction::Right => 0,
            Direction::Bottom => 1,
            Direction::Left => 2,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum MapTile {
    Void,
    Open,
    Wall,
}

impl Display for MapTile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MapTile::Void => {
                    " "
                }
                MapTile::Open => {
                    "."
                }
                MapTile::Wall => {
                    "#"
                }
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Rotation {
    Left,
    Right,
}

impl Display for Rotation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Rotation::Left => "L",
                Rotation::Right => "R",
            }
        )
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Game {
    map: Vec<Vec<MapTile>>,
    width: usize,
    height: usize,
    pub start: Position,
    pub path: Vec<Command>,
    goto: HashMap<Position, Position>,
}

impl Game {
    pub fn new(data: &str) -> Self {
        let grid: Vec<Vec<_>> = data
            .lines()
            .map_while(|line| {
                if line.is_empty() {
                    None
                } else {
                    Some(
                        line.chars()
                            .map(|char| match char {
                                ' ' => MapTile::Void,
                                '.' => MapTile::Open,
                                '#' => MapTile::Wall,
                                _ => unreachable!(),
                            })
                            .collect(),
                    )
                }
            })
            .collect();

        // Get the dimension
        let width = grid.iter().map(|line| line.len()).max().unwrap();
        let height = grid.len();

        // Add a border to the map
        let grid: Vec<Vec<_>> = std::iter::once(vec![MapTile::Void; width + 2])
            .chain(grid.into_iter().map(|line| {
                let len = line.len();
                std::iter::once(MapTile::Void)
                    .chain(line.into_iter())
                    .chain(vec![MapTile::Void; width - len].into_iter())
                    .chain(std::iter::once(MapTile::Void))
                    .collect()
            }))
            .chain(std::iter::once(vec![MapTile::Void; width + 2]))
            .collect();

        // Get starting position
        let start = Position {
            line: 1,
            column: grid[1]
                .iter()
                .position(|tile| *tile == MapTile::Open)
                .unwrap(),
            direction: Direction::Right,
        };

        // Get the path
        let (res, path) = parser::parse_command_line(data.lines().rev().next().unwrap()).unwrap();
        if !res.is_empty() {
            panic!("Unable to fully parse the input: {}", res);
        }

        Self {
            map: grid,
            width: width + 2,
            height: height + 2,
            start,
            path,
            goto: HashMap::new(),
        }
    }

    pub fn add_part_one_goto(&mut self) {
        let mut goto: HashMap<Position, Position> = HashMap::new();

        // Add goto options for lines
        for (i_line, line) in self.map.iter().enumerate() {
            let start = match line
                .iter()
                .position(|tile| *tile == MapTile::Wall || *tile == MapTile::Open)
            {
                None => continue,
                Some(start) => start,
            };
            let end = match line
                .iter()
                .rev()
                .position(|tile| *tile == MapTile::Wall || *tile == MapTile::Open)
            {
                None => continue,
                Some(end) => line.len() - end - 1,
            };

            // Fill start and end with value depending of what is at the other side
            goto.insert(
                Position {
                    line: i_line,
                    column: start - 1,
                    direction: Direction::Left,
                },
                Position {
                    line: i_line,
                    column: end,
                    direction: Direction::Left,
                },
            );
            goto.insert(
                Position {
                    line: i_line,
                    column: end + 1,
                    direction: Direction::Right,
                },
                Position {
                    line: i_line,
                    column: start,
                    direction: Direction::Right,
                },
            );
        }

        // Add goto options for columns
        for i_column in 0..self.width {
            let mut start = 0;
            for i_line in 0..self.height {
                if self.map[i_line][i_column] == MapTile::Wall
                    || self.map[i_line][i_column] == MapTile::Open
                {
                    start = i_line;
                    break;
                }
            }
            let mut end = 0;
            for i_line in (0..self.height).rev() {
                if self.map[i_line][i_column] == MapTile::Wall
                    || self.map[i_line][i_column] == MapTile::Open
                {
                    end = i_line;
                    break;
                }
            }
            if start == 0 && end == 0 {
                continue;
            }

            // Fill start and end with value depending of what is at the other side
            goto.insert(
                Position {
                    line: start - 1,
                    column: i_column,
                    direction: Direction::Top,
                },
                Position {
                    line: end,
                    column: i_column,
                    direction: Direction::Top,
                },
            );
            goto.insert(
                Position {
                    line: end + 1,
                    column: i_column,
                    direction: Direction::Bottom,
                },
                Position {
                    line: start,
                    column: i_column,
                    direction: Direction::Bottom,
                },
            );
        }

        // Update goto
        self.goto = goto;
    }

    pub fn go_forward(&self, current: &Position) -> Position {
        let next = match current.direction {
            Direction::Top => Position {
                line: current.line - 1,
                column: current.column,
                direction: current.direction,
            },
            Direction::Right => Position {
                line: current.line,
                column: current.column + 1,
                direction: current.direction,
            },
            Direction::Bottom => Position {
                line: current.line + 1,
                column: current.column,
                direction: current.direction,
            },
            Direction::Left => Position {
                line: current.line,
                column: current.column - 1,
                direction: current.direction,
            },
        };

        match self.map[next.line][next.column] {
            MapTile::Void => self.goto.get(&next).unwrap().clone(),
            MapTile::Open | MapTile::Wall => next,
        }
    }

    pub fn get_tile(&self, current: &Position) -> MapTile {
        self.map[current.line][current.column]
    }

    pub fn pretty_print(&self) {
        for (i_line, line) in self.map.iter().enumerate() {
            for (i_column, tile) in line.iter().enumerate() {
                let hor = self.goto.contains_key(&Position {
                    line: i_line,
                    column: i_column,
                    direction: Direction::Left,
                }) || self.goto.contains_key(&Position {
                    line: i_line,
                    column: i_column,
                    direction: Direction::Right,
                });
                let ver = self.goto.contains_key(&Position {
                    line: i_line,
                    column: i_column,
                    direction: Direction::Top,
                }) || self.goto.contains_key(&Position {
                    line: i_line,
                    column: i_column,
                    direction: Direction::Bottom,
                });
                match (hor, ver) {
                    (false, false) => print!("{}", tile),
                    (false, true) => print!("-"),
                    (true, false) => print!("|"),
                    (true, true) => print!("+"),
                }
            }
            println!();
        }

        println!();

        // Show path
        for path in &self.path {
            match path {
                Command::Forward(count) => {
                    print!("{}", count)
                }
                Command::Rotate(rotation) => {
                    print!("{}", rotation)
                }
            }
        }
        println!();

        println!();

        // Show the starting position
        println!("Starting at {:?}", self.start);
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Position {
    pub line: usize,
    pub column: usize,
    pub direction: Direction,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Command {
    Forward(usize),
    Rotate(Rotation),
}
