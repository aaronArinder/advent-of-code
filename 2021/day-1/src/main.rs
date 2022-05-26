/*
 * first problem: should have used .array_windows() off of the collection, but only
 * available in unstable release
 *
 * second problem: turns out there's a .windows that's stable
 **/

use std::fs;

fn main() {
    let first_sol = first_problem();
    let first_sol_refactor = first_problem_refactored();

    assert!(first_sol == first_sol_refactor);

    let second_sol = second_problem();
    println!("first sol: {}\nsecond sol: {}", first_sol, second_sol);
}

fn get_input() -> String {
    fs::read_to_string("./input").expect("something went wrong while reading input for 2021/day-1")
}

// original
fn first_problem() -> i32 {
    let day_1_input = get_input();
    let values = day_1_input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<i32>>();

    let total_values_indices = values.len() - 1;

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
        })
}

// refactored based on others' answers, especially .arrow_windows() (unstable, using
// .windows(N))
fn first_problem_refactored() -> i32 {
    get_input()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<i32>>()
        .windows(2)
        // same dstructuring funk as prob 2
        .filter(|window| window[0] < window[1])
        .count()
        .try_into()
        .unwrap()
}

fn second_problem() -> i32 {
    get_input()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<i32>>()
        .windows(3)
        // had trouble destructuring, figure out why: .map(|[a,b,c]| a + b + c)
        // refutable pattern in function argument: `&[]`, `&[_]`, `&[_, _]` and 1 more
        // not covered the matched value is of type `&[i32]`
        .map(|window| window[0] + window[1] + window[2])
        // there has to be a way to avoid collecting here
        .collect::<Vec<i32>>()
        .windows(2)
        // same destructuring funkiness as above
        .filter(|window| window[0] < window[1])
        .count()
        .try_into()
        .unwrap()
}
