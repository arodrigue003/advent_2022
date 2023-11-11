use crate::model::Point;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FrontLine {
    pub start: usize,
    pub low: usize,
    pub end: usize,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum FrontLineDirection {
    Left,
    Top,
    Right,
    Bot,
}

impl FrontLineDirection {
    pub fn get_left_and_front_positions(&self, position: &Point) -> (Option<Point>, Option<Point>) {
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

    pub fn turn_left(&self) -> Self {
        match self {
            FrontLineDirection::Left => FrontLineDirection::Bot,
            FrontLineDirection::Top => FrontLineDirection::Left,
            FrontLineDirection::Right => FrontLineDirection::Top,
            FrontLineDirection::Bot => FrontLineDirection::Right,
        }
    }

    pub fn turn_right(&self) -> Self {
        match self {
            FrontLineDirection::Left => FrontLineDirection::Top,
            FrontLineDirection::Top => FrontLineDirection::Right,
            FrontLineDirection::Right => FrontLineDirection::Bot,
            FrontLineDirection::Bot => FrontLineDirection::Left,
        }
    }
}
