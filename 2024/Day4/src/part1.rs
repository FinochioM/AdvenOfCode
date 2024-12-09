use std::fs::read_to_string;

pub fn main() {
    let content = read_to_string("data.txt").expect("No se pudo leer el archivo");

    let grid: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();

    let total = count_xmas_patterns(&grid);
    println!("Total: {}", total);
}

fn count_xmas_patterns(grid: &Vec<Vec<char>>) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    let directions = [
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];

    for i in 0..rows {
        for j in 0..cols {
            for &(di, dj) in &directions {
                count += check_pattern(grid, i, j, di, dj, rows, cols);
            }
        }
    }

    count
}

fn check_pattern(
    grid: &Vec<Vec<char>>,
    start_i: usize,
    start_j: usize,
    di: i32,
    dj: i32,
    rows: usize,
    cols: usize,
) -> usize {
    if grid[start_i][start_j] != 'X' {
        return 0;
    }

    let positions = [(0, 0), (1, 1), (2, 2), (3, 3)];
    let pattern = ['X', 'M', 'A', 'S'];

    for (idx, &(step_i, step_j)) in positions.iter().enumerate() {
        let new_i = start_i as i32 + di * step_i;
        let new_j = start_j as i32 + dj * step_j;

        if new_i < 0 || new_i >= rows as i32 || new_j < 0 || new_j >= cols as i32 {
            return 0;
        }

        if grid[new_i as usize][new_j as usize] != pattern[idx] {
            return 0;
        }
    }

    1
}
