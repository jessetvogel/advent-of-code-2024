use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

struct Map {
    cells: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct Guard {
    x: i32,
    y: i32,
    dir_x: i32,
    dir_y: i32,
}

impl Map {
    fn advance(&self, guard: &Guard) -> Option<Guard> {
        // Compute the next guard's position
        let next_x = guard.x + guard.dir_x;
        let next_y = guard.y + guard.dir_y;

        // If the guard exits the map, break the loop
        if next_x < 0 || next_x >= self.width as i32 || next_y < 0 || next_y >= self.height as i32 {
            return None;
        }

        // If there is an obstacle in the way, rotate to the right
        if self.cells[next_y as usize][next_x as usize] == '#' {
            return self.advance(&Guard {
                x: guard.x,
                y: guard.y,
                dir_x: -guard.dir_y,
                dir_y: guard.dir_x,
            });
        }

        // Otherwise, move forward
        return Some(Guard {
            x: next_x,
            y: next_y,
            dir_x: guard.dir_x,
            dir_y: guard.dir_y,
        });
    }
}

fn input() -> (Map, Guard) {
    let file = File::open("input/6.txt").unwrap();
    let reader = BufReader::new(file);

    let mut cells: Vec<Vec<char>> = Vec::new();
    let mut guard = Guard {
        x: 0,
        y: 0,
        dir_x: 0, // initially, the guard moves upwards
        dir_y: -1,
    };

    for (y, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if let Some(x) = line.find('^') {
            guard.x = x as i32;
            guard.y = y as i32;
        }
        cells.push(line.chars().collect());
    }

    let height = cells.len();
    let width = cells[0].len();
    let map = Map {
        cells,
        width,
        height,
    };

    (map, guard)
}

#[allow(dead_code)]
pub fn puzzle_1() -> usize {
    // Get input
    let (map, mut guard) = input();

    // Simulate the guard's movement, and keep track of guard's position in a hashset
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    loop {
        // Record current position
        visited.insert((guard.x, guard.y));

        match map.advance(&guard) {
            // If the guard exits the map, break the loop
            None => break,
            // Otherwise, move to the next position
            Some(next) => {
                guard = next;
            }
        }
    }

    visited.len()
}

#[allow(dead_code)]
pub fn puzzle_2() -> usize {
    // Get input
    let (mut map, mut guard) = input();

    // Keep track of the viable obstruction places in a hashset
    let mut obstructions: HashSet<(usize, usize)> = HashSet::new();

    // Every place the guard visits, we try to place an obstacle.
    // If that results in a loop, it is a viable place for an obstacle.
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    loop {
        // Record current position
        visited.insert((guard.x, guard.y));

        match map.advance(&guard) {
            // If the guard exits the map, break the loop
            None => break,
            Some(next) => {
                // If the guard has not yet visited the next position, we may try to place an obstacle there.
                // If the guard then loops, we record the place as viable for an obstacle.
                // Afterwards, remove the obstacle again(!)
                if !visited.contains(&(next.x, next.y)) {
                    map.cells[next.y as usize][next.x as usize] = '#';
                    if loops(&map, guard.clone()) {
                        obstructions.insert((next.x as usize, next.y as usize));
                    }
                    map.cells[next.y as usize][next.x as usize] = '.';
                }
                // Finally, move forward
                guard = next;
            }
        }
    }

    obstructions.len()
}

fn loops(map: &Map, mut guard: Guard) -> bool {
    let mut visited: HashSet<Guard> = HashSet::new();
    loop {
        // Record current position and direction
        // If the guard was here before, she is in a loop
        if !visited.insert(guard.clone()) {
            return true;
        }
        match map.advance(&guard) {
            // If guard walks off the map, she does not loop
            None => return false,
            // Otherwise, move forward
            Some(next) => {
                guard = next;
            }
        }
    }
}
