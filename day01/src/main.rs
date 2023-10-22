use itertools::Itertools;
use std::{env, fs};

fn solve_part_one(data: &Vec<Option<i32>>) {
    let res = data
        .iter()
        .fold((0, 0), |(acc, max), value| match value {
            None => (0, max),
            Some(value) => (acc + *value, max.max(acc + *value)),
        })
        .1;

    println!("Part one solution: {:#?}", res);
}

fn solve_part_two(data: &Vec<Option<i32>>) {
    let res: i32 = data
        .iter()
        .scan(0, |acc, value| match value {
            None => {
                let total = *acc;
                *acc = 0;
                Some(Some(total))
            }
            Some(value) => {
                *acc += *value;
                Some(None)
            }
        })
        .filter_map(|acc| acc) // Here we have a list of each elf calories storage
        .map(|acc| -acc) // Take the negation to use k_smallest from itertools
        .k_smallest(3) // Get the three smallest elements
        .map(|acc| -acc) // Make them positive again
        .sum(); // Sum them

    println!("Part two solution: {:#?}", res);
}

fn main() {
    let args: Vec<_> = env::args().collect();

    let file_path = if args.len() > 1 {
        args[1].clone()
    } else {
        "input".to_string()
    };

    let data: Vec<Option<i32>> = fs::read_to_string(&file_path)
        .unwrap()
        .lines()
        .map(|line| {
            if line != "" {
                Some(line.parse().unwrap())
            } else {
                None
            }
        })
        .chain(std::iter::once(None))
        .collect();

    solve_part_one(&data);
    solve_part_two(&data);
}
