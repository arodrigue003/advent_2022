use clap::Parser;
use day11::model::Monkey;
use day11::parser::parse_data;
use day11::utils::{display_monkeys, two_max};
use std::fs;
use std::path::PathBuf;
use euc_lib::Lcm;

#[derive(Parser, Debug, Eq, PartialEq, Clone)]
struct Cli {
    /// Enable verbose display
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// File to parse
    #[arg(default_value = "input")]
    path: PathBuf,
}

fn solve_part_one(verbose: bool, mut monkeys: Vec<Monkey>) {
    if verbose {
        display_monkeys(0, &monkeys);
    }
    for round in 1..=20 {
        for i_monkey in 0..monkeys.len() {
            loop {
                let inspect_result = monkeys[i_monkey].inspect(true, None);

                match inspect_result {
                    None => break,
                    Some((target, worry_level)) => {
                        // Add the item to the target monkey
                        monkeys[target].items.push_back(worry_level)
                    }
                }
            }
        }

        if verbose {
            display_monkeys(round, &monkeys);
        }
    }

    // Get the two max inspection times
    let (max1, max2) = two_max(&monkeys);

    println!("Part one solution: {:?}", max1 * max2);
}

fn solve_part_two(mut monkeys: Vec<Monkey>) {
    // Compute lcm of monkeys divisibility tests
    let lcm = monkeys.iter().map(|monkey| {
        monkey.test.quotient
    }).fold(1, |acc, x| euc_lib::I64::lcm(acc, x));

    for _ in 1..=10_000 {
        for i_monkey in 0..monkeys.len() {
            loop {
                let inspect_result = monkeys[i_monkey].inspect(false, Some(lcm));

                match inspect_result {
                    None => break,
                    Some((target, worry_level)) => {
                        // Add the item to the target monkey
                        monkeys[target].items.push_back(worry_level)
                    }
                }
            }
        }
    }

    // Get the two max inspection times
    let (max1, max2) = two_max(&monkeys);

    println!("Part one solution: {:?}", max1 * max2);
}

fn main() {
    let args = Cli::parse();

    let data: String = fs::read_to_string(&args.path).unwrap();
    let monkeys = parse_data(&data);

    solve_part_one(args.verbose, monkeys.clone());
    solve_part_two(monkeys.clone());
}
