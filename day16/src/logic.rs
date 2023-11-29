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
            let added_score = (max_time - current_time - vgwd.distance - 1) * vgwd.flow_rate;

            // Call the function recursively with the new parameter
            optimize_flow_rate_rec_one_person(
                vgwd.node,
                current_time + vgwd.distance + 1,
                current_score + added_score,
                visited | (1 << vgwd.node),
                max_time,
                distances,
            )
        })
        .max()
        .unwrap()
}

#[allow(clippy::too_many_arguments)]
pub fn optimize_flow_rate_rec_two_person(
    current_node_1: usize,
    current_node_2: usize,
    current_time_1: usize,
    current_time_2: usize,
    current_score: usize,
    visited: usize,
    max_time: usize,
    distances: &Distances,
) -> usize {
    distances[&current_node_1]
        .iter()
        .flat_map(|vgwd_1| {
            distances[&current_node_2].iter().map(|vgwd_2| {
                // Don't let them go at the same destination
                if vgwd_1.node == vgwd_2.node {
                    return current_score;
                }

                // Check which one is ok
                let first_is_ok = visited & (1 << vgwd_1.node) == 0
                    && current_time_1 + vgwd_1.distance + 1 < max_time;
                let second_is_ok = visited & (1 << vgwd_2.node) == 0
                    && current_time_2 + vgwd_2.distance + 1 < max_time;

                // Action depends on the status of each part
                match (first_is_ok, second_is_ok) {
                    (false, false) => current_score,
                    (false, true) => {
                        // One is out, fallback on optimize_flow_rate_rec_one_person
                        let added_score =
                            (max_time - current_time_2 - vgwd_2.distance - 1) * vgwd_2.flow_rate;

                        optimize_flow_rate_rec_one_person(
                            vgwd_2.node,
                            current_time_2 + vgwd_2.distance + 1,
                            current_score + added_score,
                            visited | (1 << vgwd_2.node),
                            max_time,
                            distances,
                        )
                    }
                    (true, false) => {
                        // Two is out, fallback on optimize_flow_rate_rec_one_person
                        let added_score =
                            (max_time - current_time_1 - vgwd_1.distance - 1) * vgwd_1.flow_rate;

                        optimize_flow_rate_rec_one_person(
                            vgwd_1.node,
                            current_time_1 + vgwd_1.distance + 1,
                            current_score + added_score,
                            visited | (1 << vgwd_1.node),
                            max_time,
                            distances,
                        )
                    }
                    (true, true) => {
                        // keep going with both of them
                        let added_score = (max_time - current_time_2 - vgwd_2.distance - 1)
                            * vgwd_2.flow_rate
                            + (max_time - current_time_1 - vgwd_1.distance - 1) * vgwd_1.flow_rate;

                        optimize_flow_rate_rec_two_person(
                            vgwd_1.node,
                            vgwd_2.node,
                            current_time_1 + vgwd_1.distance + 1,
                            current_time_2 + vgwd_2.distance + 1,
                            current_score + added_score,
                            visited | (1 << vgwd_1.node) | (1 << vgwd_2.node),
                            max_time,
                            distances,
                        )
                    }
                }
            })
        })
        .max()
        .unwrap()
}
