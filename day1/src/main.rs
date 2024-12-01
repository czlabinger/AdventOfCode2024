use std::fs;
use std::collections::HashMap;

fn main() {
    let path = "./input";
    let content: String = fs::read_to_string(path).expect(format!("Could not read file {}", path).as_str());
    //println!("{}", content);

    let mut first: Vec<String> = Vec::new();
    let mut second: Vec<String> = Vec::new();

    for line in content.lines() {
        let e: Vec<&str> = line.split("   ").collect();
        first.push(String::from(e[0 as usize]));
        second.push(String::from(e[1 as usize]));
    }

    first.sort();
    second.sort();

    let mut out: i32 = 0;
    let mut list: HashMap<i32, i32> = HashMap::new();

    for i in 0..first.len() {
        /*  First part
        let o = (first[i].parse::<i32>().unwrap() - second[i].parse::<i32>().unwrap()).abs();
        out += o;
        */
        let s: i32 = second[i].parse::<i32>().unwrap();

        //  Second part
        if !list.contains_key(&s) && first.contains(&s.to_string()) {
            list.insert(s, 1);
        } else if list.contains_key(&s) {
            list.insert(s, list.get(&s).unwrap() + 1);
        }
    }

    for e in list.iter() {
        out += e.0 * e.1;
    }

    println!("{}", out);
}
