use crate::model::Monkey;

pub fn display_monkeys(round: usize, monkeys: &[Monkey]) {
    println!(
        "After round {}, the monkeys are holding items with these worry levels:",
        round
    );
    for monkey in monkeys {
        println!("{}", monkey)
    }
    println!();
}

pub fn two_max(monkeys: &[Monkey]) -> (usize, usize) {
    monkeys
        .iter()
        .map(|monkey| monkey.inspected_items)
        .fold((0, 0), |(max1, max2), x| {
            if x >= max1 {
                (x, max1)
            } else if x >= max2 {
                (max1, x)
            } else {
                (max1, max2)
            }
        })
}
