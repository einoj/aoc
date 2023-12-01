use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut sum = 0;
    for line in stdin.lock().lines() {
        let line = line.expect("Couldn't read line");
        let mut first = false;
        let mut second = false;
        let mut x = 0;
        let mut y = 0;
        for c in line.chars() {
            if c.is_numeric() {
                if !first {
                    x = c.to_digit(10).unwrap();
                    first = true;
                } else {
                    y = c.to_digit(10).unwrap();
                    second = true;
                }
            }
        }
        if !second {
            let digit = format!("{}{}",x,x);
            sum += digit.parse::<i32>().unwrap();
        } else {
            let digit = format!("{}{}",x,y);
            sum += digit.parse::<i32>().unwrap();
        }
    }
    println!("sum = {}",sum);
}
