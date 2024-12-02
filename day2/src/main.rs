use std::fs;

fn main() {
    let content: String = fs::read_to_string("./input").unwrap();

    let mut out: i32 = 0;

    for line in content.lines() {
    
        let mut desc: i32 = 0;
        let mut last = 0;
        let mut valid = true;
        
        for e in line.split_whitespace() {
            let curr: i32 = e.parse::<i32>().unwrap();
      
            println!("last: {}\ncurr: {}", last, curr);

            if last == 0 {
                last = curr.clone();
                continue;
            }

            if last == curr {
                valid = false;
                break;
            }

            if last > curr && desc == 0 {
                desc = 1;
            } else if last < curr && desc == 0 {
                desc = 2;
            } else if last == curr && desc == 0 {
                valid = false;
                break;
            }


            if last > curr && desc == 2 {
                valid = false;
            }

            if last < curr && desc == 1 {
                valid = false;
            }

            if (last - curr).abs() > 3 && (last - curr).abs() > 0 {
                valid = false;
            }

            last = curr.clone();

        }
        if valid {
            println!("Line: {} is {}", line, valid);
            out += 1;
        }
    }
    println!("{}", out);

}
