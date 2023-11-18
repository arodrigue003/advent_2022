use clap::Parser;
use day22::game::{Command, Game, MapTile, Position};
use std::fmt::Display;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug, Eq, PartialEq, Clone)]
struct Cli {
    /// Enable verbose display
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// File to parse
    #[arg(default_value = "input")]
    path: PathBuf,
}

fn solve_part_one(game: &mut Game, verbose: bool) {
    game.add_part_one_goto();
    if verbose {
        game.pretty_print();
    }

    let mut current = game.start.clone();
    let mut path: Vec<Position> = vec![current.clone()];

    for command in &game.path {
        match command {
            Command::Forward(count) => {
                for _ in 0..(*count) {
                    let next = game.go_forward(&current);
                    let next_tile = game.get_tile(&next);
                    if next_tile == MapTile::Open {
                        current = next
                    }
                    path.push(current.clone());
                }
            }
            Command::Rotate(rotation) => {
                current.direction = current.direction.rotate(*rotation);
                path.push(current.clone());
            }
        }
    }

    println!(
        "Part one solution: {:?}",
        1000 * current.line + 4 * current.column + current.direction.value()
    );
}

fn solve_part_two(data: &str) {
    println!("Part two solution:");
}

fn main() {
    let args = Cli::parse();

    let data: String = fs::read_to_string(&args.path).unwrap();
    let mut game = Game::new(&data);

    solve_part_one(&mut game, args.verbose);

    solve_part_two(&"data");
}
