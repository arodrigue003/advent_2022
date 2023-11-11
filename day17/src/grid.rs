use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::ops::{Index, IndexMut};

use crate::front_line::{FrontLine, FrontLineDirection};
use crate::model::{Direction, Point, Shape, SHAPES};

/// Grid buffer size in number of lines
static GRID_HEIGHT: usize = 1_000;

/// State of a grid element
#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum GridState {
    Air,
    Rock,
}

/// Grid structure
pub struct Grid {
    grid: Vec<Vec<GridState>>,
    max_height: usize,
    y_offset: usize,
}

impl Grid {
    /// Create a grid with the predefined buffer size. Add a line of rock to simplify further
    /// computation.
    pub fn new() -> Self {
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

    /// Pretty print a grid
    ///
    /// If full is false only display the last 45 lines of shapes.
    pub fn pretty_print(&self, full: bool) {
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

    /// We call front line a continuous set of rock that start on the left part of the grid
    /// and ends on the right one.
    ///
    /// This information is interesting because if we know the y position of the start, the end
    /// and the lowest point of this path we are sure that whatever shape come into the grid it
    /// will not be possible for it to go bellow the front line.
    ///
    /// This property allows us to discard everything that is bellow the front line without  
    /// impacting how the simulation works.
    ///
    /// The algorithm is a simple labyrinth traversal algorithm:
    ///  * We first take the highest rock element on the first column.
    ///  * We face right and for each step, we try to:
    ///    * for to the case at the left position of the direction we are facing if it's a rock ;
    ///    * for to the case at the right position of the direction we are facing if it's a rock ;
    ///    * rotate right of 90 degrees.
    ///  * We stop when we reach the right column
    pub fn compute_front_line(&self) -> FrontLine {
        // Get the position of the first element of the front line
        let mut starting = Point {
            x: 0,
            y: self.max_height,
        };
        loop {
            if self[(starting.x, starting.y)] == GridState::Rock {
                break;
            }
            starting.y -= 1
        }

        // Front line path
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

    /// We already know that after having calculated the three eights of the front line, it is not
    /// possible for a new shape to go bellow this line.
    ///
    /// knowing this, we can now uniquely identify this front line by computing the hash of the
    /// position of every rock that is above the lowest point of the front line. This hash will
    /// later be used in order to be able to skip simulation step by detecting two similar
    /// configurations.
    pub fn get_front_line_hash(&self) -> u64 {
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

    /// Since everything that is under the front line is useless for the simulation, we can
    /// periodically remove everything that is bellow it without impacting the simulation.
    ///
    /// This allows us to keep a constant size for the grid line buffer by putting the front line
    /// at the start of the internal grid and by storing the offset to the front line.
    ///
    /// This function implement this shift operation.
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

    /// Push the shape according to the wind direction.
    pub fn push(&self, shape: &Shape, shape_position: &mut Point, direction: Direction) {
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

    /// Make the shape fall of one case if possible.
    pub fn fall(&mut self, shape: &Shape, shape_position: &mut Point) -> bool {
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

    /// Simulation the falling of 'target' shapes.
    ///
    /// In order to optimize the computation, we look for two moments in the simulation where the
    /// configuration is identical.
    ///
    /// We can prove that if after having making a shape fall, we find a couple (front line,
    /// shape index, direction index) that was already found before we have detected a pattern.
    /// Shape index refers to the index of the shape that just fell.
    /// Direction index refer to the index of the current wind direction.
    ///
    /// Once we found this pattern we can compute the number of shapes (n_shape) that have fallen
    /// between theses two moment and the height gained from theses shapes falling (n_height). If
    /// we add n_shape to the number of shapes that have already fallen and if we add n_height to
    /// the grid offset as well as the grid max height, we skipped the simulation computation
    /// of theses n_shapes while keeping a correct grid topology to resume the grid simulation
    /// after that.
    ///
    /// If we compute how much time we can do this skip and directly offset the three values
    /// mentioned before, we can approach very close to the target shape count. We just have to
    /// finish the simulation in order to get the score.
    pub fn simulate_falling(&mut self, directions: &[Direction], target: usize) -> usize {
        let mut shape_iterator = SHAPES.iter().cycle();
        let mut fallen_shapes = 0;

        // Initialize shape and it's position
        let mut current_shape = shape_iterator.next().unwrap();
        let mut current_shape_pos = Point {
            x: 2,
            y: self.max_height + 3,
        };

        let direction_len = directions.len();
        let mut step = 0;
        let shape_len = SHAPES.len();
        let mut step_shape = 0;

        let mut to_check: HashMap<(usize, usize, u64), (usize, usize)> = HashMap::new();
        let mut test = true;

        for direction in directions.iter().cycle() {
            // Push the shape with the wind
            self.push(current_shape, &mut current_shape_pos, *direction);
            let is_stopped = self.fall(current_shape, &mut current_shape_pos);
            if is_stopped {
                // Increase the count of fallen shapes
                fallen_shapes += 1;

                // If we reached the target, break the loop
                if fallen_shapes == target {
                    return self.max_height - 1;
                }

                // If we reach a synchronisation point, skip as much steps as possible
                if test {
                    let new_check = (step, step_shape, self.get_front_line_hash());
                    if let Some((last_fallen_shapes, last_max_height)) = to_check.get(&new_check) {
                        let shape_skip = fallen_shapes - last_fallen_shapes;
                        let max_height_skip = self.max_height - last_max_height;

                        // Compute the number of skip we can perform
                        let skip_count = (target - fallen_shapes) / shape_skip;

                        // Do the skip
                        fallen_shapes += skip_count * shape_skip;
                        self.max_height += skip_count * max_height_skip;
                        self.y_offset += skip_count * max_height_skip;
                        test = false;
                    }
                    to_check.insert(new_check, (fallen_shapes, self.max_height));
                }

                // the shape reached a stop position, spawn a new one
                current_shape = shape_iterator.next().unwrap();
                step_shape = (step_shape + 1) % shape_len;
                current_shape_pos = Point {
                    x: 2,
                    y: self.max_height + 3,
                };
            }

            step = (step + 1) % direction_len;
        }

        unreachable!()
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
