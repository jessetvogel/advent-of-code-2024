use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

struct Map {
    tiles: Vec<Vec<char>>,
    start_x: i32,
    start_y: i32,
    end_x: i32,
    end_y: i32,
}

fn input() -> Map {
    let file = File::open("input/16.txt").unwrap();
    let reader = BufReader::new(file);

    let mut tiles: Vec<Vec<char>> = Vec::new();
    let mut start_x = 0;
    let mut start_y = 0;
    let mut end_x = 0;
    let mut end_y = 0;

    let mut y = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let mut row: Vec<char> = line.trim().chars().collect();
        if let Some(x) = row.iter().position(|&c| c == 'S') {
            start_x = x as i32;
            start_y = y as i32;
            row[x] = '.';
        }
        if let Some(x) = row.iter().position(|&c| c == 'E') {
            end_x = x as i32;
            end_y = y as i32;
            row[x] = '.';
        }
        tiles.push(row);
        y += 1;
    }

    Map {
        tiles,
        start_x,
        start_y,
        end_x,
        end_y,
    }
}

#[allow(dead_code)]
pub fn puzzle_1() -> i32 {
    // Get input
    let map = input();

    // Find the lowest score a Reindeer could possibly get
    let mut scores = HashMap::new();
    find_lowest_score(&map, &mut scores)
}

fn find_lowest_score(map: &Map, scores: &mut HashMap<(i32, i32, i32, i32), i32>) -> i32 {
    // In a queue, keep track of the tiles to look at
    let mut queue: VecDeque<(i32, i32, i32, i32, i32)> = VecDeque::new();

    // Initially, we should look at the starting tile (looking east)
    queue.push_back((map.start_x, map.start_y, 1, 0, 0));

    // While the queue is non-empty, consider the tile with the lowest current score from the queue
    while !queue.is_empty() {
        queue
            .make_contiguous()
            .sort_by(|(_, _, _, _, score_a), (_, _, _, _, score_b)| score_a.cmp(score_b));
        let (x, y, dx, dy, score) = queue.pop_front().unwrap();
        find_score(&map, x, y, dx, dy, score, scores, &mut queue);
    }

    // Finally, look up the score at the end tile (the lowest in any direction)
    *[(1, 0), (-1, 0), (0, 1), (0, -1)]
        .map(|(dx, dy)| *scores.get(&(map.end_x, map.end_y, dx, dy)).unwrap())
        .iter()
        .min()
        .unwrap()
}

fn find_score(
    map: &Map,
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
    score: i32,
    scores: &mut HashMap<(i32, i32, i32, i32), i32>,
    queue: &mut VecDeque<(i32, i32, i32, i32, i32)>,
) {
    let tile = map.tiles[y as usize][x as usize];
    if tile == '#' {
        return;
    }

    // If the current score is already at least as good as the new score, nothing to do
    if scores.get(&(x, y, dx, dy)).is_some_and(|&s| s <= score) {
        return;
    }

    // Insert or update new score
    scores.insert((x, y, dx, dy), score);

    // Add new positions to consider to the queue
    queue.push_back((x + dx, y + dy, dx, dy, score + 1));
    queue.push_back((x, y, dy, -dx, score + 1000));
    queue.push_back((x, y, -dy, dx, score + 1000));
}

#[allow(dead_code)]
pub fn puzzle_2() -> usize {
    // Get input
    let map = input();

    // Find the lowest score a Reindeer could possibly get
    let mut scores = HashMap::new();
    let score = find_lowest_score(&map, &mut scores);

    // Starting from the end tile, find all optimal tiles
    let mut optimal = HashSet::new();
    find_optimal_tiles(map.end_x, map.end_y, 0, -1, score, &scores, &mut optimal);

    optimal.len()
}

fn find_optimal_tiles(
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
    score: i32,
    scores: &HashMap<(i32, i32, i32, i32), i32>,
    optimal: &mut HashSet<(i32, i32)>,
) {
    // Check if the given score is optimal
    let optimal_score = scores.get(&(x, y, dx, dy));
    if optimal_score.is_none_or(|&s| s != score) {
        return;
    }

    // If so, this tile is optimal
    optimal.insert((x, y));

    // Also check optimality for the tile walking backwards ..
    find_optimal_tiles(x - dx, y - dy, dx, dy, score - 1, scores, optimal);
    // .. and for the rotated positions
    find_optimal_tiles(x, y, -dy, dx, score - 1000, scores, optimal);
    find_optimal_tiles(x, y, dy, -dx, score - 1000, scores, optimal);
}
