use crate::model::{Monkey, Operand, Operation, Operator, Test};

impl Operation {
    pub fn apply_operation(&self, old: i64) -> i64 {
        let left = match &self.left {
            Operand::Old => old,
            Operand::Value(value) => *value,
        };

        let right = match &self.right {
            Operand::Old => old,
            Operand::Value(value) => *value,
        };

        match &self.operator {
            Operator::Addition => left + right,
            Operator::Subtraction => left - right,
            Operator::Multiplication => left * right,
            Operator::Division => left / right,
        }
    }
}

impl Test {
    pub fn get_target(&self, worry_level: i64) -> usize {
        if worry_level % self.quotient == 0 {
            self.true_target
        } else {
            self.false_target
        }
    }
}

impl Monkey {
    /// Inspect an item and return a tuple containing the target monkey and the new worry level.
    ///
    /// Return none if the monkey no longer has an item to inspect
    pub fn inspect(&mut self, apply_division: bool, lcm: Option<i64>) -> Option<(usize, i64)> {
        // take the first item in the list
        let to_inspect = self.items.pop_front();

        match to_inspect {
            None => None,
            Some(worry_level) => {
                // Add 1 to the number of inspected items
                self.inspected_items += 1;

                // apply the operation
                let mut worry_level = self.operation.apply_operation(worry_level);

                // Only keep the reminder if requested
                if let Some(lcm) = lcm {
                    worry_level = worry_level % lcm;
                }

                // Make the divide by three division if requested
                if apply_division {
                    worry_level = worry_level / 3;
                }

                // Get the target
                let target = self.test.get_target(worry_level);

                // Return the result
                Some((target, worry_level))
            }
        }
    }
}
