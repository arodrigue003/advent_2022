#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn distance(&self, other: &Point) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct SensorScan {
    pub sensor: Point,
    pub beacon: Point,
    distance: i64,
}

impl SensorScan {
    pub fn new(sensor: Point, beacon: Point) -> Self {
        let distance = sensor.distance(&beacon);

        Self {
            sensor,
            beacon,
            distance,
        }
    }
}

impl SensorScan {
    /// Return the interval of positions that cannot contain an undetected beacon
    /// for the given line.
    ///
    /// The interval is inclusive on both sides
    pub fn get_line_intersection_interval(&self, y: i64) -> Option<(i64, i64)> {
        let distance_from_line = (self.sensor.y - y).abs();

        if distance_from_line > self.distance {
            None
        } else {
            Some((
                self.sensor.x - self.distance + distance_from_line,
                self.sensor.x + self.distance - distance_from_line,
            ))
        }
    }
}
