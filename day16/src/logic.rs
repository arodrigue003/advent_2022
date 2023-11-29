use crate::models::{Distances, Valve, ValveGraphWithDistance};
use petgraph::algo::floyd_warshall;
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone)]
struct ValveGraph {
    node: NodeIndex,
    flow_rate: usize,
}

pub fn compute_distances(valves: &[Valve]) -> (usize, HashMap<usize, Vec<ValveGraphWithDistance>>) {
    if valves.len() >= 64 {
        panic!("Input has two much valves to work with our optimization");
    }

    // Create a graph from the grid
    let mut graph: Graph<String, usize, _> = Graph::new_undirected();

    // Create graph nodes from the valve list
    let nodes: HashMap<_, ValveGraph> = valves
        .iter()
        .map(|valve| {
            (
                valve.name.clone(),
                ValveGraph {
                    node: graph.add_node(valve.name.clone()),
                    flow_rate: valve.flow_rate,
                },
            )
        })
        .collect();

    // Add edges
    for valve in valves {
        for neighbour in &valve.direction {
            graph.add_edge(nodes[&valve.name].node, nodes[neighbour].node, 1);
        }
    }

    // Compute cost of every movement
    let distances = floyd_warshall(&graph, |e| *e.weight()).unwrap();

    // Group distances by the starting position
    let mut distances_from_start: HashMap<usize, Vec<ValveGraphWithDistance>> = HashMap::new();
    for ((start, end), distance) in distances.into_iter() {
        let flow_rate = nodes[&graph[end]].flow_rate;

        if flow_rate > 0 {
            distances_from_start
                .entry(start.index())
                .or_default()
                .push(ValveGraphWithDistance {
                    node: end.index(),
                    flow_rate,
                    distance,
                });
        }
    }

    let start = nodes["AA"].clone();
    if start.flow_rate != 0 {
        panic!("Starting node must have a null flow rate")
    }

    (start.node.index(), distances_from_start)
}

pub fn optimize_flow_rate_rec_one_person(
    memo: &mut HashMap<usize, usize>,
    current_node: usize,
    current_time: usize,
    current_score: usize,
    visited: usize,
    max_time: usize,
    distances: &Distances,
) -> usize {
    distances[&current_node]
        .iter()
        .map(|vgwd| {
            // Check that we didn't visit this node yet, we stop the recursion here in this case
            if visited & (1 << vgwd.node) > 0 {
                return current_score;
            }

            // Check that we will have the time, we stop the recursion here in this case
            if current_time + vgwd.distance + 1 >= max_time {
                return current_score;
            }

            // Compute the added score
            let new_score =
                current_score + (max_time - current_time - vgwd.distance - 1) * vgwd.flow_rate;

            let new_visited = visited | (1 << vgwd.node);

            // Update the memo
            memo.entry(new_visited)
                .and_modify(|value| *value = (*value).max(new_score))
                .or_insert(new_score);

            // Call the function recursively with the new parameter
            optimize_flow_rate_rec_one_person(
                memo,
                vgwd.node,
                current_time + vgwd.distance + 1,
                new_score,
                new_visited,
                max_time,
                distances,
            )
        })
        .max()
        .unwrap()
}
