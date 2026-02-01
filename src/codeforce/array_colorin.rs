/* https://codeforces.com/contest/2191/problem/A */

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().and_then(|l| l.ok()).unwrap_or_default();
    let t: usize = first_line.trim().parse().unwrap_or(0);

    for _ in 0..t {
        let n = lines.next().and_then(|l| l.ok()).unwrap_or_default();
        let n = n.parse::<u32>().unwrap();

        let arr_str = lines.next().and_then(|l| l.ok()).unwrap_or_default();

        let arr: Vec<usize> = arr_str
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let mut color_vec: Vec<(&usize, &str)> = Vec::new();
        for (i, elm) in arr.iter().enumerate() {
            if i == 0 || color_vec[i - 1].1 == "blue" {
                color_vec.push((elm, "red"));
            } else {
                color_vec.push((elm, "blue"));
            }
        }

        let mut sorted_arr = color_vec.clone();
        sorted_arr.sort_by_key(|k| k.0);
        let mut res = "YES";

        for i in 1..n {
            if sorted_arr[i as usize].1 == sorted_arr[(i - 1) as usize].1 {
                res = "NO";
                break;
            }
        }
        println!("{}", res);
    }
}

// first line the  contains number of test cases
// first line of each test cases contains a single interger n
// all element of a is distance
// print yes if the ragene of colors is be
//

/*
* 4
2 3 4 1

red blue red blue
1 2 3 4
blue red  blue red
so true



3
2 3 1

red blue red
1 2 3
red red blue
fasle


5
3 4 1 2 5

red blue red blue red

1 2 3 4 5
red blue red  blue red
so yes


5
3 1 4 2 5
blue red blue red blue

1 2 3 4 5
red red blue blue blue

*/
