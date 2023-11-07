use crate::model::{Line, Point};
use itertools::Itertools;
use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum GridState {
    Air,
    Rock,
    Sand,
}

impl Display for GridState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GridState::Air => {
                write!(f, ".")
            }
            GridState::Rock => {
                write!(f, "#")
            }
            GridState::Sand => {
                write!(f, "o")
            }
        }
    }
}

enum SandUnitState {
    Falling,
    Blocked,
    FreeFalling,
    SourceBlocked,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Grid {
    grid: Vec<Vec<GridState>>,
    min_column: usize,
    max_column: usize,
    max_line: usize,
}

impl Grid {
    pub fn new(lines: &[Line], add_floor: bool) -> Self {
        // Get playground area
        // Add one to the border to facilitate further computations
        let max_line = lines
            .iter()
            .flat_map(|line| line.0.iter().map(|point| point.line))
            .max()
            .unwrap()
            + 2;

        let offset = if add_floor { max_line } else { 2 };

        let min_column = lines
            .iter()
            .flat_map(|line| line.0.iter().map(|point| point.column))
            .min()
            .unwrap()
            - offset;
        let max_column = lines
            .iter()
            .flat_map(|line| line.0.iter().map(|point| point.column))
            .max()
            .unwrap()
            + offset;

        // Create a grid with just enough pixels for the desired area
        let mut grid = vec![vec![GridState::Air; max_column + 1 - min_column]; max_line + 1];

        // Add the elements from the lines
        for line in lines {
            for (left, right) in line.0.iter().tuple_windows() {
                if left.line == right.line {
                    // Iterate over the column
                    for column in left.column.min(right.column)..=left.column.max(right.column) {
                        grid[left.line][column - min_column] = GridState::Rock;
                    }
                } else {
                    #[allow(clippy::needless_range_loop)]
                    for line in left.line.min(right.line)..=left.line.max(right.line) {
                        grid[line][left.column - min_column] = GridState::Rock;
                    }
                }
            }
        }

        // Eventually add the floor
        if add_floor {
            for column in min_column..=max_column {
                grid[max_line][column - min_column] = GridState::Rock;
            }
        }

        // Return the grid object
        Self {
            grid,
            min_column,
            max_column,
            max_line,
        }
    }

    pub fn pretty_print(&self) {
        // Print header
        for modulo in (0..3).rev() {
            let modulo: usize = 10usize.pow(modulo);
            print!("    ");
            for i in self.min_column..=self.max_column {
                if i == self.min_column || i == self.max_column || i % 5 == 0 {
                    print!("{}", (i % (modulo * 10) / modulo));
                } else {
                    print!(" ");
                }
            }
            println!();
        }

        for (line_number, line) in self.grid.iter().enumerate() {
            print!("{:>3} ", line_number);
            for pixel in line {
                print!("{}", pixel)
            }
            println!()
        }
    }

    fn get_sand_unit_state(&self, pos: &Point) -> SandUnitState {
        if pos.line == self.max_line {
            SandUnitState::FreeFalling
        } else if self.grid[pos.line + 1][pos.column - self.min_column - 1] != GridState::Air
            && self.grid[pos.line + 1][pos.column - self.min_column] != GridState::Air
            && self.grid[pos.line + 1][pos.column - self.min_column + 1] != GridState::Air
        {
            if pos.line == 0 && pos.column == 500 {
                SandUnitState::SourceBlocked
            } else {
                SandUnitState::Blocked
            }
        } else {
            SandUnitState::Falling
        }
    }

    /// Get the next position for the sand unit.
    /// This function considers that the sand unit is falling
    fn update_sand_unit_position(&self, position: &Point) -> Point {
        if self.grid[position.line + 1][position.column - self.min_column] == GridState::Air {
            Point {
                line: position.line + 1,
                column: position.column,
            }
        } else if self.grid[position.line + 1][position.column - self.min_column - 1]
            == GridState::Air
        {
            Point {
                line: position.line + 1,
                column: position.column - 1,
            }
        } else if self.grid[position.line + 1][position.column - self.min_column + 1]
            == GridState::Air
        {
            Point {
                line: position.line + 1,
                column: position.column + 1,
            }
        } else {
            unreachable!()
        }
    }

    /// Drop one unit of sand.
    /// Returns true if the unit was blocked.
    /// Returns false if the unit will be free falling or the source is blocked
    pub fn drop_sand_unit(&mut self) -> bool {
        let mut sand_unit_position = Point {
            line: 0,
            column: 500,
        };

        loop {
            let sand_unit_status = self.get_sand_unit_state(&sand_unit_position);

            match sand_unit_status {
                SandUnitState::Falling => {
                    // Update sand unit position
                    sand_unit_position = self.update_sand_unit_position(&sand_unit_position);
                }
                SandUnitState::Blocked => {
                    // Update the grid and return true because we found an equilibrium
                    self.grid[sand_unit_position.line]
                        [sand_unit_position.column - self.min_column] = GridState::Sand;
                    return true;
                }
                SandUnitState::FreeFalling | SandUnitState::SourceBlocked => {
                    // Sand is free falling, return false
                    return false;
                }
            }
        }
    }
}
