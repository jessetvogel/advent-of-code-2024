use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter::zip,
};

fn input() -> (Vec<i32>, Vec<i32>) {
    let file = File::open("input/1.txt").unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let parts: Vec<&str> = line.split("   ").collect();
            let x: i32 = parts[0].parse().unwrap();
            let y: i32 = parts[1].parse().unwrap();
            (x, y)
        })
        .unzip()
}

#[allow(dead_code)]
pub fn puzzle_1() -> i32 {
    // Get input
    let (mut left, mut right) = input();

    // Sort both vectors
    left.sort();
    right.sort();

    // Compute sum of absolute differences
    let answer = zip(left, right).map(|(x, y)| (x - y).abs()).sum();

    answer
}

#[allow(dead_code)]
pub fn puzzle_2() -> i32 {
    // Get input
    let (mut left, mut right) = input();

    // Sort both vectors
    left.sort();
    right.sort();

    // Convert to iterators
    let mut left = left.iter();
    let mut right = right.iter();

    // Compute similarity score
    let mut score = 0;
    let mut x = left.next();
    let mut y = right.next();
    loop {
        match x {
            None => break,
            Some(x) => {
                // Let y catch up to x
                while y.is_some_and(|y| y < x) {
                    y = right.next();
                }
                // Count how y's are equal to x
                let mut count = 0;
                while y.is_some_and(|y| y == x) {
                    count += 1;
                    y = right.next();
                }
                // Update score
                score += x * count;
            }
        }
        // Go to next x
        x = left.next();
    }

    score
}
