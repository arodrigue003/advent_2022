use colored::Colorize;
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct GridWithBorder {
    data: Vec<Vec<u8>>,
    start: Point,
    end: Point,
    width: usize,
    height: usize,
}

impl GridWithBorder {
    pub fn from_str(data: &str) -> GridWithBorder {
        let mut start: Option<Point> = None;
        let mut end: Option<Point> = None;

        // Parse the data
        let data: Vec<Vec<u8>> = data
            .lines()
            .enumerate()
            .map(|(line, line_data)| {
                line_data
                    .chars()
                    .enumerate()
                    .map(|(column, char)| match char {
                        'S' => {
                            start = Some(Point { line, column });
                            0
                        }
                        'E' => {
                            end = Some(Point { line, column });
                            25
                        }
                        c => c as u8 - 97,
                    })
                    .collect()
            })
            .collect();

        let width = data[0].len();
        let height = data.len();

        // Add a border to it
        let data = std::iter::once(vec![u8::MAX; width + 2])
            .chain(data.into_iter().map(|line| {
                std::iter::once(u8::MAX)
                    .chain(line.into_iter())
                    .chain(std::iter::once(u8::MAX))
                    .collect()
            }))
            .chain(std::iter::once(vec![u8::MAX; width + 2]))
            .collect();

        GridWithBorder {
            data,
            start: start.unwrap(),
            end: end.unwrap(),
            width,
            height,
        }
    }

    #[inline(always)]
    pub fn height(&self) -> usize {
        self.height
    }

    #[inline(always)]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline(always)]
    pub fn start(&self) -> &Point {
        &self.start
    }

    #[inline(always)]
    pub fn end(&self) -> &Point {
        &self.end
    }

    #[inline(always)]
    pub fn get(&self, line: usize, column: usize) -> u8 {
        self.data[line + 1][column + 1]
    }

    pub fn get_neighbors(&self, line: usize, column: usize) -> Vec<Point> {
        // Compute coordinates to take borders into account
        let line = line + 1;
        let column = column + 1;

        let mut res = vec![];

        // get current value
        // val is never equal to u8::MAX because we are only looking for points inside the grid and
        // not on the border.
        let val = self.data[line][column];

        // check top neighbor
        if self.data[line - 1][column] <= val + 1 {
            res.push(Point {
                line: line - 2,
                column: column - 1,
            })
        }
        // check bottom neighbor
        if self.data[line + 1][column] <= val + 1 {
            res.push(Point {
                line,
                column: column - 1,
            })
        }
        // check left neighbor
        if self.data[line][column - 1] <= val + 1 {
            res.push(Point {
                line: line - 1,
                column: column - 2,
            })
        }
        // check right neighbor
        if self.data[line][column + 1] <= val + 1 {
            res.push(Point {
                line: line - 1,
                column,
            })
        }

        res
    }

    pub fn get_a_elevation_list(&self) -> Vec<Point> {
        let to_test: Vec<_> = (0..self.height)
            .into_iter()
            .flat_map(|line| {
                (0..self.width).into_iter().filter_map(move |column| {
                    if self.get(line, column) == 0 {
                        Some(Point { line, column })
                    } else {
                        None
                    }
                })
            })
            .collect();
        to_test
    }

    pub fn pretty_print(&self) {
        for line in 0..self.height {
            for column in 0..self.width {
                if line == self.start.line && column == self.start.column {
                    print!("S");
                } else if line == self.end.line && column == self.end.column {
                    print!("E");
                } else {
                    print!("{}", (self.get(line, column) + 97) as char);
                }
            }
            println!();
        }
        println!();
    }

    pub fn pretty_print_path(&self, path: &[Point]) {
        let points: HashSet<_> = path.iter().cloned().collect();

        for line in 0..self.height {
            for column in 0..self.width {
                // determinate if the point is on the result path
                let on_path = points.contains(&Point { line, column });

                if line == self.start.line && column == self.start.column {
                    if on_path {
                        print!("{}", "S".green().bold());
                    } else {
                        print!("S");
                    }
                } else if line == self.end.line && column == self.end.column {
                    if on_path {
                        print!("{}", "E".green().bold());
                    } else {
                        print!("E");
                    }
                } else {
                    if on_path {
                        print!(
                            "{}",
                            ((self.get(line, column) + 97) as char)
                                .to_string()
                                .green()
                                .bold()
                        );
                    } else {
                        print!("{}", (self.get(line, column) + 97) as char);
                    }
                }
            }
            println!();
        }
        println!();
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Point {
    pub line: usize,
    pub column: usize,
}
