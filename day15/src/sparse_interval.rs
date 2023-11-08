use itertools::Itertools;

#[derive(Debug, Eq, PartialEq, Clone)]
enum AdditionOperation {
    Insert(usize),
    Combine(usize, usize),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct SparseInterval {
    intervals: Vec<(i64, i64)>,
}

impl Default for SparseInterval {
    fn default() -> Self {
        Self::new()
    }
}

impl SparseInterval {
    pub fn new() -> Self {
        Self { intervals: vec![] }
    }

    fn get_add_interval_operation(&self, interval: &(i64, i64)) -> AdditionOperation {
        // Check if we need to insert the interval in the stored intervals
        if self.intervals.is_empty() {
            return AdditionOperation::Insert(0);
        }
        if interval.1 + 1 < self.intervals[0].0 {
            // Interval is before the first stored interval
            return AdditionOperation::Insert(0);
        }
        for (index, (left, right)) in self.intervals.iter().tuple_windows().enumerate() {
            if interval.0 - 1 > left.1 && interval.1 + 1 < right.0 {
                // Interval is between two intervals
                return AdditionOperation::Insert(index + 1);
            }
        }
        if interval.0 - 1 > self.intervals[self.intervals.len() - 1].1 {
            // Interval is after the last stored interval
            return AdditionOperation::Insert(self.intervals.len());
        }

        // We look for the first interval for which the end is after the start of the new interval.
        let first_before = self
            .intervals
            .iter()
            .position(|elt| elt.1 + 1 >= interval.0);
        // We look for the last interval for which the start is before the end of the new interval.
        let last_after = self
            .intervals
            .iter()
            .rev()
            .position(|elt| elt.0 - 1 <= interval.1)
            .map(|pos| self.intervals.len() - pos - 1);

        match (first_before, last_after) {
            (None, None) => unreachable!(),
            (Some(left), None) => AdditionOperation::Combine(left, left),
            (None, Some(right)) => AdditionOperation::Combine(right, right),
            (Some(left), Some(right)) => AdditionOperation::Combine(left, right),
        }
    }

    pub fn add_interval(&mut self, interval: &(i64, i64)) {
        match self.get_add_interval_operation(interval) {
            AdditionOperation::Insert(index) => self.intervals.insert(index, *interval),
            AdditionOperation::Combine(left, right) => {
                // Get the first and last interval objects for further comparison
                let left_interval = self.intervals[left];
                let right_interval = self.intervals[right];

                // Remove every intervals in the range
                for _ in left..=right {
                    self.intervals.remove(left);
                }

                // Add the new merged interval
                self.intervals.insert(
                    left,
                    (
                        left_interval.0.min(interval.0),
                        right_interval.1.max(interval.1),
                    ),
                )
            }
        }
    }

    // Count the number of elements in the sparse interval
    pub fn size(&self) -> i64 {
        self.intervals
            .iter()
            .map(|interval| (interval.1 - interval.0 + 1))
            .sum()
    }

    // Return true if the interval given as a parameter is full
    pub fn contains(&self, interval: &(i64, i64)) -> bool {
        for inner_interval in &self.intervals {
            if inner_interval.0 <= interval.0 && inner_interval.1 >= interval.1 {
                return true;
            }
        }
        false
    }

