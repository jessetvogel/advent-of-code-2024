use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn input() -> (Vec<u32>, Vec<u32>) {
    let file = File::open("input/25.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut locks = Vec::new();
    let mut keys = Vec::new();

    let mut line = String::new();
    loop {
        line.clear();
        reader.read_line(&mut line).unwrap();

        if line.is_empty() {
            break;
        }

        let is_lock = match line.trim() {
            "#####" => true,
            "....." => false,
            _ => panic!("neither lock nor key"),
        };

        let mut bits: u32 = 0;

        for _ in 0..5 {
            line.clear();
            reader.read_line(&mut line).unwrap();
            for c in line.trim().chars() {
                bits <<= 1;
                match c {
                    '#' => {
                        bits |= 0x1;
                    }
                    '.' => {}
                    _ => panic!("invalid char"),
                }
            }
        }

        reader.read_line(&mut line).unwrap(); // last line of lock/key is not interesting
        reader.read_line(&mut line).unwrap(); // skip newline

        if is_lock {
            locks.push(bits);
        } else {
            keys.push(bits);
        }
    }

    (locks, keys)
}

#[allow(dead_code)]
pub fn puzzle_1() -> usize {
    // Get input
    let (locks, keys) = input();

    // Count the number of lock/key pairs that fit together:
    // the pair fits if the `and` of the pair is zero
    let mut count = 0;
    for lock in &locks {
        for key in &keys {
            if *lock & *key == 0 {
                count += 1;
            }
        }
    }

    count
}
