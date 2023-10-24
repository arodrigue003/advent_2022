use std::{env, fs};

#[derive(Debug, Clone)]
struct Command {
    quantity: usize,
    src: usize,
    dst: usize,
}

impl From<&str> for Command {
    fn from(value: &str) -> Self {
        let (_, rem) = value.split_once("move ").unwrap();
        let (quantity, rem) = rem.split_once(" from ").unwrap();
        let (src, dst) = rem.split_once(" to ").unwrap();
        Self {
            quantity: quantity.parse().unwrap(),
            src: src.parse().unwrap(),
            dst: dst.parse().unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
struct Game {
    stacks: Vec<Vec<char>>,
    commands: Vec<Command>,
}

impl Game {
    pub fn new(data: &str) -> Self {
        // let data: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();

        // determinate the line containing the base of the stack
        // We suppose that the base has less than 9 stacks to simplify the parsing
        let mut base: usize = 0;
        let mut stack_count: usize = 0;
        for (i, line) in data.lines().enumerate() {
            if line.starts_with(" 1 ") {
                base = i;
                stack_count = (line.len() + 2) / 4;
                break;
            }
        }

        // Get an array of char for stack reading
        let stack_input: Vec<Vec<char>> = data
            .lines()
            .map(|line| line.chars().collect())
            .take(base)
            .collect();

        // Initialize the stack, we create on more vector than required in order to
        // be able to use command indices in order to access them.
        let mut stacks: Vec<Vec<char>> = vec![Vec::new(); stack_count + 1];

        // Now parse the input from the base to the top in order to fill the stacks starting pos
        for i in (0..base).rev() {
            for stack in 1..=stack_count {
                let stack_value = stack_input[i][(stack * 4) - 3];
                if stack_value != ' ' {
                    stacks[stack].push(stack_value)
                }
            }
        }

        // Parse command by skipping the base definition
        let commands: Vec<Command> = data.lines().skip(base + 2).map(From::from).collect();

        Self { stacks, commands }
    }

    fn simulate_day_01(&mut self) {
        for command in &self.commands {
            for _ in 0..command.quantity {
                let crate_to_move = self.stacks[command.src].pop().unwrap();
                self.stacks[command.dst].push(crate_to_move);
            }
        }
    }

    fn simulate_day_02(&mut self) {
        for command in &self.commands {
            let mut crates_to_move = Vec::new();
            for _ in 0..command.quantity {
                crates_to_move.push(self.stacks[command.src].pop().unwrap());
            }
            self.stacks[command.dst].extend(crates_to_move.iter().rev());
        }
    }

    fn get_result(&self) -> String {
        self.stacks
            .iter()
            .skip(1)
            .map(|stack| stack[stack.len() - 1])
            .collect()
    }
}

fn solve_part_one(mut game: Game) {
    game.simulate_day_01();
    let res = game.get_result();

    println!("Part one solution: {:#?}", res);
}

fn solve_part_two(mut game: Game) {
    game.simulate_day_02();
    let res = game.get_result();

    println!("Part one solution: {:#?}", res);
}

fn main() {
    let args: Vec<_> = env::args().collect();

    let file_path = if args.len() > 1 {
        args[1].clone()
    } else {
        "input".to_string()
    };

    let data: String = fs::read_to_string(&file_path).unwrap();
    let game = Game::new(&data);

    solve_part_one(game.clone());
    solve_part_two(game);
}
