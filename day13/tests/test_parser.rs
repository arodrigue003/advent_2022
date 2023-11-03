use day13::model::{PacketInner, PacketPair};
use day13::parser::{parse_packet_inner, parse_packet_pair};

#[test]
fn test_parse_packet_pair() {
    let to_parse: &'static str = "[1,1,3,1,1]
[1,1,5,1,1]

";

    assert_eq!(
        parse_packet_pair(to_parse),
        Ok((
            "",
            PacketPair {
                left: PacketInner::List(vec![
                    PacketInner::Value(1),
                    PacketInner::Value(1),
                    PacketInner::Value(3),
                    PacketInner::Value(1),
                    PacketInner::Value(1)
                ]),
                right: PacketInner::List(vec![
                    PacketInner::Value(1),
                    PacketInner::Value(1),
                    PacketInner::Value(5),
                    PacketInner::Value(1),
                    PacketInner::Value(1)
                ])
            }
        ))
    )
}

#[test]
fn test_parse_packet_inner_simple_list() {
    assert_eq!(
        parse_packet_inner("[1,1,3,1,1]"),
        Ok((
            "",
            PacketInner::List(vec![
                PacketInner::Value(1),
                PacketInner::Value(1),
                PacketInner::Value(3),
                PacketInner::Value(1),
                PacketInner::Value(1)
            ])
        ))
    )
}

#[test]
fn test_parse_packet_inner_nested_list() {
    assert_eq!(
        parse_packet_inner("[[1],[2,3,4]]"),
        Ok((
            "",
            PacketInner::List(vec![
                PacketInner::List(vec![PacketInner::Value(1)]),
                PacketInner::List(vec![
                    PacketInner::Value(2),
                    PacketInner::Value(3),
                    PacketInner::Value(4)
                ]),
            ])
        ))
    )
}

#[test]
fn test_parse_packet_inner_nested_mixed_list() {
    assert_eq!(
        parse_packet_inner("[[4,4],4,4]"),
        Ok((
            "",
            PacketInner::List(vec![
                PacketInner::List(vec![PacketInner::Value(4), PacketInner::Value(4)]),
                PacketInner::Value(4),
                PacketInner::Value(4)
            ])
        ))
    )
}

#[test]
fn test_parse_packet_inner_empty_list() {
    assert_eq!(
        parse_packet_inner("[]"),
        Ok(("", PacketInner::List(vec![])))
    )
}

#[test]
fn test_parse_packet_inner_nested_empty_list() {
    assert_eq!(
        parse_packet_inner("[[[]]]"),
        Ok((
            "",
            PacketInner::List(vec![PacketInner::List(vec![PacketInner::List(vec![])])])
        ))
    )
}
