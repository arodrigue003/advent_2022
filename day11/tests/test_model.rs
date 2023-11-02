use day11::model::{Operand, Operation, Operator, Test};

#[test]
pub fn test_apply_operation() {
    assert_eq!(Operation{
        left: Operand::Old,
        operator: Operator::Addition,
        right: Operand::Old,
    }.apply_operation(42), 84);

    assert_eq!(Operation{
        left: Operand::Old,
        operator: Operator::Multiplication,
        right: Operand::Value(19),
    }.apply_operation(42), 798);
}

#[test]
pub fn test_get_target() {
    assert_eq!(Test {
        quotient: 23,
        true_target: 2,
        false_target: 3,
    }.get_target(500), 3);
    assert_eq!(Test {
        quotient: 13,
        true_target: 1,
        false_target: 3,
    }.get_target(2080), 1)
}