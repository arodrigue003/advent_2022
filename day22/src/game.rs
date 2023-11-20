use crate::enums::MapTile;
use crate::enums::{Command, Direction};
use crate::parser;
use crate::structs::{GotoLine, GotoLinePair, Point, Position};
use std::collections::{HashMap, HashSet};

const CUBE_DETECTION_VEC_WIDTH: usize = 6;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Game {
    map: Vec<Vec<MapTile>>,
    width: usize,
    height: usize,
    face_width: usize,
    pub start: Position,
    pub path: Vec<Command>,
    goto: HashMap<Position, Position>,
}

impl Game {
    pub fn new(data: &str, face_width: usize) -> Self {
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
                    .chain(line)
                    .chain(vec![MapTile::Void; width - len])
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
        let (res, path) = parser::parse_command_line(data.lines().next_back().unwrap()).unwrap();
        if !res.is_empty() {
            panic!("Unable to fully parse the input: {}", res);
        }

        Self {
            map: grid,
            width: width + 2,
            height: height + 2,
            face_width,
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

    pub fn add_part_two_goto(&mut self) {
        // Create a array to store face position
        // We choose 6 as the size because a cube pattern takes at most 4 cases and we want margin
        // around that in order to simplify the computation
        let mut faces: Vec<Vec<bool>> =
            vec![vec![false; CUBE_DETECTION_VEC_WIDTH]; CUBE_DETECTION_VEC_WIDTH];

        // Iterate over the shape in order to detect faces
        for line in 0..((self.height - 2) / self.face_width) {
            for column in 0..((self.width - 2) / self.face_width) {
                if self.map[line * self.face_width + 1][column * self.face_width + 1]
                    != MapTile::Void
                {
                    faces[line + 1][column + 1] = true
                }
            }
        }

        // We must have 6 faces
        if faces
            .iter()
            .flat_map(|line| line.iter())
            .filter(|tile| **tile)
            .count()
            != 6
        {
            panic!("A cube must have 6 faces.")
        }

        // Build a list of GotoLine in order look into them after
        let mut goto_lines_hashset: HashSet<GotoLine> = HashSet::new();
        for line in 1..CUBE_DETECTION_VEC_WIDTH - 1 {
            for column in 1..CUBE_DETECTION_VEC_WIDTH - 1 {
                if faces[line][column] {
                    // Check if we can create an horizontal line on top
                    if !faces[line - 1][column] {
                        goto_lines_hashset.insert(GotoLine {
                            start: Point {
                                line: (line - 1) * self.face_width,
                                column: (column - 1) * self.face_width + 1,
                            },
                            end: Point {
                                line: (line - 1) * self.face_width,
                                column: column * self.face_width,
                            },
                        });
                    }
                    // Check if we can create an horizontal line on the bottom
                    if !faces[line + 1][column] {
                        goto_lines_hashset.insert(GotoLine {
                            start: Point {
                                line: line * self.face_width + 1,
                                column: (column - 1) * self.face_width + 1,
                            },
                            end: Point {
                                line: line * self.face_width + 1,
                                column: column * self.face_width,
                            },
                        });
                    }
                    // Check if we can create a vertical line on the left part
                    if !faces[line][column - 1] {
                        goto_lines_hashset.insert(GotoLine {
                            start: Point {
                                line: (line - 1) * self.face_width + 1,
                                column: (column - 1) * self.face_width,
                            },
                            end: Point {
                                line: line * self.face_width,
                                column: (column - 1) * self.face_width,
                            },
                        });
                    }
                    // Check if we can create a vertical line on the right part
                    if !faces[line][column + 1] {
                        goto_lines_hashset.insert(GotoLine {
                            start: Point {
                                line: (line - 1) * self.face_width + 1,
                                column: column * self.face_width + 1,
                            },
                            end: Point {
                                line: line * self.face_width,
                                column: column * self.face_width + 1,
                            },
                        });
                    }
                }
            }
        }

        // Build a hashmap for search efficiency
        let mut goto_lines_hashmap: HashMap<Point, Vec<GotoLine>> = HashMap::new();
        for goto_line in &goto_lines_hashset {
            // Add both start and end
            goto_lines_hashmap
                .entry(goto_line.start.clone())
                .or_default()
                .push(goto_line.clone());
            goto_lines_hashmap
                .entry(goto_line.end.clone())
                .or_default()
                .push(goto_line.clone());
        }

        // Store formed pairs
        let mut goto_lines_pairs: Vec<GotoLinePair> = vec![];

        // 1. Look for goto lines that have a point in common
        for goto_lines in goto_lines_hashmap.values() {
            if goto_lines.len() == 1 {
                continue;
            }
            if goto_lines.len() > 2 || goto_lines.is_empty() {
                unreachable!("This cannot exists by construction!");
            }

            // We know here that goto_lines.len() == 2
            let first = &goto_lines[0];
            let second = &goto_lines[1];

            // Remove them from the goto lines hashset
            let first_was_removed = goto_lines_hashset.remove(first);
            let second_was_remove = goto_lines_hashset.remove(second);
            if !first_was_removed || !second_was_remove {
                continue;
            }

            // Construct the goto_line_pair in the correct order
            if first.start == second.start {
                goto_lines_pairs.push(GotoLinePair {
                    first: first.clone(),
                    second: second.clone(),
                });
            } else if first.start == second.end {
                goto_lines_pairs.push(GotoLinePair {
                    first: first.clone(),
                    second: second.clone().rev(),
                });
            } else if first.end == second.start {
                goto_lines_pairs.push(GotoLinePair {
                    first: first.clone().rev(),
                    second: second.clone(),
                });
            } else if first.end == second.end {
                goto_lines_pairs.push(GotoLinePair {
                    first: first.clone().rev(),
                    second: second.clone().rev(),
                })
            } else {
                unreachable!();
            }
        }

        // 2. We try to expand from the pairs
        let mut goto_lines_pairs_to_handle = goto_lines_pairs.clone();
        while let Some(goto_lines_pair) = goto_lines_pairs_to_handle.pop() {
            // Check if we have a line that continue with one that turns
            if Self::try_form_pair(
                &mut goto_lines_hashset,
                &mut goto_lines_hashmap,
                &mut goto_lines_pairs_to_handle,
                &mut goto_lines_pairs,
                goto_lines_pair.first.next_point_forward(),
                goto_lines_pair.second.next_point_right_rotation(),
            ) {
                continue;
            }
            if Self::try_form_pair(
                &mut goto_lines_hashset,
                &mut goto_lines_hashmap,
                &mut goto_lines_pairs_to_handle,
                &mut goto_lines_pairs,
                goto_lines_pair.first.next_point_forward(),
                goto_lines_pair.second.next_point_left_rotation(),
            ) {
                continue;
            }
            if Self::try_form_pair(
                &mut goto_lines_hashset,
                &mut goto_lines_hashmap,
                &mut goto_lines_pairs_to_handle,
                &mut goto_lines_pairs,
                goto_lines_pair.first.next_point_right_rotation(),
                goto_lines_pair.second.next_point_forward(),
            ) {
                continue;
            }
            if Self::try_form_pair(
                &mut goto_lines_hashset,
                &mut goto_lines_hashmap,
                &mut goto_lines_pairs_to_handle,
                &mut goto_lines_pairs,
                goto_lines_pair.first.next_point_left_rotation(),
                goto_lines_pair.second.next_point_forward(),
            ) {
                continue;
            }
        }

        if !goto_lines_hashset.is_empty() {
            // Here, we should theoretically only have two elements left in the hashset, we only
            // have to assemble them in a
            panic!("We didn't planned for this use case.")
        }

        if goto_lines_pairs.len() != 7 {
            panic!("We should have 7 pairs to connect.")
        }

        self.goto = goto_lines_pairs
            .into_iter()
            .flat_map(|goto_line_pair| goto_line_pair.get_goto(self.face_width))
            .collect();
    }

    fn try_form_pair(
        goto_lines_hashset: &mut HashSet<GotoLine>,
        goto_lines_hashmap: &mut HashMap<Point, Vec<GotoLine>>,
        goto_lines_pairs_to_handle: &mut Vec<GotoLinePair>,
        goto_lines_pairs: &mut Vec<GotoLinePair>,
        first_next: Option<Point>,
        second_next: Option<Point>,
    ) -> bool {
        let first_next = match first_next {
            None => return false,
            Some(first_next) => first_next,
        };
        let second_next = match second_next {
            None => return false,
            Some(second_next) => second_next,
        };

        if let Some(first_lines) = goto_lines_hashmap.get(&first_next) {
            if let Some(second_lines) = goto_lines_hashmap.get(&second_next) {
                // Get the elements from the vecs
                if first_lines.len() != 1 || second_lines.len() != 1 {
                    unreachable!("This cannot exists by construction")
                }

                let first = &first_lines[0];
                let second = &second_lines[0];

                // Remove them from the goto lines hashset
                let first_was_removed = goto_lines_hashset.remove(first);
                let second_was_remove = goto_lines_hashset.remove(second);
                if !first_was_removed || !second_was_remove {
                    return false;
                }

                // Add them to the goto_lines_pairs in the correct order
                let first = if first.start == first_next {
                    first.clone()
                } else {
                    first.clone().rev()
                };
                let second = if second.start == second_next {
                    second.clone()
                } else {
                    second.clone().rev()
                };
                let goto_line_pair = GotoLinePair { first, second };

                goto_lines_pairs_to_handle.push(goto_line_pair.clone());
                goto_lines_pairs.push(goto_line_pair);

                return true;
            }
        }
        false
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
            MapTile::Void => self
                .goto
                .get(&next)
                .expect("The add goto function did not work properly and needs to be fixed.")
                .clone(),
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
