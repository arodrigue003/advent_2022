use std::cmp::Ordering;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum PacketInner {
    List(Vec<PacketInner>),
    Value(i32),
}

impl PartialOrd for PacketInner {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PacketInner {
    fn cmp(&self, other: &Self) -> Ordering {
        // Less means right order
        // Greater means wrong order
        // Equal should not be possible
        match (self, other) {
            (PacketInner::Value(left), PacketInner::Value(right)) => left.cmp(right),
            (PacketInner::List(left), PacketInner::List(right)) => {
                for i in 0..left.len().min(right.len()) {
                    // Compare elements one by one if while their is one in both lists
                    let cmp_res = left[i].cmp(&right[i]);
                    if cmp_res != Ordering::Equal {
                        return cmp_res;
                    }
                }

                // Compare list sizes
                left.len().cmp(&right.len())
            }
            (PacketInner::Value(left), right) => {
                PacketInner::List(vec![PacketInner::Value(*left)]).cmp(right)
            }
            (left, PacketInner::Value(right)) => {
                left.cmp(&PacketInner::List(vec![PacketInner::Value(*right)]))
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PacketPair {
    pub left: PacketInner,
    pub right: PacketInner,
}

impl PacketPair {
    pub fn compare(&self) -> Ordering {
        self.left.partial_cmp(&self.right).unwrap()
    }
}
