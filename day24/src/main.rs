use std::fmt::{Display, Formatter};
use std::fs;
use std::path::PathBuf;

use clap::Parser;
use num::integer::lcm;
use petgraph::algo::astar;
use petgraph::graph::NodeIndex;
use petgraph::Graph;

static RIGHT: u8 = 1;
static BOT: u8 = 1 << 1;
static LEFT: u8 = 1 << 2;
static TOP: u8 = 1 << 3;
static WALL: u8 = 1 << 4;

#[derive(Debug, Eq, PartialEq, Clone)]
struct Point {
    line: usize,
    column: usize,
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
struct TileContent(u8);

impl TileContent {
    fn new(content: u8) -> Self {
        Self(content)
    }

    #[inline(always)]
    fn is_free(&self) -> bool {
        self.0 == 0
    }

    #[inline(always)]
    fn is_occupied(&self) -> bool {
        self.0 > 0
    }

    #[inline(always)]
    fn is_wall(&self) -> bool {
        self.0 == WALL
    }

    #[inline(always)]
    fn is_going_right(&self) -> bool {
        self.0 & RIGHT > 0
    }

    #[inline(always)]
    fn is_going_bot(&self) -> bool {
        self.0 & BOT > 0
    }

    #[inline(always)]
    fn is_going_left(&self) -> bool {
        self.0 & LEFT > 0
    }

    #[inline(always)]
    fn is_going_top(&self) -> bool {
        self.0 & TOP > 0
    }
}

impl From<char> for TileContent {
    fn from(value: char) -> Self {
        match value {
            '.' => TileContent(0),
            '>' => TileContent(RIGHT),
            'v' => TileContent(BOT),
            '<' => TileContent(LEFT),
            '^' => TileContent(TOP),
            '#' => TileContent(WALL),
            _ => unreachable!(),
        }
    }
}

impl Display for TileContent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.0.count_ones() > 1 {
            write!(f, "{}", self.0.count_ones())
        } else {
            write!(
                f,
                "{}",
                match self.0 {
                    0 => ".",
                    x if x == RIGHT => ">",
                    x if x == BOT => "v",
                    x if x == LEFT => "<",
                    x if x == TOP => "^",
                    x if x == WALL => "#",
                    _ => unreachable!(),
                }
            )
        }
    }
}

fn pretty_print_grid(grid: &[Vec<TileContent>]) {
    for line in grid {
        for tile in line {
            print!("{}", tile);
        }
        println!()
    }
    println!()
}

#[derive(Parser, Debug, Eq, PartialEq, Clone)]
struct Cli {
    /// Enable verbose display
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// File to parse
    #[arg(default_value = "input")]
    path: PathBuf,
}

fn compute_path(
    graph: &Graph<(usize, usize, usize), usize>,
    start: NodeIndex,
    end: &Point,
) -> (usize, NodeIndex) {
    let (len, path) = astar(
        &graph,
        start,
        |finish| {
            let weight = &graph[finish];
            weight.1 == end.line && weight.2 == end.column
        },
        |e| *e.weight(),
        |_| 0,
    )
    .unwrap();

    (len, path[path.len() - 1])
}

