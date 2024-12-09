use std::fs::read_to_string;

pub fn main() {
    let content = read_to_string("data.txt").expect("No se pudo leer el archivo");

    let grid: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();

    let total = count_xmas_patterns_part2(&grid);
    println!("El patrón X-MAS aparece {} veces en total", total);
}

fn count_xmas_patterns_part2(grid: &Vec<Vec<char>>) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for i in 1..rows - 1 {
        for j in 1..cols - 1 {
            if grid[i][j] == 'A' {
                count += check_x_pattern(grid, i, j, rows, cols);
            }
        }
    }

    count
}

fn check_x_pattern(
    grid: &Vec<Vec<char>>,
    center_i: usize,
    center_j: usize,
    rows: usize,
    cols: usize,
) -> usize {
    let directions = [[(-1, -1), (1, 1)], [(-1, 1), (1, -1)]];

    for dir_pair in directions.iter() {
        let dir1 = dir_pair[0];
        let dir2 = dir_pair[1];

        if is_valid_mas(grid, center_i, center_j, dir1.0, dir1.1, rows, cols)
            && is_valid_mas(grid, center_i, center_j, dir2.0, dir2.1, rows, cols)
        {
            return 1;
        }
    }

    0
}

fn is_valid_mas(
    grid: &Vec<Vec<char>>,
    center_i: usize,
    center_j: usize,
    di: i32,
    dj: i32,
    rows: usize,
    cols: usize,
) -> bool {
    let m_i = (center_i as i32 - di) as usize;
    let m_j = (center_j as i32 - dj) as usize;
    let s_i = (center_i as i32 + di) as usize;
    let s_j = (center_j as i32 + dj) as usize;

    if m_i >= rows || m_j >= cols || s_i >= rows || s_j >= cols {
        return false;
    }

    let forward = grid[m_i][m_j] == 'M' && grid[s_i][s_j] == 'S';
    let backward = grid[m_i][m_j] == 'S' && grid[s_i][s_j] == 'M';

    forward || backward
}
