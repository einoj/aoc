use std::io::Read;
use regex::Regex;

fn mul(a: i64, b: i64) -> i64 {
    a*b
}

fn remove_dont_block (chars: &mut Vec<char>) -> bool {
    let mut start_i = 0;
    let mut dont_start = false;
    for (i, c) in chars.iter().enumerate() {
        if *c == 'd' {
            if dont_start {
                let chunk: String = chars[i..i+4].iter().collect();
                if  chunk == "do()" {
                    dont_start = false;
                    chars.drain(start_i..i+4);
                    return true;
                }
            } else {
                let chunk: String = chars[i..i+7].iter().collect();
                if  chunk == "don't()" {
                    start_i = i;
                    dont_start = true;
                }
            }
        }
    }
    return false;
}

fn parse(data: &mut String) {
    let mut chars: Vec<char> = data.chars().collect();
    while remove_dont_block(&mut chars) {

    }
    *data = chars.iter().collect();

}

fn main() {
    let mut stdin = std::io::stdin();
    let mut data = String::new();
    let _ = stdin.read_to_string(&mut data);

    parse(&mut data);
    let re = Regex::new(r"(mul\(\d{1,3},\d{1,3}\))").unwrap();
    let digits = Regex::new(r"(\d{1,3})").unwrap();

    let mut sum = 0;
    for cap in re.find_iter(data.as_str()) {
        let mut digs: Vec<i64> = Vec::new();
        for d in digits.find_iter(cap.as_str()) {
            digs.push(d.as_str().parse::<i64>().unwrap());
        }
        sum += mul(digs[0], digs[1]);
    }
    println!("{:?}", sum);
}
