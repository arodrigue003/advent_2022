use std::collections::HashSet;
use std::{env, fs};

#[derive(Debug, Eq, PartialEq, Clone)]
enum Instruction {
    Addx(i32),
    Noop,
}

impl<'a> From<&'a str> for Instruction {
    fn from(value: &'a str) -> Self {
        if value == "noop" {
            return Self::Noop;
        }

        if value.starts_with("addx") {
            let (_, val) = value.split_once(" ").unwrap();
            return Self::Addx(val.parse().unwrap());
        }

        unreachable!()
    }
}

fn solve_part_one(instructions: &[Instruction]) {
    let mut register: i32 = 1;
    let mut cycle: i32 = 1;
    let mut signal_strength: i32 = 0;
    let target_cycles: HashSet<i32> = [20, 60, 100, 140, 180, 220].into_iter().collect();

    for instruction in instructions {
        match instruction {
            Instruction::Addx(val) => {
                // If the addx operation will happen during the target cycle
                if target_cycles.contains(&cycle) {
                    signal_strength += cycle * register;
                }
                if target_cycles.contains(&(cycle + 1)) {
                    signal_strength += (cycle + 1) * register;
                }

                register += val;
                cycle += 2;
            }
            Instruction::Noop => {
                // if we are during one of the target cycle
                if target_cycles.contains(&cycle) {
                    signal_strength += cycle * register;
                }
                cycle += 1;
            }
        }
    }

    println!("Part one solution: {:#?}", signal_strength);
}

fn solve_part_two(instructions: &[Instruction]) {
    let mut register: i32 = 1;
    let mut cycle: i32 = 1;
    let mut screen: [String; 6] = Default::default();

    for instruction in instructions {
        match instruction {
            Instruction::Addx(val) => {
                // Handle current cycle
                let screen_pos = (cycle - 1) % 40;
                let line = ((cycle - 1) / 40) as usize;
                if (register - screen_pos).abs() <= 1 {
                    screen[line].push('#')
                } else {
                    screen[line].push('.')
                }

                // handle next cycle
                let screen_pos = cycle % 40;
                let line = ((cycle) / 40) as usize;
                if (register - screen_pos).abs() <= 1 {
                    screen[line].push('#')
                } else {
                    screen[line].push('.')
                }

                register += val;
                cycle += 2;
            }
            Instruction::Noop => {
                // Handle current cycle
                let screen_pos = (cycle - 1) % 40;
                let line = ((cycle - 1) / 40) as usize;
                if (register - screen_pos).abs() <= 1 {
                    screen[line].push('#')
                } else {
                    screen[line].push('.')
                }

                cycle += 1
            }
        }
    }

    println!("Part two solution: {:#?}", screen);
}

fn main() {
    let args: Vec<_> = env::args().collect();

    let file_path = if args.len() > 1 {
        args[1].clone()
    } else {
        "input".to_string()
    };

    let data: String = fs::read_to_string(&file_path).unwrap();
    let instructions: Vec<Instruction> = data.lines().map(From::from).collect();

    solve_part_one(&instructions);
    solve_part_two(&instructions);
}
