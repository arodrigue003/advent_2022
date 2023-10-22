use itertools::{Either, Itertools};
use std::collections::HashSet;
use std::{env, fs};

fn solve_part_one(data: &str) {
    let res: i32 = data
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .map(|line| {
            let middle = line.len() / 2;
            line.into_iter().enumerate().partition_map(|(pos, char)| {
                if pos < middle {
                    Either::Left(char)
                } else {
                    Either::Right(char)
                }
            })
        })
        .map(|(left, right): (HashSet<char>, HashSet<char>)| {
            left.intersection(&right).next().unwrap().to_owned()
        })
        .map(|char| match char {
            'a'..='z' => char as i32 - 96,
            'A'..='Z' => char as i32 - 64 + 26,
            _ => unimplemented!(),
        })
        .sum();

    println!("Part one solution: {:#?}", res);
}

fn solve_part_two(data: &str) {
    let res: i32 = data
        .lines()
        .map(|line| line.chars().collect::<HashSet<_>>())
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            let chunks: Vec<_> = chunk.collect();
            chunks[0]
                .intersection(&chunks[1])
                .cloned()
                .collect::<HashSet<_>>()
                .intersection(&chunks[2])
                .next()
                .unwrap()
                .to_owned()
        })
        .map(|char| match char {
            'a'..='z' => char as i32 - 96,
            'A'..='Z' => char as i32 - 64 + 26,
            _ => unimplemented!(),
        })
        .sum();

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
