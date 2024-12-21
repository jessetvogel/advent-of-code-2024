use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn input() -> Vec<String> {
    let file = File::open("input/21.txt").unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            line.trim().into()
        })
        .collect()
}

#[allow(dead_code)]
pub fn puzzle_1() -> usize {
    // Get input
    let sequences = input();

    // Map sequences to their complexity and add those
    // Computing the length of the shortest sequence of
    // button presses is done recursively, with memoizations
    // Note that depth = number of robots = 3
    let mut memory: HashMap<(String, usize), usize> = HashMap::new();
    sequences
        .into_iter()
        .map(|seq| {
            let numeric = &seq[0..seq.len() - 1].parse::<usize>().unwrap();
            let min_len = shortest_length(seq, 3, &mut memory, true);
            let complexity = min_len * numeric;
            complexity
        })
        .sum()
}

#[allow(dead_code)]
pub fn puzzle_2() -> usize {
    // Get input
    let sequences = input();

    // Map sequences to their complexity and add those
    // Computing the length of the shortest sequence of
    // button presses is done recursively, with memoizations
    // Note that depth = number of robots = 26
    let mut memory: HashMap<(String, usize), usize> = HashMap::new();
    sequences
        .into_iter()
        .map(|seq| {
            let numeric = &seq[0..seq.len() - 1].parse::<usize>().unwrap();
            let min_len = shortest_length(seq, 26, &mut memory, true);
            let complexity = min_len * numeric;
            complexity
        })
        .sum()
}

fn shortest_length(
    sequence: String,
    depth: usize,
    memory: &mut HashMap<(String, usize), usize>,
    numeric: bool,
) -> usize {
    // Base case: at depth zero, the shortest length is just the length of the pattern
    if depth == 0 {
        return sequence.len();
    }

    // Check if answer lies in memory
    let key = (sequence.clone(), depth);
    if let Some(&length) = memory.get(&key) {
        return length;
    }

    // Compute the minimal length recursively: for every movement of some button to another,
    // find all possible ways to do this, compute the minimal length for that pattern (with one less depth)
    // and take the minimum of that. Sum those minima for every movement of buttons.
    let mut length = 0;
    let mut from = 'A'; // robots always start and end at 'A'
    for to in sequence.chars() {
        let subsequences = if numeric {
            subsequences_num(from, to)
        } else {
            subsequences_arrow(from, to)
        };
        length += subsequences
            .into_iter()
            .map(|subseq| shortest_length(subseq, depth - 1, memory, false))
            .min()
            .unwrap();
        from = to;
    }

    // Update memory
    memory.insert(key, length);

    length
}

fn coord_num(c: char) -> (i32, i32) {
    match c {
        '0' => (1, 3),
        '1' => (0, 2),
        '2' => (1, 2),
        '3' => (2, 2),
        '4' => (0, 1),
        '5' => (1, 1),
        '6' => (2, 1),
        '7' => (0, 0),
        '8' => (1, 0),
        '9' => (2, 0),
        'A' => (2, 3),
        _ => panic!("invalid numpad key"),
    }
}

fn coord_arrow(c: char) -> (i32, i32) {
    match c {
        '<' => (0, 1),
        '>' => (2, 1),
        '^' => (1, 0),
        'v' => (1, 1),
        'A' => (2, 0),
        _ => panic!("invalid dirpad key"),
    }
}

fn subsequences_num(from: char, to: char) -> Vec<String> {
    let coord_from = coord_num(from);
    let coord_to = coord_num(to);

    let dx = coord_to.0 - coord_from.0;
    let dy = coord_to.1 - coord_from.1;

    let cx = if dx > 0 { '>' } else { '<' };
    let cy = if dy > 0 { 'v' } else { '^' };

    let nx = dx.abs() as usize;
    let ny = dy.abs() as usize;

    // Edge cases: do not go over the gap!
    if coord_from.0 == 0 && coord_to.1 == 3 {
        let mut v = vec![cx; nx];
        v.extend(vec![cy; ny]);
        v.push('A');
        return vec![v.iter().collect()];
    }

    if coord_from.1 == 3 && coord_to.0 == 0 {
        let mut v = vec![cy; ny];
        v.extend(vec![cx; nx]);
        v.push('A');
        return vec![v.iter().collect()];
    }

    return combinations(cx, nx, cy, ny);
}

fn subsequences_arrow(from: char, to: char) -> Vec<String> {
    let coord_from = coord_arrow(from);
    let coord_to: (i32, i32) = coord_arrow(to);

    let dx = coord_to.0 - coord_from.0;
    let dy = coord_to.1 - coord_from.1;

    let cx = if dx > 0 { '>' } else { '<' };
    let cy = if dy > 0 { 'v' } else { '^' };

    let nx = dx.abs() as usize;
    let ny = dy.abs() as usize;

    // Edge cases: do not go over the gap!
    if coord_from.0 == 0 && coord_to.1 == 0 {
        let mut v = vec![cx; nx];
        v.extend(vec![cy; ny]);
        v.push('A');
        return vec![v.iter().collect()];
    }

    if coord_from.1 == 0 && coord_to.0 == 0 {
        let mut v = vec![cy; ny];
        v.extend(vec![cx; nx]);
        v.push('A');
        return vec![v.iter().collect()];
    }

    return combinations(cx, nx, cy, ny);
}

fn combinations(cx: char, nx: usize, cy: char, ny: usize) -> Vec<String> {
    if ny == 0 {
        let mut v = vec![cx; nx];
        v.push('A');
        return vec![v.iter().collect()];
    }
    if nx == 0 {
        let mut v = vec![cy; ny];
        v.push('A');
        return vec![v.iter().collect()];
    }
    let mut v1 = vec![cx; nx];
    v1.extend(vec![cy; ny]);
    v1.push('A');
    let mut v2 = vec![cy; ny];
    v2.extend(vec![cx; nx]);
    v2.push('A');
    vec![v1.iter().collect(), v2.iter().collect()]
}