fn main() {
    let args = Cli::parse();

    let data: String = fs::read_to_string(&args.path).unwrap();

    // Parse the initial grid
    let start_grid: Vec<Vec<TileContent>> = data
        .lines()
        .map(|line| line.chars().map(From::from).collect())
        .collect();

    // Verbose output
    if args.verbose {
        pretty_print_grid(&start_grid);
    }

    // Get grid size and compute cycle size
    let width = start_grid[0].len();
    let height = start_grid.len();
    let cycle_len = lcm(width - 2, height - 2);

    // Detect start and end position
    let start = Point {
        line: 0,
        column: start_grid[0]
            .iter()
            .position(|tile| tile.is_free())
            .unwrap(),
    };
    let end = Point {
        line: height - 1,
        column: start_grid[height - 1]
            .iter()
            .position(|tile| tile.is_free())
            .unwrap(),
    };

    // Generate every next grid
    let mut grids: Vec<Vec<Vec<TileContent>>> = vec![];

    // Add the first one
    grids.push(start_grid.clone());

    // Add the remaining ones
    for _ in 0..cycle_len - 1 {
        let current = &grids[grids.len() - 1];
        let mut next_grid = vec![vec![TileContent(0); width]; height];

        for i_line in 0..height {
            for i_column in 0..width {
                if i_line == 0 || i_line == height - 1 || i_column == 0 || i_column == width - 1 {
                    next_grid[i_line][i_column] = TileContent::new(WALL);
                    continue;
                }

                let tile = &current[i_line][i_column];
                if tile.is_going_right() {
                    next_grid[i_line][i_column % (width - 2) + 1].0 |= RIGHT;
                }
                if tile.is_going_bot() {
                    next_grid[i_line % (height - 2) + 1][i_column].0 |= BOT;
                }
                if tile.is_going_left() {
                    next_grid[i_line][(i_column + width - 4) % (width - 2) + 1].0 |= LEFT;
                }
                if tile.is_going_top() {
                    next_grid[(i_line + height - 4) % (height - 2) + 1][i_column].0 |= TOP;
                }
            }
        }

        // Add start and end back
        next_grid[start.line][start.column] = TileContent::new(0);
        next_grid[end.line][end.column] = TileContent::new(0);
        if args.verbose {
            pretty_print_grid(&next_grid);
        }

        // Append the grid to the list of grids
        grids.push(next_grid);
    }

    // Add the first one again in order to be able to loop
    grids.push(start_grid);

    // Create a graph for the grid
    let mut graph: Graph<(usize, usize, usize), usize> = Graph::new();

    // Create graph nodes from the grid
    let nodes: Vec<Vec<Vec<_>>> = (0..cycle_len)
        .map(|generation| {
            (0..height)
                .map(|i_line| {
                    (0..width)
                        .map(|i_column| graph.add_node((generation, i_line, i_column)))
                        .collect()
                })
                .collect()
        })
        .collect();

    // Add the connections
    for generation in 0..cycle_len {
        // Add elements
        for i_line in 1..height - 1 {
            for i_column in 1..width - 1 {
                // If the current element is not free, skip for the next part
                if grids[generation][i_line][i_column].is_occupied() {
                    continue;
                }

                // Check same position
                if grids[generation + 1][i_line][i_column].is_free() {
                    graph.add_edge(
                        nodes[generation][i_line][i_column],
                        nodes[(generation + 1) % cycle_len][i_line][i_column],
                        1,
                    );
                }

                // Check right
                if grids[generation + 1][i_line][i_column + 1].is_free() {
                    graph.add_edge(
                        nodes[generation][i_line][i_column],
                        nodes[(generation + 1) % cycle_len][i_line][i_column + 1],
                        1,
                    );
                }

                // Check bottom
                if grids[generation + 1][i_line + 1][i_column].is_free() {
                    graph.add_edge(
                        nodes[generation][i_line][i_column],
                        nodes[(generation + 1) % cycle_len][i_line + 1][i_column],
                        1,
                    );
                }

                // check left
                if grids[generation + 1][i_line][i_column - 1].is_free() {
                    graph.add_edge(
                        nodes[generation][i_line][i_column],
                        nodes[(generation + 1) % cycle_len][i_line][i_column - 1],
                        1,
                    );
                }

                // check top
                if grids[generation + 1][i_line - 1][i_column].is_free() {
                    graph.add_edge(
                        nodes[generation][i_line][i_column],
                        nodes[(generation + 1) % cycle_len][i_line - 1][i_column],
                        1,
                    );
                }
            }
        }

        // Add start chain and going out
        graph.add_edge(
            nodes[generation][start.line][start.column],
            nodes[(generation + 1) % cycle_len][start.line][start.column],
            1,
        );
        if grids[generation + 1][start.line + 1][start.column].is_free() {
            graph.add_edge(
                nodes[generation][start.line][start.column],
                nodes[(generation + 1) % cycle_len][start.line + 1][start.column],
                1,
            );
        }

        // Add end chain and going out
        graph.add_edge(
            nodes[generation][end.line][end.column],
            nodes[(generation + 1) % cycle_len][end.line][end.column],
            1,
        );
        if grids[generation + 1][end.line - 1][end.column].is_free() {
            graph.add_edge(
                nodes[generation][end.line][end.column],
                nodes[(generation + 1) % cycle_len][end.line - 1][end.column],
                1,
            );
        }
    }

    // Find the path
    let (len1, new_start) = compute_path(&graph, nodes[0][start.line][start.column], &end);
    println!("Part one solution: {}", len1);

    // Going back to the start and to the end again
    let (len2, new_start) = compute_path(&graph, new_start, &start);
    let (len3, _) = compute_path(&graph, new_start, &end);
    println!("Part two solution: {}", len1 + len2 + len3);
}
