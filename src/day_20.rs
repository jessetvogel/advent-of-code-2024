use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

struct Map {
    tiles: Vec<Vec<char>>,
    width: usize,
    height: usize,
    start_x: usize,
    start_y: usize,
}

fn input() -> Map {
    let file = File::open("input/20.txt").unwrap();
    let reader = BufReader::new(file);

    let mut tiles: Vec<Vec<char>> = Vec::new();
    let mut start_x = 0;
    let mut start_y = 0;

    let mut y = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let mut row: Vec<char> = line.trim().chars().collect();
        if let Some(x) = row.iter().position(|&c| c == 'S') {
            start_x = x;
            start_y = y;
            row[x] = '.';
        }
        if let Some(x) = row.iter().position(|&c| c == 'E') {
            // We do not need to store the end position
            row[x] = '.';
        }
        tiles.push(row);
        y += 1;
    }

    let height = tiles.len();
    let width = tiles[0].len();

    Map {
        tiles,
        width,
        height,
        start_x,
        start_y,
    }
}

#[allow(dead_code)]
pub fn puzzle_1() -> usize {
    // Get input
    let map = input();

    // Compute the distance of all the tiles from the start
    let mut distances = HashMap::new();
    compute_distances(&map, map.start_x, map.start_y, &mut distances);

    // Count the number of cheats that save at least 100 picoseconds
    // We can count a cheat by looking at the difference in score between
    // two tiles which two steps apart.
    let time_to_save = 100;
    let mut num_cheats = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            // Get distance of current tile
            let distance = match distances.get(&(x, y)) {
                None => continue,
                Some(&x) => x,
            };

            // List tiles two steps away
            let mut uv = Vec::new();
            if x > 1 {
                uv.push((x - 2, y));
            }
            if x < map.width - 2 {
                uv.push((x + 2, y));
            }
            if y > 1 {
                uv.push((x, y - 2));
            }
            if y < map.height - 2 {
                uv.push((x, y + 2));
            }

            // If the difference in distance is larger than the threshold
            // this cheat saves enough time!
            for (u, v) in uv {
                if let Some(&s) = distances.get(&(u, v)) {
                    if s + 2 + time_to_save <= distance {
                        num_cheats += 1;
                    }
                }
            }
        }
    }

    num_cheats
}

fn compute_distances(
    map: &Map,
    x: usize,
    y: usize,
    distances: &mut HashMap<(usize, usize), usize>,
) {
    let mut x = x;
    let mut y = y;
    let mut distance = 0;

    loop {
        distances.insert((x, y), distance);
        distance += 1;

        if x > 0 && map.tiles[y][x - 1] == '.' && !distances.contains_key(&(x - 1, y)) {
            x -= 1;
            continue;
        }

        if x < map.width - 1 && map.tiles[y][x + 1] == '.' && !distances.contains_key(&(x + 1, y)) {
            x += 1;
            continue;
        }

        if y > 0 && map.tiles[y - 1][x] == '.' && !distances.contains_key(&(x, y - 1)) {
            y -= 1;
            continue;
        }

        if y < map.height - 1 && map.tiles[y + 1][x] == '.' && !distances.contains_key(&(x, y + 1))
        {
            y += 1;
            continue;
        }

        break;
    }
}

#[allow(dead_code)]
pub fn puzzle_2() -> usize {
    // Get input
    let map = input();

    // Compute the distances of all the tiles
    let mut distances = HashMap::new();
    compute_distances(&map, map.start_x, map.start_y, &mut distances);

    // Count the number of cheats that save at least 100 picoseconds
    // We can count a cheat by looking at the difference in score between
    // two tiles which two steps apart.
    let time_to_save = 100;
    let max_cheat_length = 20i32;
    let mut num_cheats = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            // Get score of current tile
            let distance_xy = match distances.get(&(x, y)) {
                None => continue,
                Some(&x) => x,
            };

            // Look for all possible endpoints of the cheat within a range of 20 steps
            for dx in -max_cheat_length..=max_cheat_length {
                for dy in -max_cheat_length..=max_cheat_length {
                    let cheat_length = dx.abs() + dy.abs();
                    if cheat_length > max_cheat_length {
                        continue;
                    }
                    let u = (x as i32) + dx;
                    let v = (y as i32) + dy;
                    if u < 0 || u >= map.width as i32 || v < 0 || v >= map.height as i32 {
                        continue;
                    }
                    let distance_uv = match distances.get(&(u as usize, v as usize)) {
                        None => continue,
                        Some(&x) => x,
                    };
                    if distance_uv + (cheat_length as usize) + time_to_save <= distance_xy {
                        num_cheats += 1;
                    }
                }
            }
        }
    }

    num_cheats
}
