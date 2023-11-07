#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Point {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Line(pub Vec<Point>);
