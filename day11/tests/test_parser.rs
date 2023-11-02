use day11::model::{Monkey, Operand, Operation, Operator, Test};
use day11::parser::{parse_monkey, parse_monkey_line, parse_operation_line, parse_starting_items_line, parse_test_lines};

#[test]
fn test_parse_starting_item_line() {
    assert_eq!(
        parse_starting_items_line("  Starting items: 74\n"),
        Ok(("", vec![74]))
    );
    assert_eq!(
        parse_starting_items_line("  Starting items: 79, 98\n"),
        Ok(("", vec![79, 98]))
    );
}

#[test]
fn test_parse_operation_line() {
    assert_eq!(
        parse_operation_line("  Operation: new = old + 3\n"),
        Ok((
            "",
            Operation {
                left: Operand::Old,
                operator: Operator::Addition,
                right: Operand::Value(3),
            }
        ))
    );
    assert_eq!(
        parse_operation_line("  Operation: new = old * old\n"),
        Ok((
            "",
            Operation {
                left: Operand::Old,
                operator: Operator::Multiplication,
                right: Operand::Old,
            }
        ))
    );
    assert_eq!(
        parse_operation_line("  Operation: new = old * 19\n"),
        Ok((
            "",
            Operation {
                left: Operand::Old,
                operator: Operator::Multiplication,
                right: Operand::Value(19),
            }
        ))
    );
}

#[test]
pub fn test_parse_test_lines() {
    let to_parse: &'static str = "  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";
    assert_eq!(
        parse_test_lines(to_parse),
        Ok((
            (""),
            Test {
                quotient: 17,
                true_target: 0,
                false_target: 1,
            }
        ))
    );
}

#[test]
pub fn test_parse_monkey_line() {
    assert_eq!(parse_monkey_line("Monkey 0:\n"), Ok(("", 0)));
    assert_eq!(parse_monkey_line("Monkey 3:\n"), Ok(("", 3)));
}

#[test]
pub fn test_parse_monkey() {
    let to_parse: &'static str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3
";

    assert_eq!(parse_monkey(to_parse), Ok(("", Monkey {
        number: 0,
        items: vec![79, 98].into(),
        operation: Operation {
            left: Operand::Old,
            operator: Operator::Multiplication,
            right: Operand::Value(19),
        },
        test: Test {
            quotient: 23,
            true_target: 2,
            false_target: 3,
        },
        inspected_items: 0,
    })))
}