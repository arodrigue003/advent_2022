use std::{env, fs};

use day17::grid::Grid;
use day17::model::Direction;

fn solve_part_one(directions: &[Direction]) {
    let mut grid = Grid::new();

    let res = grid.simulate_falling(directions, 2022);

    println!("Part two solution: {}", res);
}

fn solve_part_two(directions: &[Direction]) {
    let mut grid = Grid::new();

    let res = grid.simulate_falling(directions, 1_000_000_000_000);

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
    let directions: Vec<_> = data
        .chars()
        .filter_map(|char| match char {
            '<' => Some(Direction::Left),
            '>' => Some(Direction::Right),
            '\n' | '\r' => None,
            _ => unreachable!(),
        })
        .collect();

    solve_part_one(&directions);
    solve_part_two(&directions);
}
