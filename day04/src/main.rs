use std::fs;

fn main() {
    let content: String = fs::read_to_string("./input").unwrap();

    println!("{}", find_xmas(content));
}

fn find_xmas(input: String) -> u32 {
    
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();
    let word = "XMAS";
    let mut out: u32 = 0;

    let directions = [
        (0, 1),   // right
        (0, -1),  // left
        (1, 0),   // down
        (-1, 0),  // up
        (1, 1),   // down-right
        (1, -1),  // down-left
        (-1, 1),  // up-right
        (-1, -1), // up-left
    ];

    fn is_valid(row: i32, col: i32, rows: usize, cols: usize) -> bool {
        row >= 0 && row < rows as i32 && col >= 0 && col < cols as i32
    }

    for start_row in 0..rows {
        for start_col in 0..cols {
            for &(dr, dc) in &directions {
                let mut current_word = String::new();
                let mut positions = Vec::new();
                let mut r = start_row as i32;
                let mut c = start_col as i32;

                for _ in 0..4 {
                    if !is_valid(r, c, rows, cols) {
                        break;
                    }
                    current_word.push(grid[r as usize][c as usize]);
                    positions.push((r as usize, c as usize));
                    r += dr;
                    c += dc;
                }

                if current_word == word {
                    out += 1;
                }
            }
        }
    }
    out
}
