use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

struct Map {
    heights: Vec<Vec<u32>>,
    width: usize,
    height: usize,
}

fn input() -> Map {
    let file = File::open("input/10.txt").unwrap();
    let reader = BufReader::new(file);

    let heights: Vec<Vec<u32>> = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10))
                .filter(|x| x.is_some())
                .map(|x| x.unwrap())
                .collect()
        })
        .collect();

    let height = heights.len();
    let width = heights[0].len();

    Map {
        heights,
        width,
        height,
    }
}

#[allow(dead_code)]
pub fn puzzle_1() -> usize {
    // Get input
    let map = input();

    // Compute sum of the scores of all trailheads
    // For each starting position (x, y), keep track of the possible endpoints of hikes in a hashset
    // For each starting position, the score is the size of this hashset
    let mut total_score = 0;
    let mut endpoints = HashSet::new();
    for x in 0..map.width {
        for y in 0..map.height {
            endpoints.clear();
            search_for_endpoints(&map, x, y, 0, &mut endpoints);
            total_score += endpoints.len();
        }
    }

    total_score
}

fn search_for_endpoints(
    map: &Map,
    x: usize,
    y: usize,
    z: u32,
    endpoints: &mut HashSet<(usize, usize)>,
) {
    if map.heights[y][x] != z {
        return;
    }
    // If we found an endpoint, store it
    if z == 9 {
        endpoints.insert((x, y));
        return;
    }
    // Search for endpoints left, right, down or up
    if x > 0 {
        search_for_endpoints(&map, x - 1, y, z + 1, endpoints);
    }
    if x < map.width - 1 {
        search_for_endpoints(&map, x + 1, y, z + 1, endpoints);
    }
    if y > 0 {
        search_for_endpoints(&map, x, y - 1, z + 1, endpoints);
    }
    if y < map.height - 1 {
        search_for_endpoints(&map, x, y + 1, z + 1, endpoints);
    }
}

#[allow(dead_code)]
pub fn puzzle_2() -> usize {
    // Get input
    let map = input();

    // Compute sum of the ratings of all trailheads
    // For each starting position (the 0's), its rating is the sum of the ratings of the surrounding 1's,
    // whose ratings are the sum of the ratings of the surrounding 2's,
    // whose ratings are the sum of the ratings of the surrounding 3's, ...
    // Try to be smart by memorizing the ratings of already visited places on the map
    let mut total_rating = 0;
    let mut memory = HashMap::new();
    for x in 0..map.width {
        for y in 0..map.height {
            total_rating += rating(&map, x, y, 0, &mut memory);
        }
    }

    total_rating
}

fn rating(
    map: &Map,
    x: usize,
    y: usize,
    z: u32,
    memory: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if map.heights[y][x] != z {
        return 0;
    }
    // Every endpoint has a rating of 1
    if z == 9 {
        return 1;
    }
    // Check if the rating was already computed
    if memory.contains_key(&(x, y)) {
        return *memory.get(&(x, y)).unwrap();
    }
    // Compute rating from the ratings left, right, down and up
    let mut r = 0;
    if x > 0 {
        r += rating(&map, x - 1, y, z + 1, memory);
    }
    if x < map.width - 1 {
        r += rating(&map, x + 1, y, z + 1, memory);
    }
    if y > 0 {
        r += rating(&map, x, y - 1, z + 1, memory);
    }
    if y < map.height - 1 {
        r += rating(&map, x, y + 1, z + 1, memory);
    }
    // Memorize the rating
    memory.insert((x, y), r);

    r
}
