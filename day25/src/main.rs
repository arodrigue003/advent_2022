use std::{env, fs};

fn to_snafu(fuel_requirement: &mut i64) -> String {
    let mut snafu_fuel_requirement = String::new();
    while fuel_requirement != 0 {
        let quotient = ((fuel_requirement + 2) % 5) - 2;
        fuel_requirement = (fuel_requirement + 2) / 5;
        match quotient {
            -2 => snafu_fuel_requirement.push('='),
            -1 => snafu_fuel_requirement.push('-'),
            0 => snafu_fuel_requirement.push('0'),
            1 => snafu_fuel_requirement.push('1'),
            2 => snafu_fuel_requirement.push('2'),
            _ => unreachable!(),
        }
    }
    snafu_fuel_requirement.chars().rev().collect()
}

fn solve_part_one(data: &str) {
    let mut fuel_requirement: i64 = data
        .lines()
        .map(|line| {
            line.chars().fold(0, |acc, x| {
                acc * 5
                    + match x {
                        '=' => -2,
                        '-' => -1,
                        '0' => 0,
                        '1' => 1,
                        '2' => 2,
                        _ => unreachable!(),
                    }
            })
        })
        .sum();

    let snafu_fuel_requirement = to_snafu(&mut fuel_requirement);

    println!("Part one solution: {:#?}", snafu_fuel_requirement);
}

fn solve_part_two(data: &str) {
    println!("Part two solution:");
}

fn main() {
    let args: Vec<_> = env::args().collect();

    let file_path = if args.len() > 1 {
        args[1].clone()
    } else {
        "input".to_string()
    };

    let data = fs::read_to_string(&file_path).unwrap();

    solve_part_one(&data);
    solve_part_two(&data);
}
