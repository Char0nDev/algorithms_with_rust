use std::{
    collections::HashSet,
    io::{self, BufRead},
};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut passed_levels = HashSet::new();

    let x_levels: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();
    for level in x_levels {
        passed_levels.insert(level);
    }

    let y_levels: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();
    for level in y_levels {
        passed_levels.insert(level);
    }

    if passed_levels.len() == n {
        println!("I become the guy.");
    } else {
        println!("Oh, my keyboard!");
    }
}
