use std::fs;
use regex::Regex;

fn main() {
    let content: String = fs::read_to_string("./input").unwrap();
    let regex: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap(); 
    let mut out: u32 = 0;

    for line in content.lines() {
        for cap in regex.captures_iter(line) {
            out += cap[1].parse::<u32>().unwrap() * cap[2].parse::<u32>().unwrap();
        }
    }

    println!("{}", out);
}
