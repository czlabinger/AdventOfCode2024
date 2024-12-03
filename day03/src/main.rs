use std::fs;
use regex::Regex;

fn main() {
    let content: String = fs::read_to_string("./input").unwrap();
    let regex: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap(); 
    let do_regex = Regex::new(r"do\(\)").unwrap();
    let dont_regex = Regex::new(r"don't\(\)").unwrap();
    
    let mut mul_enabled = true;
    let mut out: u32 = 0;

     let binding = Regex::new(r"(mul\(\d+,\d+\)|do\(\)|don't\(\))").unwrap();
     let tokens = binding
        .find_iter(&content)
        .map(|m| m.as_str());
    
    for token in tokens {
        if do_regex.is_match(token) {
            // Enable mul instructions
            mul_enabled = true;
        } else if dont_regex.is_match(token) {
            // Disable mul instructions
            mul_enabled = false;
        } else if let Some(captures) = regex.captures(token) {
            // Process mul instruction if enabled
            if mul_enabled {
                let x: u32 = captures[1].parse().unwrap();
                let y: u32 = captures[2].parse().unwrap();
                out += x * y;
                println!("Processing: {} * {} = {}", x, y, x * y);
            } else {
                println!("Skipping disabled mul: {}", token);
            }
        }
    }
    println!("{}", out);
}
