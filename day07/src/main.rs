use std::process::exit;
use std::{env, fs};

use id_tree::Tree;

use day07::model::Inode;
use day07::parser::parse_input;
use day07::tree;
use day07::tree::pretty_print_tree;

fn solve_part_one(data: &Tree<Inode>) {
    // for node in data
    //     .traverse_post_order(data.root_node_id().unwrap())
    //     .unwrap()
    // {
    //     println!("{:#?}", node.data())
    // }

    println!("Part one solution: {:#?}", "");
}

fn solve_part_two(data: &str) {
    println!("Part two solution: {:#?}", "");
}

fn main() {
    let args: Vec<_> = env::args().collect();

    let file_path = if args.len() > 1 {
        args[1].clone()
    } else {
        "input".to_string()
    };

    let data: String = fs::read_to_string(&file_path).unwrap();

    let (res, commands) = parse_input(&data).unwrap();
    if res != "" {
        println!("Unable to fully parse the input: {}", res);
        exit(1);
    }
    let tree = tree::generate_tree_from_commands(&commands);

    pretty_print_tree(&tree);

    solve_part_one(&tree);
    solve_part_two(&data);
}
