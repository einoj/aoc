use std::io::{self, BufRead};
fn main() {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(input) =>     {
                let numbers: Vec<i32> = input.trim()
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
                left.push(numbers[0]);
                    right.push(numbers[1]);
            },
            Err(e) => {
                println!("Error reading line: {}", e);
            }
        }
    }

    let mut sum = 0;
    for l in left.iter() {
        let simscore: i32 = right.iter().filter(|&x| x == l).count() as i32;
        let partsum = l*simscore;
        sum += partsum.abs();
    }
    println!("{}", sum);
}
