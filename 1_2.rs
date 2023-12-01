use std::io::{self, BufRead};
use std::collections::HashMap;


struct Dig {
   val: u32,
   idx: u32
}

fn part1(line: &str) -> (Dig, Dig) {
    let mut mindig = Dig{val: 0, idx:10000};
    let mut maxdig = Dig{val: 0, idx:0};
    let mut first = false;
    let mut second = false;
    for (i,c) in line.chars().enumerate() {
        if c.is_numeric() {
            if !first {
                mindig.val = c.to_digit(10).unwrap();
                mindig.idx = i as u32;
                first = true;
            } else {
                maxdig.val = c.to_digit(10).unwrap();
                maxdig.idx = i as u32;
                second = true;
            }
        }
    }
    if !second {
        maxdig.val = mindig.val;
        maxdig.idx = mindig.idx;
    }
    return (mindig, maxdig);
}

fn getdigit(p1min: Dig, p1max: Dig, p2min: Dig, p2max: Dig) -> u32 {
    let mut min = p2min.val;
    let mut max = p2max.val;
    if p1min.idx < p2min.idx {
        min = p1min.val;
    }

    if p1max.idx > p2max.idx {
        max = p1max.val;
    }

    if max == 0 {
        max = min;
    }

    let digit = format!("{}{}",min,max);
    return digit.parse::<u32>().unwrap();
}

fn main() {
    let digits: HashMap<&str, u32> = HashMap::from( [("one", 1), ("two",2), ("three",3), ("four",4), ("five",5), ("six",6), ("seven", 7), ("eight",8), ("nine",9),]);
    let stdin = io::stdin();
    let mut sum = 0;
    for line in stdin.lock().lines() {
        let line = line.expect("Couldn't read line");
        let mut mindig = Dig{val: 0, idx:10000};
        let mut maxdig = Dig{val: 0, idx:0};
        for (key,d) in &digits {
            let i = line.find(key);
            if i == None {
                continue;
            }
            let inum = i.unwrap() as u32;
            if inum < mindig.idx {
                mindig.idx = inum;
                mindig.val = *d; 
            }
            let i = line.rfind(key);
            let inum = i.unwrap() as u32;
            if inum > maxdig.idx {
                maxdig.idx = inum;
                maxdig.val = *d;
            }
        }
        println!("{} {}@{} {}@{}", line, mindig.val, mindig.idx, maxdig.val, maxdig.idx);
        let (p1mindig, p1maxdig) = part1(&line);
        println!("{}@{} {}@{}",p1mindig.val, p1mindig.idx, p1maxdig.val, p1maxdig.idx);
        sum += getdigit(p1mindig, p1maxdig, mindig, maxdig);
    }
    println!("sum = {}",sum);
}
