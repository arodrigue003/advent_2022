use std::collections::HashMap;
use std::{env, fs};

use day21::model::{Monkey, Operation, Operator};
use day21::parser::parse_data;

fn apply_operation(monkeys: &HashMap<&str, Monkey>, monkey_name: &str) -> i64 {
    // get the monkey
    let monkey = monkeys.get(monkey_name).unwrap();

    match monkey {
        Monkey::Operation(operation) => operation.operator.compute(
            apply_operation(monkeys, operation.left),
            apply_operation(monkeys, operation.right),
        ),
        Monkey::Value(value) => *value,
    }
}

fn apply_operation_with_human(monkeys: &HashMap<&str, Monkey>, monkey_name: &str) -> Option<i64> {
    // If monkey is human return None
    if monkey_name == "humn" {
        return None;
    }

    // get the monkey
    let monkey = monkeys.get(monkey_name).unwrap();

    match monkey {
        Monkey::Operation(operation) => operation.operator.compute_with_human(
            apply_operation_with_human(monkeys, operation.left),
            apply_operation_with_human(monkeys, operation.right),
        ),
        Monkey::Value(value) => Some(*value),
    }
}

fn get_human_value(monkeys: &HashMap<&str, Monkey>, monkey_name: &str, target: i64) -> i64 {
    if monkey_name == "humn" {
        return target;
    }

    // get the monkey
    let monkey = monkeys.get(monkey_name).unwrap();

    match monkey {
        Monkey::Operation(operation) => {
            let left = apply_operation_with_human(monkeys, operation.left);
            let right = apply_operation_with_human(monkeys, operation.right);

            match (left, right) {
                (Some(left), None) => match operation.operator {
                    Operator::Addition => get_human_value(monkeys, operation.right, target - left),
                    Operator::Subtraction => {
                        get_human_value(monkeys, operation.right, left - target)
                    }
                    Operator::Multiplication => {
                        get_human_value(monkeys, operation.right, target / left)
                    }
                    Operator::Division => get_human_value(monkeys, operation.right, left / target),
                },
                (None, Some(right)) => match operation.operator {
                    Operator::Addition => get_human_value(monkeys, operation.left, target - right),
                    Operator::Subtraction => {
                        get_human_value(monkeys, operation.left, target + right)
                    }
                    Operator::Multiplication => {
                        get_human_value(monkeys, operation.left, target / right)
                    }
                    Operator::Division => get_human_value(monkeys, operation.left, target * right),
                },
                _ => unreachable!(),
            }
        }
        Monkey::Value(value) => *value,
    }
}

fn solve_part_one(monkeys: &HashMap<&str, Monkey>) {
    let res = apply_operation(monkeys, "root");

    println!("Part one solution: {}", res);
}

fn solve_part_two(monkeys: &HashMap<&str, Monkey>) {
    let mut monkeys = monkeys.clone();

    // Modify monkey in order to simplify the computation
    let mut root = monkeys.get_mut("root").unwrap();
    *root = match root {
        Monkey::Operation(operation) => Monkey::Operation(Operation {
            left: operation.left,
            operator: Operator::Subtraction,
            right: operation.right,
        }),
        Monkey::Value(_) => unreachable!(),
    };

    // Get the result
    let res = get_human_value(&monkeys, "root", 0);

    // Get the expected result
    println!("Part two solution: {}", res);
}

fn main() {
    let args: Vec<_> = env::args().collect();

    let file_path = if args.len() > 1 {
        args[1].clone()
    } else {
        "input".to_string()
    };

    let data = fs::read_to_string(&file_path).unwrap();
    let monkeys = parse_data(&data);

    solve_part_one(&monkeys);
    solve_part_two(&monkeys);
}
