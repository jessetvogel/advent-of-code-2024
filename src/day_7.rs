use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn input() -> Vec<(u64, Vec<u64>)> {
    let file = File::open("input/7.txt").unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let parts: Vec<&str> = line.split(":").collect();
            let total = parts[0].parse().unwrap();
            let numbers: Vec<u64> = parts[1]
                .trim()
                .split(" ")
                .map(|x| x.parse().unwrap())
                .collect();
            (total, numbers)
        })
        .collect()
}

#[allow(dead_code)]
pub fn puzzle_1() -> u64 {
    // Get input
    let equations = input();

    // Count the number of possibly true equations
    equations
        .iter()
        .filter(|(total, numbers)| {
            let mut numbers = numbers.clone();
            is_possibly_true(*total, &mut numbers, false)
        })
        .map(|(total, _)| total)
        .sum()
}

fn concatenate(a: u64, b: u64) -> u64 {
    if b == 0 {
        // edge case
        return a * 10;
    }

    let mut copy_a = a;
    let mut copy_b = b;
    while copy_b > 0 {
        copy_a *= 10;
        copy_b /= 10;
    }

    copy_a + b
}

fn is_possibly_true(total: u64, numbers: &mut [u64], allow_concat: bool) -> bool {
    // Base case: if there is only one number, the total must be equal to the number
    if numbers.len() == 1 {
        return total == numbers[0];
    }

    // Try to add the first two numbers, and repeat recursively
    numbers[1] += numbers[0];
    if is_possibly_true(total, &mut numbers[1..], allow_concat) {
        return true;
    }
    numbers[1] -= numbers[0];

    // Try to multiply the first two numbers, and repeat recursively
    numbers[1] *= numbers[0];
    if is_possibly_true(total, &mut numbers[1..], allow_concat) {
        return true;
    }
    numbers[1] /= numbers[0];

    if !allow_concat {
        return false;
    }

    // Try to concatenate the first two numbers, and repeat recursively
    let old = numbers[1];
    numbers[1] = concatenate(numbers[0], numbers[1]);
    if is_possibly_true(total, &mut numbers[1..], allow_concat) {
        return true;
    }
    numbers[1] = old;

    false
}

#[allow(dead_code)]
pub fn puzzle_2() -> u64 {
    // Get input
    let equations = input();

    // Count the number of possibly true equations
    equations
        .iter()
        .filter(|(total, numbers)| {
            let mut numbers = numbers.clone();
            is_possibly_true(*total, &mut numbers, true)
        })
        .map(|(total, _)| total)
        .sum()
}
