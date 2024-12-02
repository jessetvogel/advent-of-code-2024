use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn input() -> Vec<Vec<i32>> {
    let file = File::open("input/2.txt").unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            line.unwrap()
                .split(" ")
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect()
}

#[allow(dead_code)]
pub fn puzzle_1() -> usize {
    // Get input
    let reports = input();

    // Count safe reports
    let count_safe = reports.iter().filter(|&v| is_safe(v)).count();

    count_safe
}

fn is_safe(report: &Vec<i32>) -> bool {
    let sign = (report[1] - report[0]).signum();
    report.windows(2).all(|x| {
        let diff = (x[1] - x[0]) * sign;
        diff >= 1 && diff <= 3
    })
}

#[allow(dead_code)]
pub fn puzzle_2() -> usize {
    // Get input
    let reports = input();

    // Count safe reports
    let count_safe = reports
        .iter()
        .filter(|report| {
            // `report` is safe if it is safe in the ordinary sense ..
            if is_safe(report) {
                return true;
            }
            // .. or if it is safe when removing any level
            for i in 0..report.len() {
                let report_without_i = report
                    .iter()
                    .enumerate()
                    .filter(|(j, _)| *j != i)
                    .map(|(_, x)| *x)
                    .collect();
                if is_safe(&&report_without_i) {
                    return true;
                }
            }

            false
        })
        .count();

    count_safe
}
