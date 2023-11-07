use std::fs;
use std::path::PathBuf;

use clap::{ArgAction, Parser};

use day14::grid::Grid;
use day14::model::Line;
use day14::parser;

#[derive(Parser, Debug, Eq, PartialEq, Clone)]
struct Cli {
    /// Enable verbose display
    #[arg(short, long, action = ArgAction::Count)]
    verbose: u8,

    /// File to parse
    #[arg(default_value = "input")]
    path: PathBuf,
}

fn solve_part_one(lines: &[Line], verbose: u8) {
    let mut grid = Grid::new(lines, false);

    if verbose > 0 {
        grid.pretty_print();
    }

    // Add sand units until we are free falling
    let mut dropped_sand_unit_count = 0;
    loop {
        let was_blocked = grid.drop_sand_unit();
        if verbose > 1 {
            grid.pretty_print();
        }
        if !was_blocked {
            break;
        }
        dropped_sand_unit_count += 1;
    }

    if verbose > 0 {
        grid.pretty_print();
    }

    println!("Part one solution: {}", dropped_sand_unit_count);
}

fn solve_part_two(lines: &[Line], verbose: u8) {
    let mut grid = Grid::new(lines, true);

    if verbose > 0 {
        grid.pretty_print();
    }

    // Add sand units until we filled the cave
    let mut dropped_sand_unit_count = 0;
    loop {
        let was_blocked = grid.drop_sand_unit();
        if verbose > 1 {
            grid.pretty_print();
        }
        dropped_sand_unit_count += 1;
        if !was_blocked {
            break;
        }
    }

    if verbose > 0 {
        grid.pretty_print();
    }

    println!("Part one solution: {}", dropped_sand_unit_count);
}

fn main() {
    let args = Cli::parse();

    let data: String = fs::read_to_string(&args.path).unwrap();
    let lines = parser::parse_data(&data);

    solve_part_one(&lines, args.verbose);
    solve_part_two(&lines, args.verbose);
}
