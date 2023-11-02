use std::collections::VecDeque;
use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Operand {
    Old,
    Value(i64),
}

impl<'a> From<&'a str> for Operand {
    fn from(value: &'a str) -> Self {
        match value {
            "old" => Self::Old,
            number => Self::Value(number.parse().unwrap()),
        }
    }
}

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

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Operation {
    pub left: Operand,
    pub operator: Operator,
    pub right: Operand,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Test {
    pub quotient: i64,
    pub true_target: usize,
    pub false_target: usize,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Monkey {
    pub number: usize,
    pub items: VecDeque<i64>,
    pub operation: Operation,
    pub test: Test,
    pub inspected_items: usize
}

impl Display for Monkey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Monkey {}: {:?}", self.number, self.items)
    }
}