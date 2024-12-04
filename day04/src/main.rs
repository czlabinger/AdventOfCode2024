use std::fs;
use std::collections::HashSet;

fn main() {
    let content: String = fs::read_to_string("./input2").unwrap();

    println!("{}", find_x_mas(content));
}

fn find_x_mas(input: String) -> u32 {
    // Convert the input string into a 2D grid of characters
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();
    let mut x_mas_count = 0;

    // Check if a position is within grid bounds
    fn is_valid(row: i32, col: i32, rows: usize, cols: usize) -> bool {
        row >= 0 && row < rows as i32 && col >= 0 && col < cols as i32
    }

    // Try to find a MAS pattern in a specific direction
    fn find_mas(grid: &Vec<Vec<char>>, start_row: i32, start_col: i32, dr: i32, dc: i32) -> Option<Vec<(usize, usize)>> {
        let rows = grid.len() as i32;
        let cols = grid[0].len() as i32;
        
        // Try forwards MAS
        let mas_chars = ['M', 'A', 'S'];
        let mut positions = Vec::new();
        let mut r = start_row;
        let mut c = start_col;
        
        // Forwards direction
        for &ch in &mas_chars {
            if !is_valid(r, c, grid.len(), grid[0].len()) { return None; }
            if grid[r as usize][c as usize] != ch { 
                // Reset and try again
                r = start_row;
                c = start_col;
                positions.clear();
                break;
            }
            positions.push((r as usize, c as usize));
            r += dr;
            c += dc;
        }

        if positions.len() == 3 {
            return Some(positions);
        }

        // Try reverse MAS
        let mas_chars_rev = ['S', 'A', 'M'];
        positions.clear();
        r = start_row;
        c = start_col;
        
        for &ch in &mas_chars_rev {
            if !is_valid(r, c, grid.len(), grid[0].len()) { return None; }
            if grid[r as usize][c as usize] != ch { return None; }
            positions.push((r as usize, c as usize));
            r += dr;
            c += dc;
        }

        Some(positions)
    }

    // Search for X-MAS configurations
    for start_row in 0..rows {
        for start_col in 0..cols {
            // Check various X configurations
            let x_configs = [
                // Combinations of directions that could form an X
                [(0, 1), (1, 0)],   // down-right
                [(0, -1), (1, 0)],  // down-left
                [(0, 1), (-1, 0)],  // up-right
                [(0, -1), (-1, 0)], // up-left
                [(1, 1), (-1, -1)], // diagonal
                [(1, -1), (-1, 1)], // opposite diagonal
            ];

            for config in &x_configs {
                let mas1 = find_mas(&grid, 
                    start_row as i32, 
                    start_col as i32, 
                    config[0].0, config[0].1
                );

                let mas2 = find_mas(&grid, 
                    start_row as i32, 
                    start_col as i32, 
                    config[1].0, config[1].1
                );

                // Validate X-MAS configuration
                if let (Some(m1), Some(m2)) = (&mas1, &mas2) {
                    x_mas_count += 1;
                    println!("Found X-MAS at start: ({}, {}), dirs: {:?}", start_row, start_col, config);
                }
            }
        }
    }

    x_mas_count
}
