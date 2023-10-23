use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use std::collections::HashSet;
use std::{env, fs};

fn solve_part_one(data: &str) {
    let first_char = data.chars().next().unwrap();
    let res = data
        .chars()
        .fold_while(
            ((first_char, first_char, first_char), 1),
            |((a, b, c), pos), char| {
                if char != a && char != b && char != c && c != b && c != a && b != a {
                    Done(((a, b, c), pos))
                } else {
                    Continue(((b, c, char), pos + 1))
                }
            },
        )
        .into_inner()
        .1;

    println!("Part one solution: {:#?}", res);
}

fn solve_part_two(data: &str) {
    let res = data
        .chars()
        .collect::<Vec<_>>()
        .windows(14)
        .enumerate()
        .filter_map(|(pos, window)| {
            if window.iter().collect::<HashSet<_>>().len() == 14 {
                Some(pos + 14)
            } else {
                None
            }
        })
        .next()
        .unwrap();

    println!("Part two solution: {:#?}", res);
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
