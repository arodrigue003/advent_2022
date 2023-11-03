use clap::Parser;
use day12::grid::{GridWithBorder, Point};
use petgraph::algo::astar;
use petgraph::Graph;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug, Eq, PartialEq, Clone)]
struct Cli {
    /// Enable verbose display
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// File to parse
    #[arg(default_value = "input")]
    path: PathBuf,
}

fn main() {
    let args = Cli::parse();

    let data: String = fs::read_to_string(&args.path).unwrap();
    let grid = GridWithBorder::from_str(&data);

    if args.verbose {
        grid.pretty_print();
    }

    // Create a graph from the grid
    let mut graph: Graph<Point, usize> = Graph::new();

    // Create graph nodes from the grid
    let nodes: Vec<Vec<_>> = (0..grid.height())
        .into_iter()
        .map(|line| {
            (0..grid.width())
                .into_iter()
                .map(|column| graph.add_node(Point { line, column }))
                .collect()
        })
        .collect();

    // Create graph edges if possible
    for line in 0..grid.height() {
        for column in 0..grid.width() {
            for neighbor in grid.get_neighbors(line, column) {
                graph.add_edge(
                    nodes[line][column],
                    nodes[neighbor.line][neighbor.column],
                    1,
                );
            }
        }
    }

    let start = nodes[grid.start().line][grid.start().column];
    let end = nodes[grid.end().line][grid.end().column];

    let (len, path) = astar(
        &graph,
        start,
        |finish| finish == end,
        |e| *e.weight(),
        |_| 0,
    )
    .unwrap();
    let path: Vec<_> = path
        .into_iter()
        .map(|node_index| graph.node_weight(node_index).unwrap().clone())
        .collect();

    if args.verbose {
        grid.pretty_print_path(&path);
    }

    println!("Part one solution: {:#?}", len);

    // For every point of the graph that is at elevation a
    // This is not an optimal answer, we should instead build a graph that starts from the end
    // and has any point with the elevation a as a possible end
    let to_test = grid.get_a_elevation_list();
    let res = to_test.into_iter().filter_map(|to_test| {
        let start = nodes[to_test.line][to_test.column];
        astar(
            &graph,
            start,
            |finish| finish == end,
            |e| *e.weight(),
            |_| 0,
        )
    }).map(|(len, _)| len).min().unwrap();
    println!("Part two solution: {:#?}", res);

}
