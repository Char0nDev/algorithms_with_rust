use std::{
    cmp,
    io::{self, BufRead},
};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let nums: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let n = nums[0];
    let k = nums[1];
    let l = nums[2];
    let c = nums[3];
    let d = nums[4];
    let p = nums[5];
    let nl = nums[6];
    let np = nums[7];

    let total_drink_toasts = (k * l) / nl;
    let total_lime_toasts = c * d;
    let total_salt_toasts = p / np;

    let total_toasts = cmp::min(
        total_drink_toasts,
        cmp::min(total_lime_toasts, total_salt_toasts),
    );

    let toasts_per_friend = total_toasts / n;

    println!("{}", toasts_per_friend);
}
