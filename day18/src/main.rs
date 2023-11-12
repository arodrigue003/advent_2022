/// WARNING: part 02 does not work in dev mode because of stack size limitation
use std::{env, fs};

static EXPLORED_MARKER: u8 = u8::MAX;

#[derive(Debug, Eq, PartialEq, Clone)]
struct Cube {
    x: usize,
    y: usize,
    z: usize,
}

fn get_surface_area(cubes: &[Cube], x_max: usize, y_max: usize, z_max: usize) -> usize {
    // Create arrays to store cubes faces positions
    let mut x_faces = vec![vec![vec![0u8; z_max + 2]; y_max + 2]; x_max + 2];
    let mut y_faces = vec![vec![vec![0u8; z_max + 2]; y_max + 2]; x_max + 2];
    let mut z_faces = vec![vec![vec![0u8; z_max + 2]; y_max + 2]; x_max + 2];

    // Iter over cubes and add them to the arrays
    for cube in cubes {
        x_faces[cube.x][cube.y][cube.z] += 1;
        x_faces[cube.x + 1][cube.y][cube.z] += 1;
        y_faces[cube.x][cube.y][cube.z] += 1;
        y_faces[cube.x][cube.y + 1][cube.z] += 1;
        z_faces[cube.x][cube.y][cube.z] += 1;
        z_faces[cube.x][cube.y][cube.z + 1] += 1;
    }

    // Add the number of faces that are alone
    x_faces
        .iter()
        .flat_map(|plan| plan.iter().flat_map(|line| line.iter()))
        .filter(|point| **point == 1)
        .count()
        + y_faces
            .iter()
            .flat_map(|plan| plan.iter().flat_map(|line| line.iter()))
            .filter(|point| **point == 1)
            .count()
        + z_faces
            .iter()
            .flat_map(|plan| plan.iter().flat_map(|line| line.iter()))
            .filter(|point| **point == 1)
            .count()
}

fn solve_part_one(cubes: &[Cube]) {
    // Get area size
    let x_max = cubes.iter().map(|cube| cube.x).max().unwrap();
    let y_max = cubes.iter().map(|cube| cube.y).max().unwrap();
    let z_max = cubes.iter().map(|cube| cube.z).max().unwrap();

    let res = get_surface_area(cubes, x_max, y_max, z_max);

    println!("Part one solution: {:?}", &res);
}

fn explore_rec(
    area: &mut Vec<Vec<Vec<u8>>>,
    x: usize,
    y: usize,
    z: usize,
    x_max: usize,
    y_max: usize,
    z_max: usize,
) {
    if area[x][y][z] > 0 {
        return;
    }

    // Mark the position as explored
    area[x][y][z] = EXPLORED_MARKER;

    // Explore every side of the position
    if x > 0 {
        explore_rec(area, x - 1, y, z, x_max, y_max, z_max);
    }
    if x < x_max + 1 {
        explore_rec(area, x + 1, y, z, x_max, y_max, z_max);
    }
    if y > 0 {
        explore_rec(area, x, y - 1, z, x_max, y_max, z_max);
    }
    if y < y_max + 1 {
        explore_rec(area, x, y + 1, z, x_max, y_max, z_max);
    }
    if z > 0 {
        explore_rec(area, x, y, z - 1, x_max, y_max, z_max);
    }
    if z < z_max + 1 {
        explore_rec(area, x, y, z + 1, x_max, y_max, z_max);
    }
}

fn solve_part_two(cubes: &[Cube]) {
    let x_max = cubes.iter().map(|cube| cube.x).max().unwrap();
    let y_max = cubes.iter().map(|cube| cube.y).max().unwrap();
    let z_max = cubes.iter().map(|cube| cube.z).max().unwrap();

    // Compute the area of every part of the grid.
    let full_surface_area = get_surface_area(cubes, x_max, y_max, z_max);

    // Put cubes in a 3D grid
    let mut area = vec![vec![vec![0u8; z_max + 2]; y_max + 2]; x_max + 2];

    // Put the cubes inside
    for cube in cubes {
        area[cube.x][cube.y][cube.z] = 1
    }

    // Fill the grid by exploring neighbors
    if area[0][0][0] == 1 {
        unreachable!("Our solution does not cover this case")
    }
    explore_rec(&mut area, 0, 0, 0, x_max, y_max, z_max);

    // Build a new list of cubes from the empty parts of the grid
    let interior_cubes: Vec<_> = area
        .iter()
        .enumerate()
        .flat_map(move |(x, plan)| {
            plan.iter().enumerate().flat_map(move |(y, line)| {
                line.iter().enumerate().filter_map(move |(z, val)| {
                    if *val == 0 {
                        Some(Cube { x, y, z })
                    } else {
                        None
                    }
                })
            })
        })
        .collect();
    let interior_surface_area = get_surface_area(&interior_cubes, x_max, y_max, z_max);

    println!(
        "Part two solution: {}",
        full_surface_area - interior_surface_area
    );
}

fn main() {
    let args: Vec<_> = env::args().collect();

    let file_path = if args.len() > 1 {
        args[1].clone()
    } else {
        "input".to_string()
    };

    let data = fs::read_to_string(&file_path).unwrap();
    let cubes: Vec<_> = data
        .lines()
        .map(|line| {
            let (x, res) = line.split_once(",").unwrap();
            let (y, z) = res.split_once(",").unwrap();
            Cube {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
                z: z.parse().unwrap(),
            }
        })
        .collect();

    solve_part_one(&cubes);
    solve_part_two(&cubes);
}
