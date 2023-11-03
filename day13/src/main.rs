use std::{env, fs};
use std::cmp::Ordering;

use day13::model::{PacketInner, PacketPair};
use day13::parser::parse_data;

fn solve_part_one(packet_pairs: &[PacketPair]) {
    let res: usize = packet_pairs
        .iter()
        .enumerate()
        .filter(|(_, packet_pair)| packet_pair.compare() == Ordering::Less)
        .map(|(index, _)| index + 1)
        .sum();

    println!("Part one solution: {:#?}", res);
}

fn solve_part_two(packet_pairs: &[PacketPair]) {
    let mut packets: Vec<PacketInner> = packet_pairs
        .iter()
        .flat_map(|packet_pair| [packet_pair.left.clone(), packet_pair.right.clone()])
        .collect();

    // Add the divider packets
    let first_divider = PacketInner::List(vec![PacketInner::List(vec![PacketInner::Value(2)])]);
    let second_divider = PacketInner::List(vec![PacketInner::List(vec![PacketInner::Value(6)])]);
    packets.push(first_divider.clone());
    packets.push(second_divider.clone());

    // Sort the packets
    packets.sort();

    // Find divider packet
    let first = packets.iter().position(|elt| elt == &first_divider).unwrap() + 1;
    let second = packets.iter().position(|elt| elt == &second_divider).unwrap() + 1;

    println!("Part two solution: {:#?}", first * second);
}

fn main() {
    let args: Vec<_> = env::args().collect();

    let file_path = if args.len() > 1 {
        args[1].clone()
    } else {
        "input".to_string()
    };

    let data: String = fs::read_to_string(file_path).unwrap();
    let packets_pairs = parse_data(&data);

    solve_part_one(&packets_pairs);
    solve_part_two(&packets_pairs);
}
