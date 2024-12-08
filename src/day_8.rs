use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

fn input() -> Vec<Vec<char>> {
    let file = File::open("input/8.txt").unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
}

#[allow(dead_code)]
pub fn puzzle_1() -> usize {
    // Get input
    let map = input();
    let height = map.len() as i32;
    let width = map[0].len() as i32;

    // Keep track of the locations of the antennas and of the antinodes
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let (x, y) = (x as i32, y as i32);

            // If the cell is empty, continue
            if *cell == '.' {
                continue;
            }

            // Create a new list of locations of antennas with this frequency if needed
            if !antennas.contains_key(cell) {
                antennas.insert(*cell, vec![]);
            }

            // For every other antenna with the same frequency, compute the antinodes
            let locations = antennas.get_mut(cell).unwrap();
            for &(u, v) in locations.iter() {
                // Compute and add antinodes (if they are within bounds)
                for (a, b) in [(2 * u - x, 2 * v - y), (2 * x - u, 2 * y - v)] {
                    if a >= 0 && a < width && b >= 0 && b < height {
                        antinodes.insert((a, b));
                    }
                }
            }

            // Finally, store the location of this antenna
            locations.push((x, y));
        }
    }

    antinodes.len()
}

#[allow(dead_code)]
pub fn puzzle_2() -> usize {
    // Get input
    let map = input();
    let height = map.len() as i32;
    let width = map[0].len() as i32;

    // Keep track of the locations of the antennas and of the antinodes
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for (y, row) in map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let (x, y) = (x as i32, y as i32);

            // If the cell is empty, continue
            if *cell == '.' {
                continue;
            }

            // Create a new list of locations of antennas with this frequency if needed
            if !antennas.contains_key(cell) {
                antennas.insert(*cell, vec![]);
            }

            // For every other antenna with the same frequency, compute the antinodes
            let locations = antennas.get_mut(cell).unwrap();
            for &(u, v) in locations.iter() {
                let (dx, dy) = (u - x, v - y);
                let gcd_dx_dy = gcd(dx.abs(), dy.abs());
                let (dx, dy) = (dx / gcd_dx_dy, dy / gcd_dx_dy);
                let t_max = i32::max(width, height); // could be smarter about this
                for t in -t_max..t_max {
                    let (a, b) = (u + t * dx, v + t * dy);
                    if a >= 0 && a < width && b >= 0 && b < height {
                        antinodes.insert((a, b));
                    }
                }
            }

            // Finally, store the location of this antenna
            locations.push((x, y));
        }
    }

    antinodes.len()
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}
