use regex::Regex;

fn input() -> String {
    std::fs::read_to_string("input/3.txt").unwrap()
}

#[allow(dead_code)]
pub fn puzzle_1() -> i32 {
    // Get input
    let input = input();

    // Use regex to find `mul(-,-)` patterns
    // Compute multiplication and sum all results
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let total: i32 = re
        .captures_iter(&input)
        .map(|c| c.extract())
        .map(|(_, [a, b])| a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap())
        .sum();

    total
}

#[allow(dead_code)]
pub fn puzzle_2() -> i32 {
    // Get input
    let input = input();

    // Use regex to find `mul(-,-)` patterns, and the `do`s and `don't`s
    // Compute multiplication and sum all results where the machine was enabled
    let re = Regex::new(r"mul\((\d+),(\d+)\)|(d)(o)\(\)|(d)(o)n't\(\)").unwrap();
    let mut total: i32 = 0;
    let mut enabled = true;
    for (str, [a, b]) in re.captures_iter(&input).map(|c| c.extract()) {
        if str == "do()" {
            enabled = true;
        } else if str == "don't()" {
            enabled = false;
        } else if enabled {
            total += a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap();
        }
    }

    total
}
