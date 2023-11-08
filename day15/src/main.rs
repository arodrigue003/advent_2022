use clap::Parser;
use day15::parser;
use day15::sensor_scan::SensorScan;
use day15::sparse_interval::SparseInterval;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug, Eq, PartialEq, Clone)]
struct Cli {
    /// Enable verbose display
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// Line to consider for part 1 solving
    #[arg(short, long, default_value_t = 2_000_000)]
    line: i64,

    /// Search space size to consider for part 2 solving
    #[arg(short, long, default_value_t = 4_000_000)]
    space_limit: i64,

    /// File to parse
    #[arg(default_value = "input")]
    path: PathBuf,
}

fn solve_part_one(sensor_scans: &[SensorScan], line: i64) {
    // Compute the union of sensor interval on the given line to determinate where it is not
    // possible to have an unknown beacon.
    let mut sparse_interval = SparseInterval::new();
    let mut beacon_pos: HashSet<i64> = HashSet::new();
    for sensor_scan in sensor_scans {
        // Add the beacon position sensor associated beacon is on the target line
        if sensor_scan.beacon.y == line {
            beacon_pos.insert(sensor_scan.beacon.x);
        }

        // Merge the interval if needed
        if let Some(interval) = sensor_scan.get_line_intersection_interval(line) {
            sparse_interval.add_interval(&interval)
        }
    }

    println!(
        "Part one solution: {}",
        sparse_interval.size() as usize - beacon_pos.len()
    );
}

fn solve_part_two(sensor_scans: &[SensorScan], space_limit: i64) {
    for line in 0..space_limit {
        // Create a sparse interval.
        let mut sparse_interval = SparseInterval::new();

        // Fill it
        for sensor_scan in sensor_scans {
            if let Some(interval) = sensor_scan.get_line_intersection_interval(line) {
                sparse_interval.add_interval(&interval)
            }
        }

        // Check that the sparser interval is full for this line
        if !sparse_interval.contains(&(0, space_limit)) {
            println!(
                "Part two solution: {}",
                sparse_interval.get_missing_elements(&(0, space_limit))[0] * 4_000_000 + line
            );
            break;
        }
    }
}

fn main() {
    let args = Cli::parse();

    let data: String = fs::read_to_string(&args.path).unwrap();
    let sensor_scans = parser::parse_data(&data);

    solve_part_one(&sensor_scans, args.line);
    solve_part_two(&sensor_scans, args.space_limit);
}
