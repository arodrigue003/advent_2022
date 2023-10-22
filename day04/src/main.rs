use std::{env, fs};

#[derive(Debug)]
struct Assignment {
    start: i32,
    end: i32,
}

#[derive(Debug)]
struct AssignmentPair {
    left: Assignment,
    right: Assignment,
}

impl AssignmentPair {
    fn fully_cover(&self) -> bool {
        (self.left.start <= self.right.start && self.left.end >= self.right.end)
            || (self.left.start >= self.right.start && self.left.end <= self.right.end)
    }

    fn partially_cover(&self) -> bool {
        (self.left.start <= self.right.start && self.left.end >= self.right.start)
            || (self.left.start >= self.right.start && self.left.start <= self.right.end)
    }
}

impl<'a> From<&'a str> for AssignmentPair {
    fn from(value: &'a str) -> Self {
        let (left, right) = value.split_once(',').unwrap();
        let (l1, l2) = left.split_once('-').unwrap();
        let (r1, r2) = right.split_once('-').unwrap();
        Self {
            left: Assignment {
                start: l1.parse().unwrap(),
                end: l2.parse().unwrap(),
            },
            right: Assignment {
                start: r1.parse().unwrap(),
                end: r2.parse().unwrap(),
            },
        }
    }
}

fn solve_part_one(data: &str) {
    let res: usize = data
        .lines()
        .map(|line| AssignmentPair::from(line).fully_cover())
        .filter(|cover| *cover)
        .count();

    println!("Part one solution: {:#?}", res);
}

fn solve_part_two(data: &str) {
    let res: usize = data
        .lines()
        .map(|line| AssignmentPair::from(line).partially_cover())
        .filter(|cover| *cover)
        .count();

    println!("Part one solution: {:#?}", res);
}

fn main() {
    let args: Vec<_> = env::args().collect();

    let file_path = if args.len() > 1 {
        args[1].clone()
    } else {
        "input".to_string()
    };

    let data = fs::read_to_string(&file_path).unwrap();

    solve_part_one(&data);
    solve_part_two(&data);
}
