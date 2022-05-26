/*
 * first problem: should have used .array_windows() off of the collection
 **/

use std::fs;

fn main() {
    first_problem();
    second_problem();
}

fn first_problem() {
    let day_1_input = fs::read_to_string("./input")
        .expect("something went wrong while reading input for 2021/day-1");

    let values = day_1_input
        .lines()
        .map(|line| line.to_string().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let total_values_indices = values.len() - 1;

    let depth_increases_count =
        values
            .clone()
            .into_iter()
            .enumerate()
            .fold(0, |acc, (idx, curr)| {
                let next_idx = idx + 1;
                if next_idx <= total_values_indices && curr < values[next_idx] {
                    return acc + 1;
                }
                acc
            });

    println!("depth increases count: {}", depth_increases_count)
}

fn second_problem() {
    let day_1_input = fs::read_to_string("./input")
        .expect("something went wrong while reading input for 2021/day-1");
}
