use crate::enums::Direction;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Position {
    pub line: usize,
    pub column: usize,
    pub direction: Direction,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Point {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct GotoLine {
    pub start: Point,
    pub end: Point,
}

impl GotoLine {
    pub fn rev(self) -> Self {
        Self {
            start: self.end,
            end: self.start,
        }
    }

    pub fn next_point_forward(&self) -> Option<Point> {
        if self.start.line == self.end.line {
            if self.start.column < self.end.column {
                Some(Point {
                    line: self.start.line,
                    column: self.end.column + 1,
                })
            } else if self.end.column == 0 {
                None
            } else {
                Some(Point {
                    line: self.start.line,
                    column: self.end.column - 1,
                })
            }
        } else if self.start.column == self.end.column {
            if self.start.line < self.end.line {
                Some(Point {
                    line: self.end.line + 1,
                    column: self.start.column,
                })
            } else if self.end.line == 0 {
                None
            } else {
                Some(Point {
                    line: self.end.line - 1,
                    column: self.start.column,
                })
            }
        } else {
            unreachable!()
        }
    }

    pub fn next_point_right_rotation(&self) -> Option<Point> {
        if self.start.line == self.end.line {
            if self.start.column < self.end.column {
                Some(Point {
                    line: self.start.line + 1,
                    column: self.end.column + 1,
                })
            } else if self.start.line == 0 || self.end.column == 0 {
                None
            } else {
                Some(Point {
                    line: self.start.line - 1,
                    column: self.end.column - 1,
                })
            }
        } else if self.start.column == self.end.column {
            if self.start.line < self.end.line {
                if self.start.column == 0 {
                    None
                } else {
                    Some(Point {
                        line: self.end.line + 1,
                        column: self.start.column - 1,
                    })
                }
            } else if self.end.line == 0 {
                None
            } else {
                Some(Point {
                    line: self.end.line - 1,
                    column: self.start.column + 1,
                })
            }
        } else {
            unreachable!()
        }
    }

    pub fn next_point_left_rotation(&self) -> Option<Point> {
        if self.start.line == self.end.line {
            if self.start.column < self.end.column {
                if self.start.line == 0 {
                    None
                } else {
                    Some(Point {
                        line: self.start.line - 1,
                        column: self.end.column + 1,
                    })
                }
            } else if self.end.column == 0 {
                None
            } else {
                Some(Point {
                    line: self.start.line + 1,
                    column: self.end.column - 1,
                })
            }
        } else if self.start.column == self.end.column {
            if self.start.line < self.end.line {
                Some(Point {
                    line: self.end.line + 1,
                    column: self.start.column + 1,
                })
            } else if self.end.line == 0 || self.start.column == 0 {
                None
            } else {
                Some(Point {
                    line: self.end.line - 1,
                    column: self.start.column - 1,
                })
            }
        } else {
            unreachable!()
        }
    }

    pub fn detect_direction(&self, face_width: usize) -> Direction {
        // In order to detect the direction, we check from which side of a face, the line was
        // generated
        if self.start.line == self.end.line {
            if self.start.line % face_width == 0 {
                Direction::Top
            } else {
                Direction::Bottom
            }
        } else if self.start.column == self.end.column {
            if self.start.column % face_width == 0 {
                Direction::Left
            } else {
                Direction::Right
            }
        } else {
            unreachable!()
        }
    }

    pub fn points(&self) -> Vec<Point> {
        if self.start.line == self.end.line {
            if self.start.column < self.end.column {
                (self.start.column..=self.end.column)
                    .map(|column| Point {
                        line: self.start.line,
                        column,
                    })
                    .collect()
            } else {
                (self.end.column..=self.start.column)
                    .rev()
                    .map(|column| Point {
                        line: self.start.line,
                        column,
                    })
                    .collect()
            }
        } else if self.start.column == self.end.column {
            if self.start.line < self.end.line {
                (self.start.line..=self.end.line)
                    .map(|line| Point {
                        line,
                        column: self.start.column,
                    })
                    .collect()
            } else {
                (self.end.line..=self.start.line)
                    .rev()
                    .map(|line| Point {
                        line,
                        column: self.start.column,
                    })
                    .collect()
            }
        } else {
            unreachable!()
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct GotoLinePair {
    /// Represent a pair of goto line that can be converted to a set of goto.
    /// Since the conversion to a set of goto will be done from the start to the end of each line,
    /// both lines must be in the correct order.
    pub first: GotoLine,
    pub second: GotoLine,
}

impl GotoLinePair {
    pub fn rev(self) -> Self {
        Self {
            first: self.first,
            second: self.second,
        }
    }

    pub fn get_goto(&self, face_width: usize) -> Vec<(Position, Position)> {
        let first_direction = self.first.detect_direction(face_width);
        let second_direction = self.second.detect_direction(face_width);

        self.first
            .points()
            .into_iter()
            .zip(self.second.points())
            .flat_map(|(first, second)| {
                [
                    (
                        Position {
                            line: first.line,
                            column: first.column,
                            direction: first_direction,
                        },
                        Position {
                            line: match second_direction {
                                Direction::Top => second.line + 1,
                                Direction::Bottom => second.line - 1,
                                Direction::Left | Direction::Right => second.line,
                            },
                            column: match second_direction {
                                Direction::Top | Direction::Bottom => second.column,
                                Direction::Right => second.column - 1,
                                Direction::Left => second.column + 1,
                            },
                            direction: second_direction.opposite(),
                        },
                    ),
                    (
                        Position {
                            line: second.line,
                            column: second.column,
                            direction: second_direction,
                        },
                        Position {
                            line: match first_direction {
                                Direction::Top => first.line + 1,
                                Direction::Bottom => first.line - 1,
                                Direction::Left | Direction::Right => first.line,
                            },
                            column: match first_direction {
                                Direction::Top | Direction::Bottom => first.column,
                                Direction::Right => first.column - 1,
                                Direction::Left => first.column + 1,
                            },
                            direction: first_direction.opposite(),
                        },
                    ),
                ]
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_next_point_forward() {
        // Horizontal to the right
        assert_eq!(
            GotoLine {
                start: Point { line: 1, column: 1 },
                end: Point { line: 1, column: 2 }
            }
            .next_point_forward(),
            Some(Point { line: 1, column: 3 })
        );
        // Horizontal to the left
        assert_eq!(
            GotoLine {
                start: Point { line: 1, column: 2 },
                end: Point { line: 1, column: 1 }
            }
            .next_point_forward(),
            Some(Point { line: 1, column: 0 })
        );
        // vertical to the bottom
        assert_eq!(
            GotoLine {
                start: Point { line: 1, column: 1 },
                end: Point { line: 2, column: 1 }
            }
            .next_point_forward(),
            Some(Point { line: 3, column: 1 })
        );
        // vertical to the top
        assert_eq!(
            GotoLine {
                start: Point { line: 2, column: 1 },
                end: Point { line: 1, column: 1 }
            }
            .next_point_forward(),
            Some(Point { line: 0, column: 1 })
        );
    }

    #[test]
    fn test_next_point_right_rotation() {
        // Horizontal to the right
        assert_eq!(
            GotoLine {
                start: Point { line: 1, column: 1 },
                end: Point { line: 1, column: 2 }
            }
            .next_point_right_rotation(),
            Some(Point { line: 2, column: 3 })
        );
        // Horizontal to the left
        assert_eq!(
            GotoLine {
                start: Point { line: 1, column: 2 },
                end: Point { line: 1, column: 1 }
            }
            .next_point_right_rotation(),
            Some(Point { line: 0, column: 0 })
        );
        // vertical to the bottom
        assert_eq!(
            GotoLine {
                start: Point { line: 1, column: 1 },
                end: Point { line: 2, column: 1 }
            }
            .next_point_right_rotation(),
            Some(Point { line: 3, column: 0 })
        );
        // vertical to the top
        assert_eq!(
            GotoLine {
                start: Point { line: 2, column: 1 },
                end: Point { line: 1, column: 1 }
            }
            .next_point_right_rotation(),
            Some(Point { line: 0, column: 2 })
        );
    }

    #[test]
    fn test_next_point_left_rotation() {
        // Horizontal to the right
        assert_eq!(
            GotoLine {
                start: Point { line: 1, column: 1 },
                end: Point { line: 1, column: 2 }
            }
            .next_point_left_rotation(),
            Some(Point { line: 0, column: 3 })
        );
        // Horizontal to the left
        assert_eq!(
            GotoLine {
                start: Point { line: 1, column: 2 },
                end: Point { line: 1, column: 1 }
            }
            .next_point_left_rotation(),
            Some(Point { line: 2, column: 0 })
        );
        // vertical to the bottom
        assert_eq!(
            GotoLine {
                start: Point { line: 1, column: 1 },
                end: Point { line: 2, column: 1 }
            }
            .next_point_left_rotation(),
            Some(Point { line: 3, column: 2 })
        );
        // vertical to the top
        assert_eq!(
            GotoLine {
                start: Point { line: 2, column: 1 },
                end: Point { line: 1, column: 1 }
            }
            .next_point_left_rotation(),
            Some(Point { line: 0, column: 0 })
        );
    }

    #[test]
    fn test_points() {
        // Horizontal to the right
        assert_eq!(
            GotoLine {
                start: Point { line: 1, column: 1 },
                end: Point { line: 1, column: 2 }
            }
            .points(),
            vec![Point { line: 1, column: 1 }, Point { line: 1, column: 2 }]
        );
        // Horizontal to the left
        assert_eq!(
            GotoLine {
                start: Point { line: 1, column: 2 },
                end: Point { line: 1, column: 1 }
            }
            .points(),
            vec![Point { line: 1, column: 2 }, Point { line: 1, column: 1 }]
        );
        // vertical to the bottom
        assert_eq!(
            GotoLine {
                start: Point { line: 1, column: 1 },
                end: Point { line: 2, column: 1 }
            }
            .points(),
            vec![Point { line: 1, column: 1 }, Point { line: 2, column: 1 }]
        );
        // vertical to the top
        assert_eq!(
            GotoLine {
                start: Point { line: 2, column: 1 },
                end: Point { line: 1, column: 1 }
            }
            .points(),
            vec![Point { line: 2, column: 1 }, Point { line: 1, column: 1 }]
        );
    }
}
