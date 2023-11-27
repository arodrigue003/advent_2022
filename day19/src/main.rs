use rayon::prelude::*;

use std::fs;
use std::path::PathBuf;
use std::time::Instant;

use clap::Parser;
use day19::logic::{simulate_game, simulate_game_heuristic, simulate_game_with_scout};
use day19::models::Blueprint;
use day19::parser::parse_data;

#[derive(Parser, Debug, Eq, PartialEq, Clone)]
struct Cli {
    /// Enable verbose display
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// File to parse
    #[arg(default_value = "input")]
    path: PathBuf,
}

fn solve_part_one(blueprints: &[Blueprint]) {
    // let now = Instant::now();
    //
    // let res: usize = blueprints
    //     .iter()
    //     .map(|blueprint| simulate_game(24, blueprint))
    //     .sum();
    //
    // println!(
    //     "Part one solution (simulate_game): {} in {} ms",
    //     res,
    //     now.elapsed().as_millis()
    // );

    let now = Instant::now();

    let res: usize = blueprints
        .iter()
        .map(|blueprint| simulate_game_with_scout(24, blueprint))
        .sum();

    println!(
        "Part one solution (simulate_game_with_scout): {} in {} ms",
        res,
        now.elapsed().as_millis()
    );
}

fn solve_part_two(blueprints: &[Blueprint]) {
    let res: usize = blueprints
        .par_iter()
        .take(3)
        .map(|blueprint| simulate_game_heuristic(32, blueprint))
        .product();

    println!("Part two solution: {}", res);
}

fn main() {
    let args = Cli::parse();

    let data: String = fs::read_to_string(&args.path).unwrap();
    let blueprints = parse_data(&data);

    solve_part_one(&blueprints);
    // solve_part_two(&blueprints);
}
