use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input = File::open("input.txt").unwrap();
    let reader = BufReader::new(input);

    // parse input
    let mut left_list: Vec<u32> = Vec::new();
    let mut right_list: Vec<u32> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let parts = line.split_whitespace();
        let parts: Vec<String> = parts.map(|p| p.to_string()).collect();

        // get the actual numbers and parse to u16
        let left = parts[0].parse::<u32>().unwrap();
        let right = parts[1].parse::<u32>().unwrap();

        left_list.push(left);
        right_list.push(right);
    }

    // prep lists
    left_list.sort();
    right_list.sort();

    // part 1 calculation
    let tot: i32 = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(&left, &right)| i32::abs(left as i32 - right as i32))
        .sum();

    println!("Total sum of pairwise elements: {}", tot);

    // part 2 calculation
    let similarity: u32 = left_list
        .iter()
        .map(|lnum| {
            let cnt = right_list.iter().filter(|rnum| **rnum == *lnum).count() as u32;
            cnt * lnum
        })
        .sum();

    println!("Total similarity: {}", similarity);
}
