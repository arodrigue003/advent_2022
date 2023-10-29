mod grid;

use crate::grid::Grid;
use std::fs;
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug, Eq, PartialEq, Clone)]
struct Cli {
    /// Enable verbose display
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// File to parse
    #[arg(default_value = "input")]
    path: PathBuf,
}

fn solve_part_one(data: &Grid) {
    let res: usize = (0..data.height())
        .flat_map(|line| {
            (0..data.width()).map(move |column| data.is_visible(line, column) as usize)
        })
        .sum();

    println!("Part one solution: {:#?}", res);
}

fn solve_part_two(data: &str) {
    println!("Part two solution: {:#?}", "data");
}

fn main() {
    let args = Cli::parse();

    let data: String = fs::read_to_string(&args.path).unwrap();
    let grid = Grid::from_str(&data);

    if args.verbose {
        grid.pretty_print();
    }

    solve_part_one(&grid);
    solve_part_two(&data);
}
