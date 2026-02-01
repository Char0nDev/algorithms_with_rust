/* https://codeforces.com/contest/2193/problem/C */

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().and_then(|l| l.ok()).unwrap_or_default();
    let repeat: usize = first_line.trim().parse().unwrap_or(0);

    for _ in 0..repeat {
        let n_q_line = lines.next().and_then(|l| l.ok()).unwrap_or_default();
        let n_q: Vec<usize> = n_q_line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let (n, q) = (n_q[0], n_q[1]);

        let a_line = lines.next().and_then(|l| l.ok()).unwrap_or_default();
        let b_line = lines.next().and_then(|l| l.ok()).unwrap_or_default();

        let a_vec: Vec<u32> = a_line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let b_vec: Vec<u32> = b_line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let mut v: Vec<u64> = (0..n)
            .map(|i| std::cmp::max(a_vec[i], b_vec[i]) as u64)
            .collect();

        for i in (0..n - 1).rev() {
            v[i] = std::cmp::max(v[i], v[i + 1]);
        }

        let mut prefix_sum = vec![0u64; n + 1];
        for i in 0..n {
            prefix_sum[i + 1] = prefix_sum[i] + v[i];
        }

        let mut results = Vec::with_capacity(q);

        for _ in 0..q {
            let q_line = lines.next().and_then(|l| l.ok()).unwrap_or_default();
            let query: Vec<usize> = q_line
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            let (l, r) = (query[0], query[1]);

            let range_sum = prefix_sum[r] - prefix_sum[l - 1];
            results.push(range_sum.to_string());
        }

        println!("{}", results.join(" "));
    }
}