    // This function return a vec of missing elements in the given interval.
    // TODO: correctly implement this function
    pub fn get_missing_elements(&self, _interval: &(i64, i64)) -> Vec<i64> {
        vec![self.intervals[0].1 + 1]
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_get_add_interval_operation() {
        // Empty interval test
        let interval = SparseInterval { intervals: vec![] };
        assert_eq!(
            interval.get_add_interval_operation(&(5, 10)),
            AdditionOperation::Insert(0)
        );

        // Insert tests
        let interval = SparseInterval {
            intervals: vec![(5, 10), (15, 20)],
        };
        assert_eq!(
            interval.get_add_interval_operation(&(0, 3)),
            AdditionOperation::Insert(0)
        );
        assert_eq!(
            interval.get_add_interval_operation(&(12, 13)),
            AdditionOperation::Insert(1)
        );
        assert_eq!(
            interval.get_add_interval_operation(&(22, 25)),
            AdditionOperation::Insert(2)
        );

        // Combine test one element
        //  With first one before
        assert_eq!(
            interval.get_add_interval_operation(&(0, 4)),
            AdditionOperation::Combine(0, 0)
        );
        assert_eq!(
            interval.get_add_interval_operation(&(0, 5)),
            AdditionOperation::Combine(0, 0)
        );
        assert_eq!(
            interval.get_add_interval_operation(&(0, 6)),
            AdditionOperation::Combine(0, 0)
        );
        //  With first one after
        assert_eq!(
            interval.get_add_interval_operation(&(9, 13)),
            AdditionOperation::Combine(0, 0)
        );
        assert_eq!(
            interval.get_add_interval_operation(&(10, 13)),
            AdditionOperation::Combine(0, 0)
        );
        assert_eq!(
            interval.get_add_interval_operation(&(11, 13)),
            AdditionOperation::Combine(0, 0)
        );
        //  With second one before
        assert_eq!(
            interval.get_add_interval_operation(&(12, 14)),
            AdditionOperation::Combine(1, 1)
        );
        assert_eq!(
            interval.get_add_interval_operation(&(12, 15)),
            AdditionOperation::Combine(1, 1)
        );
        assert_eq!(
            interval.get_add_interval_operation(&(12, 16)),
            AdditionOperation::Combine(1, 1)
        );
        //  With second one after
        assert_eq!(
            interval.get_add_interval_operation(&(19, 25)),
            AdditionOperation::Combine(1, 1)
        );
        assert_eq!(
            interval.get_add_interval_operation(&(20, 25)),
            AdditionOperation::Combine(1, 1)
        );
        assert_eq!(
            interval.get_add_interval_operation(&(21, 25)),
            AdditionOperation::Combine(1, 1)
        );

        // Combine test two elements
        assert_eq!(
            interval.get_add_interval_operation(&(0, 17)),
            AdditionOperation::Combine(0, 1)
        );
        assert_eq!(
            interval.get_add_interval_operation(&(6, 17)),
            AdditionOperation::Combine(0, 1)
        );
        assert_eq!(
            interval.get_add_interval_operation(&(11, 17)),
            AdditionOperation::Combine(0, 1)
        );
        assert_eq!(
            interval.get_add_interval_operation(&(7, 14)),
            AdditionOperation::Combine(0, 1)
        );
        assert_eq!(
            interval.get_add_interval_operation(&(7, 17)),
            AdditionOperation::Combine(0, 1)
        );
        assert_eq!(
            interval.get_add_interval_operation(&(7, 25)),
            AdditionOperation::Combine(0, 1)
        );
        assert_eq!(
            interval.get_add_interval_operation(&(0, 25)),
            AdditionOperation::Combine(0, 1)
        );

        // More than two elements spans
        let interval = SparseInterval {
            intervals: vec![(5, 10), (15, 20), (25, 30)],
        };
        assert_eq!(
            interval.get_add_interval_operation(&(0, 35)),
            AdditionOperation::Combine(0, 2)
        );
    }

    #[test]
    fn test_add_interval() {
        // Empty interval test
        let mut interval = SparseInterval { intervals: vec![] };
        interval.add_interval(&(5, 10));
        assert_eq!(interval.intervals, vec![(5, 10)]);

        // Insert tests
        let mut interval = SparseInterval {
            intervals: vec![(5, 10), (15, 20)],
        };
        interval.add_interval(&(0, 3));
        assert_eq!(interval.intervals, vec![(0, 3), (5, 10), (15, 20)]);

        let mut interval = SparseInterval {
            intervals: vec![(5, 10), (15, 20)],
        };
        interval.add_interval(&(12, 13));
        assert_eq!(interval.intervals, vec![(5, 10), (12, 13), (15, 20)]);

        let mut interval = SparseInterval {
            intervals: vec![(5, 10), (15, 20)],
        };
        interval.add_interval(&(22, 25));
        assert_eq!(interval.intervals, vec![(5, 10), (15, 20), (22, 25)]);

        // Combine test one element
        //  With first one before
        let mut interval = SparseInterval {
            intervals: vec![(5, 10), (15, 20)],
        };
        interval.add_interval(&(0, 4));
        assert_eq!(interval.intervals, vec![(0, 10), (15, 20)]);

        let mut interval = SparseInterval {
            intervals: vec![(5, 10), (15, 20)],
        };
        interval.add_interval(&(0, 5));
        assert_eq!(interval.intervals, vec![(0, 10), (15, 20)]);

        let mut interval = SparseInterval {
            intervals: vec![(5, 10), (15, 20)],
        };
        interval.add_interval(&(0, 6));
        assert_eq!(interval.intervals, vec![(0, 10), (15, 20)]);

        //  With first one after
        let mut interval = SparseInterval {
            intervals: vec![(5, 10), (15, 20)],
        };
        interval.add_interval(&(9, 13));
        assert_eq!(interval.intervals, vec![(5, 13), (15, 20)]);

        let mut interval = SparseInterval {
            intervals: vec![(5, 10), (15, 20)],
        };
        interval.add_interval(&(10, 13));
        assert_eq!(interval.intervals, vec![(5, 13), (15, 20)]);

        let mut interval = SparseInterval {
            intervals: vec![(5, 10), (15, 20)],
        };
        interval.add_interval(&(11, 13));
        assert_eq!(interval.intervals, vec![(5, 13), (15, 20)]);

        //  With second one before
        let mut interval = SparseInterval {
            intervals: vec![(5, 10), (15, 20)],
        };
        interval.add_interval(&(12, 14));
        assert_eq!(interval.intervals, vec![(5, 10), (12, 20)]);

        let mut interval = SparseInterval {
            intervals: vec![(5, 10), (15, 20)],
        };
        interval.add_interval(&(12, 14));
        assert_eq!(interval.intervals, vec![(5, 10), (12, 20)]);

        let mut interval = SparseInterval {
            intervals: vec![(5, 10), (15, 20)],
        };
        interval.add_interval(&(12, 14));
        assert_eq!(interval.intervals, vec![(5, 10), (12, 20)]);

        //  With second one after
        let mut interval = SparseInterval {
            intervals: vec![(5, 10), (15, 20)],
        };
        interval.add_interval(&(19, 25));
        assert_eq!(interval.intervals, vec![(5, 10), (15, 25)]);

        let mut interval = SparseInterval {
            intervals: vec![(5, 10), (15, 20)],
        };
        interval.add_interval(&(20, 25));
        assert_eq!(interval.intervals, vec![(5, 10), (15, 25)]);

        let mut interval = SparseInterval {
            intervals: vec![(5, 10), (15, 20)],
        };
        interval.add_interval(&(21, 25));
        assert_eq!(interval.intervals, vec![(5, 10), (15, 25)]);

        // Combine test two elements
        let mut interval = SparseInterval {
            intervals: vec![(5, 10), (15, 20)],
        };
        interval.add_interval(&(0, 17));
        assert_eq!(interval.intervals, vec![(0, 20)]);

        let mut interval = SparseInterval {
            intervals: vec![(5, 10), (15, 20)],
        };
        interval.add_interval(&(6, 17));
        assert_eq!(interval.intervals, vec![(5, 20)]);

        let mut interval = SparseInterval {
            intervals: vec![(5, 10), (15, 20)],
        };
        interval.add_interval(&(11, 17));
        assert_eq!(interval.intervals, vec![(5, 20)]);

        let mut interval = SparseInterval {
            intervals: vec![(5, 10), (15, 20)],
        };
        interval.add_interval(&(7, 14));
        assert_eq!(interval.intervals, vec![(5, 20)]);

        let mut interval = SparseInterval {
            intervals: vec![(5, 10), (15, 20)],
        };
        interval.add_interval(&(7, 17));
        assert_eq!(interval.intervals, vec![(5, 20)]);

        let mut interval = SparseInterval {
            intervals: vec![(5, 10), (15, 20)],
        };
        interval.add_interval(&(7, 25));
        assert_eq!(interval.intervals, vec![(5, 25)]);

        let mut interval = SparseInterval {
            intervals: vec![(5, 10), (15, 20)],
        };
        interval.add_interval(&(0, 17));
        assert_eq!(interval.intervals, vec![(0, 20)]);

        let mut interval = SparseInterval {
            intervals: vec![(5, 10), (15, 20)],
        };
        interval.add_interval(&(0, 25));
        assert_eq!(interval.intervals, vec![(0, 25)]);

        // More than two elements spans
        let mut interval = SparseInterval {
            intervals: vec![(5, 10), (15, 20), (25, 30)],
        };
        interval.add_interval(&(6, 35));
        assert_eq!(interval.intervals, vec![(5, 35)]);
    }

    #[test]
    fn test_size() {
        let interval = SparseInterval {
            intervals: vec![(5, 10), (15, 20), (25, 30)],
        };
        assert_eq!(interval.size(), 18);
    }
}
