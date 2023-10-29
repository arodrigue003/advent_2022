use std::{env, fs};

use id_tree::Tree;

use day07::model::Inode;
use day07::parser::parse_data;
use day07::tree::{generate_tree_from_commands, pretty_print_tree};

static DISK_SPACE: i64 = 70_000_000;
static REQUIRED_SPACE: i64 = 30_000_000;

fn solve_part_one(data: &Tree<Inode>) {
    let res: i64 = data
        .traverse_pre_order(data.root_node_id().unwrap())
        .unwrap()
        .filter_map(|node| match node.data() {
            Inode::Dir(dir_entry) => Some(dir_entry.size),
            Inode::File(_) => None,
        })
        .filter(|size| *size <= 100_000)
        .sum();

    println!("Part one solution: {:#?}", res);
}

fn solve_part_two(data: &Tree<Inode>) {
    let occupied_space = match data.get(data.root_node_id().unwrap()).unwrap().data() {
        Inode::Dir(dir_entry) => dir_entry.size,
        _ => unreachable!(),
    };
    let to_delete = REQUIRED_SPACE - DISK_SPACE + occupied_space;

    let res = data
        .traverse_pre_order(data.root_node_id().unwrap())
        .unwrap()
        .filter_map(|node| match node.data() {
            Inode::Dir(dir_entry) => Some(dir_entry.size),
            Inode::File(_) => None,
        })
        .filter(|size| *size >= to_delete)
        .min()
        .unwrap();

    println!("Part two solution: {:#?}", res);
}

fn main() {
    let args: Vec<_> = env::args().collect();

    let file_path = if args.len() > 1 {
        args[1].clone()
    } else {
        "input".to_string()
    };

    let data: String = fs::read_to_string(&file_path).unwrap();

    let commands = parse_data(&data);
    let tree = generate_tree_from_commands(&commands);

    pretty_print_tree(&tree);

    solve_part_one(&tree);
    solve_part_two(&tree);
}
