use std::fs;

fn main() {
    let content: String = fs::read_to_string("./input").unwrap();

    println!("{}", find_x_mas(&content));
}

fn find_x_mas(grid: &str) -> usize {
    
    let lines: Vec<Vec<char>> = grid.lines().map(|line| line.chars().collect()).collect();
    let rows = lines.len();
    let cols = lines[0].len();
    let mut count = 0;

    for i in 1..rows - 1 {
        for j in 1..cols - 1 {
    
            let center = lines[i][j] == 'A';

            let backward = center
                && lines[i - 1][j - 1] == 'M'
                && lines[i - 1][j + 1] == 'S'
                && lines[i + 1][j - 1] == 'M'
                && lines[i + 1][j + 1] == 'S';

            let forward = center
                && lines[i - 1][j - 1] == 'S'
                && lines[i - 1][j + 1] == 'M'
                && lines[i + 1][j - 1] == 'S'
                && lines[i + 1][j + 1] == 'M';

            let mixed1 = center
                && lines[i - 1][j - 1] == 'M'
                && lines[i - 1][j + 1] == 'M'
                && lines[i + 1][j - 1] == 'S'
                && lines[i + 1][j + 1] == 'S';

            let mixed2 = center
                && lines[i - 1][j - 1] == 'S'
                && lines[i - 1][j + 1] == 'S'
                && lines[i + 1][j - 1] == 'M'
                && lines[i + 1][j + 1] == 'M';

            if forward || backward || mixed1 || mixed2 {
                count += 1;
            }
        }
    }
    count
}
