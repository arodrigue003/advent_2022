use std::collections::HashMap;
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
    let mut _flows = HashMap::new();
    println!(
        "Part one solution: {}",
        logic::optimize_flow_rate_rec_one_person(
            &mut _flows,
            start,
            0,
            0,
            0,
            30,
            distances_from_start
        )
    );
}

fn solve_part_two(start: usize, distances_from_start: &Distances) {
    let mut flows = HashMap::new();
    logic::optimize_flow_rate_rec_one_person(&mut flows, start, 0, 0, 0, 26, distances_from_start);

    let res = flows
        .iter()
        .flat_map(|(path_1, flow_1)| {
            flows.iter().filter_map(move |(path_2, flow_2)| {
                if path_1 & path_2 == 0 {
                    Some(flow_1 + flow_2)
                } else {
                    None
                }
            })
        })
        .max()
        .unwrap();

    println!("Part two solution: {}", res);
}

fn main() {
    let args = Cli::parse();

    let data: String = fs::read_to_string(args.path).unwrap();
    let valves = parse_data(&data);
    let (start, distances) = logic::compute_distances(&valves);

    solve_part_one(start, &distances);
    solve_part_two(start, &distances);
}
