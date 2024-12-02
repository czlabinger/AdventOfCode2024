use std::fs;

fn is_safe(nums: &[i32]) -> bool {
    let mut desc = 0;
    for w in nums.windows(2) {
        let diff = w[1] - w[0];
        if diff.abs() > 3 {
            return false;
        }
        
        if diff == 0 {
            return false;
        }
        
        if desc == 0 {
            desc = if diff > 0 { 1 } else { 2 };
        } else if (diff > 0 && desc == 2) || (diff < 0 && desc == 1) {
            return false;
        }
    }
    true
}

fn main() {
    let content: String = fs::read_to_string("./input").unwrap();
    let mut out: i32 = 0;
    
    for line in content.lines() {
        let nums: Vec<i32> = line.split_whitespace()
            .map(|e| e.parse::<i32>().unwrap())
            .collect();
        
        // Check if original sequence is safe
        if is_safe(&nums) {
            out += 1;
            continue;
        }
        
        // Try removing each level
        for i in 0..nums.len() {
            let modified: Vec<i32> = nums.iter()
                .enumerate()
                .filter(|&(idx, _)| idx != i)
                .map(|(_, &val)| val)
                .collect();
            
            if is_safe(&modified) {
                out += 1;
                break;
            }
        }
    }
    
    println!("{}", out);
}
