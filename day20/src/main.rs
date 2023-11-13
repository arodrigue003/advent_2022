use std::{env, fs};

fn mix_data(data: &[i64], array: &mut Vec<usize>) {
    let len = data.len();

    for (array_pos, value) in data.iter().enumerate() {
        let value_pos = array.iter().position(|elt| *elt == array_pos).unwrap();
        let new_pos = get_new_pos(len, *value, value_pos);

        // Remove the value
        array.remove(value_pos);

        // Add the new value
        array.insert(new_pos, array_pos);
    }
}

#[inline(always)]
fn get_new_pos(array_len: usize, value: i64, value_position: usize) -> usize {
    let new_pos = (value_position as i64 + value) % (array_len - 1) as i64;

    if new_pos > 0 {
        new_pos as usize
    } else {
        (array_len as i64 + new_pos - 1) as usize
    }
}

fn solve_part_one(data: &[i64]) {
    // Setup an array of number positions
    let len = data.len();
    let mut array: Vec<usize> = (0..len).collect();

    // Move data around
    mix_data(data, &mut array);

    // Get 0 position
    let zero_pos_in_data = data.iter().position(|elt| *elt == 0).unwrap();
    let zero_pos_in_array = array
        .iter()
        .position(|elt| *elt == zero_pos_in_data)
        .unwrap();

    let res: i64 = [1000, 2000, 3000]
        .map(|pos| data[array[(zero_pos_in_array + pos) % len]])
        .iter()
        .sum();

    println!("Part one solution: {}", res);
}

fn solve_part_two(data: &[i64]) {
    // Multiply data by the key
    let data: Vec<i64> = data.iter().map(|elt| *elt * 811589153).collect();

    // Setup an array of number positions
    let len = data.len();
    let mut array: Vec<usize> = (0..len).collect();

    // Move data around
    for _ in 0..10 {
        mix_data(&data, &mut array);
    }

    // Get 0 position
    let zero_pos_in_data = data.iter().position(|elt| *elt == 0).unwrap();
    let zero_pos_in_array = array
        .iter()
        .position(|elt| *elt == zero_pos_in_data)
        .unwrap();

    let res: i64 = [1000, 2000, 3000]
        .map(|pos| data[array[(zero_pos_in_array + pos) % len]])
        .iter()
        .sum();

    println!("Part two solution: {}", res);
}

fn main() {
    let args: Vec<_> = env::args().collect();

    let file_path = if args.len() > 1 {
        args[1].clone()
    } else {
        "input".to_string()
    };

    let data: Vec<i64> = fs::read_to_string(&file_path)
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    solve_part_one(&data);
    solve_part_two(&data);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_get_new_pos() {
        // Test from the day examples
        assert_eq!(get_new_pos(7, 1, 0), 1);
        assert_eq!(get_new_pos(7, 2, 0), 2);
        assert_eq!(get_new_pos(7, -3, 1), 4);
        assert_eq!(get_new_pos(7, 3, 2), 5);
        assert_eq!(get_new_pos(7, -2, 2), 6);
        assert_eq!(get_new_pos(7, 0, 3), 3);
        assert_eq!(get_new_pos(7, 4, 5), 3);

        // More than one loop
        assert_eq!(get_new_pos(7, 11, 4), 3);
        assert_eq!(get_new_pos(7, -11, 4), 5);
    }
}
