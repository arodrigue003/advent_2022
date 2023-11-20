use clap::Parser;
use day22::enums::{Command, MapTile};
use day22::game::Game;
use day22::structs::Position;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug, Eq, PartialEq, Clone)]
struct Cli {
    /// Enable verbose display
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// Width of a cube face
    #[arg(short, long, default_value_t = 50)]
    face_width: usize,

    /// File to parse
    #[arg(default_value = "input")]
    path: PathBuf,
}

fn solve_part_one(game: &mut Game, verbose: bool) {
    game.add_part_one_goto();
    if verbose {
        game.pretty_print();
    }

    let arrival = simulate(&game);

    println!(
        "Part one solution: {:?}",
        1000 * arrival.line + 4 * arrival.column + arrival.direction.value()
    );
}

fn solve_part_two(game: &mut Game, verbose: bool) {
    game.add_part_two_goto();
    if verbose {
        game.pretty_print();
    }

    let arrival = simulate(&game);

    println!(
        "Part two solution: {:?}",
        1000 * arrival.line + 4 * arrival.column + arrival.direction.value()
    );
}

fn simulate(game: &&mut Game) -> Position {
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
    current
}

fn main() {
    let args = Cli::parse();

    let data: String = fs::read_to_string(&args.path).unwrap();
    let mut game = Game::new(&data, args.face_width);

    solve_part_one(&mut game, args.verbose);

    solve_part_two(&mut game, args.verbose);
}
