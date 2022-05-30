// ... rust is the only language I've done AoC with that I can consistently get
// it right on the first try; its errors/type system are awesome

use std::fs;

fn main() {
    let first_sol = first_problem();
    println!("first sol: {:?}", first_sol)
}

fn get_total_rows_and_summed_columns(input: String) -> (usize, Vec<u32>) {
    input
        .lines()
        .map(|line| {
            line.split("")
                .filter(|elm| elm != &"")
                .map(|num| num.parse().unwrap())
                .collect::<Vec<u32>>()
        })
        .enumerate()
        .reduce(|prev, curr| {
            let idx = curr.0;
            let summed_vecs = prev
                .1
                .iter()
                .zip(curr.1.iter())
                .map(|(&a, &b)| a + b)
                .collect();

            (idx, summed_vecs)
        })
        .unwrap()
}

fn first_problem() -> u32 {
    let input = get_input();
    let (total_rows, summed_columns) = get_total_rows_and_summed_columns(input);
    let total_rows: u32 = total_rows.try_into().unwrap();

    let (gamma, epsilon) = summed_columns
        .iter()
        .fold((vec![], vec![]), |mut acc, num| {
            if num > &(total_rows / 2) {
                acc.0.push("1");
                acc.1.push("0");
            } else {
                acc.0.push("0");
                acc.1.push("1");
            }
            acc
        });

    bin_to_dec(gamma) * bin_to_dec(epsilon)
}

fn bin_to_dec(string: Vec<&str>) -> u32 {
    u32::from_str_radix(&string.join(""), 2).unwrap()
}

// TODO modularize for 2021
fn get_input() -> String {
    fs::read_to_string("./input").expect("something went wrong while reading input")
}
