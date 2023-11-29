use std::fs;
use std::path::PathBuf;

use clap::Parser;
use day16::logic;
use day16::models::Distances;

use day16::parser::parse_data;

#[derive(Parser, Debug, Eq, PartialEq, Clone)]
struct Cli {
    /// Enable verbose display
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// File to parse
    #[arg(default_value = "input")]
    path: PathBuf,
}

fn solve_part_one(start: usize, distances_from_start: &Distances) {
    println!(
        "Part one solution: {}",
        logic::optimize_flow_rate_rec_one_person(start, 0, 0, 0, 30, distances_from_start)
    );
}

fn solve_part_two(start: usize, distances_from_start: &Distances) {
    println!(
        "Part two solution: {}",
        logic::optimize_flow_rate_rec_two_person(
            start,
            start,
            0,
            0,
            0,
            0,
            26,
            distances_from_start
        )
    );
}

fn main() {
    let args = Cli::parse();

    let data: String = fs::read_to_string(args.path).unwrap();
    let valves = parse_data(&data);
    let (start, distances) = logic::compute_distances(&valves);

    solve_part_one(start, &distances);
    solve_part_two(start, &distances);
}
