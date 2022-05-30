// whew, the second problem has a bug in it that's hard to think through

use std::fs;

fn main() {
    let first_sol = first_problem();
    let second_sol = second_problem();
    println!("first sol: {:?}", first_sol);
    println!("second sol: {:?}", second_sol);
}

fn get_total_rows_and_summed_columns(input: String) -> (u32, Vec<u32>) {
    let (total_rows, summed_columns) = input
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
        .unwrap();

    let total_rows: u32 = total_rows.try_into().unwrap();

    (total_rows, summed_columns)
}

fn first_problem() -> u32 {
    let input = get_input();
    let (total_rows, summed_columns) = get_total_rows_and_summed_columns(input);
    // half of the number at the idx of the vec; it represents whether we've more
    // 1s than 0s (ie, if `num > half`, we know we've more 1s than 0s)
    let half = &(total_rows / 2);

    let (gamma, epsilon) = summed_columns
        .iter()
        .fold((vec![], vec![]), |mut acc, num| {
            if num > half {
                acc.0.push("1");
                acc.1.push("0");
            } else {
                acc.0.push("0");
                acc.1.push("1");
            }
            acc
        });

    let gamma = gamma.join("");
    let epsilon = epsilon.join("");

    bin_to_dec(gamma) * bin_to_dec(epsilon)
}

fn second_problem() -> u32 {
    let input = get_input();

    let values = input
        .lines()
        .map(|line| {
            line.split("")
                .filter(|elm| elm != &"")
                .map(|num| num.parse().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let final_idx = values.len() - 1;
    let a = better_name(0, values.clone(), true, final_idx);
    let b = better_name(0, values, false, final_idx);

    println!("a: {:?}, b: {:?}", a, b);
    a * b
}

fn sum_rows_at_column_idx(idx: usize, values: &[Vec<u32>]) -> u32 {
    values.iter().flat_map(|row| row.get(idx)).sum()
}

fn get_target_int(half: u32, row_sum: u32, most_common: bool) -> usize {
    if most_common {
        // `1` is tiebreaker
        if row_sum >= half {
            return 1;
        } else {
            return 0;
        }
    }

    if row_sum < half {
        1
    } else {
        0
    }
}

fn better_name(mut idx: usize, values: Vec<Vec<u32>>, most_common: bool, final_idx: usize) -> u32 {
    let half: u32 = (values.len() / 2).try_into().unwrap();
    let row_sum = sum_rows_at_column_idx(idx, &values);
    let target_int = get_target_int(half, row_sum, most_common);
    println!(
        "most_common {}, half {}, row_sum {}, target_int {}",
        most_common, half, row_sum, target_int
    );

    let vals_bk = values.clone();
    let mut values = values
        // into_iter gives ownership of vals; iter gives refs
        .into_iter()
        .filter(|sub_vec| sub_vec[idx] == target_int.try_into().unwrap())
        .collect::<Vec<Vec<u32>>>();

    if values.is_empty() {
        values = vals_bk;
    }

    //println!("{:?}\n\n", values);
    let col_len = values.len();
    println!("col_len {}, idx {}", col_len, idx);
    println!("{:?}\n", values);
    if col_len > 1 {
        idx += 1;
        return better_name(idx, values, most_common, final_idx);
    }

    let bin_string = values[0]
        .iter()
        .map(|num| if *num == 0 { "0" } else { "1" })
        .collect::<String>();

    bin_to_dec(bin_string)
}

fn bin_to_dec(string: String) -> u32 {
    u32::from_str_radix(&string, 2).unwrap()
}

// TODO modularize for 2021
fn get_input() -> String {
    fs::read_to_string("./input").expect("something went wrong while reading input")
}
