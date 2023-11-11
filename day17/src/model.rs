use lazy_static::lazy_static;

pub struct Shape {
    pub shape: Vec<Point>,
    pub height: usize,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Direction {
    Left,
    Right,
}

lazy_static! {
    pub static ref SHAPES: [Shape; 5] = [
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
