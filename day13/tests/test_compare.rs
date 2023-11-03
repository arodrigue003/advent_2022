use day13::parser::parse_packet_pair;
use std::cmp::Ordering;

#[test]
fn test_compare_simple_list() {
    let to_parse: &'static str = "[1,1,3,1,1]
[1,1,5,1,1]
";

    let packet_pair = parse_packet_pair(to_parse).unwrap().1;

    assert_eq!(packet_pair.compare(), Ordering::Less);
}
