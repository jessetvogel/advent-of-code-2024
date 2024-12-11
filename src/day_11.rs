use std::collections::HashMap;

fn input() -> Vec<u64> {
    std::fs::read_to_string("input/11.txt")
        .unwrap()
        .trim()
        .split(" ")
        .map(|x| x.parse::<u64>().unwrap())
        .collect()
}

#[allow(dead_code)]
pub fn puzzle_1() -> usize {
    // Get input
    let stones = input();

    // Count how many stones there are after 25 blinks.
    // Note that each stone can be treated independently, since there is no interaction between the stones.
    // Try to be smart by memorizing the counts associated to a stone and the number of blinks.
    let mut memory = HashMap::new();
    stones
        .iter()
        .map(|stone| count_stones(*stone, 25, &mut memory))
        .sum()
}

fn split_digits(n: u64) -> Option<(u64, u64)> {
    // If n has an even number of digits, split the left digits from the right digits
    // If n has an odd number of digits, return None
    let num_digits = {
        let mut d = 0;
        let mut n = n;
        while n > 0 {
            n /= 10;
            d += 1;
        }
        d
    };
    if num_digits % 2 == 1 {
        return None;
    }
    let k = u64::pow(10, num_digits / 2);
    Some((n / k, n % k))
}

fn count_stones(stone: u64, blinks: usize, memory: &mut HashMap<(u64, usize), usize>) -> usize {
    // Starting with a single stone, count how many stones there are after an amount of blinks

    // After zero blinks, there is still one stone
    if blinks == 0 {
        return 1;
    }

    // Check if this question was asked before
    match memory.get(&(stone, blinks)) {
        Some(count) => return *count,
        None => {}
    }

    // Calculate the count recursively according to the specified rules
    let count = {
        // (1) If the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1
        if stone == 0 {
            count_stones(1, blinks - 1, memory)
        } else {
            match split_digits(stone) {
                // (2) If the stone is engraved with a number that has an even number of digits, it is replaced by two stones.
                // The left half of the digits are engraved on the new left stone, and the right half of the digits are
                // engraved on the new right stone.
                Some((left, right)) => {
                    count_stones(left, blinks - 1, memory) + count_stones(right, blinks - 1, memory)
                }
                None => {
                    // (3) If none of the other rules apply, the stone is replaced by a new stone;
                    // the old stone's numbermultiplied by 2024 is engraved on the new stone.
                    count_stones(stone * 2024, blinks - 1, memory)
                }
            }
        }
    };

    // Memorize the count
    memory.insert((stone, blinks), count);

    count
}

#[allow(dead_code)]
pub fn puzzle_2() -> usize {
    // Get input
    let stones = input();

    // Count how many stones there are after 75 blinks.
    // Note that each stone can be treated independently, since there is no interaction between the stones.
    // Try to be smart by memorizing the counts associated to a stone and the number of blinks.
    let mut memory = HashMap::new();
    stones
        .iter()
        .map(|stone| count_stones(*stone, 75, &mut memory))
        .sum()
}
