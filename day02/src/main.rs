mod part_one;
mod part_two;

use std::{env, fs};

fn main() {
    let args: Vec<_> = env::args().collect();

    let file_path = if args.len() > 1 {
        args[1].clone()
    } else {
        "input".to_string()
    };

    let data = fs::read_to_string(&file_path).unwrap();

    part_one::solve_part_one(&data);
    part_two::solve_part_two(&data);
}
