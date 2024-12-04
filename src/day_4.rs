use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn input() -> Vec<Vec<char>> {
    let file = File::open("input/4.txt").unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
}

#[allow(dead_code)]
pub fn puzzle_1() -> usize {
    // Get input
    let input = input();

    // Count all instances of `XMAS`
    let height = input.len();
    let width = input[0].len();
    let mut count: usize = 0;

    // Start from position (x, y) and go in any of the 8 directions
    for x in 0..width {
        for y in 0..height {
            for (dx, dy) in [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ] {
                // Check for bounds
                if (dx < 0 && (x as i32) + 3 * dx < 0)
                    || (dx > 0 && (x as i32) + 3 * dx >= width as i32)
                    || (dy < 0 && (y as i32) + 3 * dy < 0)
                    || (dy > 0 && (y as i32) + 3 * dy >= height as i32)
                {
                    continue;
                }
                // Check for `XMAS`
                if input[y][x] == 'X'
                    && input[(y as i32 + dy) as usize][(x as i32 + dx) as usize] == 'M'
                    && input[(y as i32 + 2 * dy) as usize][(x as i32 + 2 * dx) as usize] == 'A'
                    && input[(y as i32 + 3 * dy) as usize][(x as i32 + 3 * dx) as usize] == 'S'
                {
                    count += 1;
                }
            }
        }
    }

    count
}

#[allow(dead_code)]
pub fn puzzle_2() -> usize {
    // Get input
    let input = input();

    // Count all instances of `XMAS`
    let height = input.len();
    let width = input[0].len();
    let mut count: usize = 0;

    // Loop over all X-shaped patterns
    for x in 1..width - 1 {
        for y in 1..height - 1 {
            // Middle char should be 'A'
            if input[y][x] != 'A' {
                continue;
            }
            // Check for combinations `MAS` or `SAM` on the diagonals
            if ((input[y - 1][x - 1] == 'M' && input[y + 1][x + 1] == 'S')
                || (input[y - 1][x - 1] == 'S' && input[y + 1][x + 1] == 'M'))
                && ((input[y - 1][x + 1] == 'M' && input[y + 1][x - 1] == 'S')
                    || (input[y - 1][x + 1] == 'S' && input[y + 1][x - 1] == 'M'))
            {
                count += 1;
            }
        }
    }

    count
}
