use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Valve {
    pub name: String,
    pub flow_rate: usize,
    pub direction: Vec<String>,
}

pub type Distances = HashMap<usize, Vec<ValveGraphWithDistance>>;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ValveGraphWithDistance {
    pub node: usize,
    pub flow_rate: usize,
    pub distance: usize,
}
