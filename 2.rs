use std::io::{self, BufRead};
use std::collections::HashMap;

fn isvalid(line: &str) -> bool {
    let beads: HashMap<&str, u32> = HashMap::from( [("red", 12), ("green", 13), ("blue", 14)]);

    let draws = line.split("; ");
    for draw in draws {
        let colors = draw.split(", "); 
        for color in colors {
            let mut valcol = color.split(" ");
            let num = valcol.next().unwrap().parse::<u32>().unwrap();
            let color = valcol.next().unwrap();
            if beads[color] < num {
                return false;
            }
        }
    }
    return true;
}

fn minprod(line: &str) -> u32 {
    let mut beads: HashMap<&str, u32> = HashMap::from( [("red", 0), ("green", 0), ("blue", 0)]);

    let draws = line.split("; ");
    for draw in draws {
        let colors = draw.split(", "); 
        for color in colors {
            let mut valcol = color.split(" ");
            let num = valcol.next().unwrap().parse::<u32>().unwrap();
            let color = valcol.next().unwrap();
            if beads[color] < num {
                beads.insert(color, num);
            }
        }
    }
    return beads["red"]*beads["green"]*beads["blue"];
}

fn main() {
    let stdin = io::stdin();
    let mut sum = 0;
    let mut prod = 0;
    for line in stdin.lock().lines() {
        let line = line.expect("Couldn't read line");
        let mut parts = line.split(": ");
        let s = parts.next().unwrap();
        let game = &s[5..].parse::<i32>().unwrap();
        let draws = parts.next().unwrap();
        if isvalid(draws) {
            sum += game;
        }
        prod += minprod(draws);
    }
    println!("sum = {}",sum);
    println!("product sum = {}", prod);
}
