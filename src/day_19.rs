use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn input() -> (Vec<String>, Vec<String>) {
    let file = File::open("input/19.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    let towels: Vec<String> = line.split(", ").map(|s| s.trim().into()).collect();

    reader.read_line(&mut line).unwrap(); // skip empty line

    let patterns = reader
        .lines()
        .map(|line| line.unwrap().trim().into())
        .collect();

    (towels, patterns)
}

#[allow(dead_code)]
pub fn puzzle_1() -> usize {
    // Get input
    let (towels, patterns) = input();

    // Memoize which patterns can and which cannot be made
    let mut memory = HashMap::new();

    // Count how many patterns can be made with the towels
    patterns
        .iter()
        .filter(|pattern| is_possible(pattern, &towels, &mut memory))
        .count()
}

fn is_possible(pattern: &str, towels: &Vec<String>, memory: &mut HashMap<String, bool>) -> bool {
    // Base case: we can always make patterns of length zero
    if pattern.len() == 0 {
        return true;
    }

    // If this question was asked before, return the same answer
    if let Some(&b) = memory.get(pattern) {
        return b;
    }

    // For every towel, check if the pattern starts with that towels pattern,
    // if so, continue recursively. If there are no matches, return false
    for towel in towels {
        if pattern.starts_with(towel) && is_possible(&pattern[towel.len()..], towels, memory) {
            memory.insert(pattern.into(), true);
            return true;
        }
    }
    memory.insert(pattern.into(), false);
    false
}

#[allow(dead_code)]
pub fn puzzle_2() -> usize {
    // Get input
    let (towels, patterns) = input();

    // Memoize the number of ways each pattern can be made
    let mut memory = HashMap::new();

    // Count the number of ways the patterns can be made with the towels
    patterns
        .iter()
        .map(|pattern| count_possibilities(pattern, &towels, &mut memory))
        .sum()
}

fn count_possibilities(
    pattern: &str,
    towels: &Vec<String>,
    memory: &mut HashMap<String, usize>,
) -> usize {
    // Base case: there is always exactly one way to make a pattern of length 0
    if pattern.len() == 0 {
        return 1;
    }

    // If this question was asked before, return the same answer
    if let Some(&n) = memory.get(pattern) {
        return n;
    }

    // For every towel, check if the pattern starts with that towels pattern,
    // if so, count the number of ways to make each subpattern
    let mut n = 0;
    for towel in towels {
        if pattern.starts_with(towel) {
            n += count_possibilities(&pattern[towel.len()..], towels, memory);
        }
    }
    memory.insert(pattern.into(), n);
    n
}
