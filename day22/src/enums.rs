use std::fmt::{Display, Formatter};

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

    pub fn opposite(&self) -> Self {
        match self {
            Direction::Top => Direction::Bottom,
            Direction::Right => Direction::Left,
            Direction::Bottom => Direction::Top,
            Direction::Left => Direction::Right,
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

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Command {
    Forward(usize),
    Rotate(Rotation),
}
