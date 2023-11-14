#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

impl<'a> From<&'a str> for Operator {
    fn from(value: &'a str) -> Self {
        match value {
            "+" => Self::Addition,
            "-" => Self::Subtraction,
            "*" => Self::Multiplication,
            "/" | "%" => Self::Division,
            _ => unreachable!(),
        }
    }
}

impl Operator {
    pub fn compute(&self, left: i64, right: i64) -> i64 {
        match self {
            Operator::Addition => left + right,
            Operator::Subtraction => left - right,
            Operator::Multiplication => left * right,
            Operator::Division => left / right,
        }
    }

    pub fn compute_with_human(&self, left: Option<i64>, right: Option<i64>) -> Option<i64> {
        match (left, right) {
            (Some(left), Some(right)) => match self {
                Operator::Addition => Some(left + right),
                Operator::Subtraction => Some(left - right),
                Operator::Multiplication => Some(left * right),
                Operator::Division => Some(left / right),
            },
            _ => None,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Operation<'a> {
    pub left: &'a str,
    pub operator: Operator,
    pub right: &'a str,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Monkey<'a> {
    Operation(Operation<'a>),
    Value(i64),
}
