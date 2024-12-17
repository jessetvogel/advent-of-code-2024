use std::{
    fs::File,
    io::{BufRead, BufReader},
    vec,
};

use regex::Regex;

#[derive(Clone)]
struct Computer {
    a: u64,
    b: u64,
    c: u64,
    ptr: usize,
    program: Vec<u8>,
    output: Vec<u8>,
}

fn input() -> Computer {
    let file = File::open("input/17.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut register_a = String::new();
    let mut register_b = String::new();
    let mut register_c = String::new();
    let mut program = String::new();

    reader.read_line(&mut register_a).unwrap();
    reader.read_line(&mut register_b).unwrap();
    reader.read_line(&mut register_c).unwrap();
    reader.read_line(&mut program).unwrap(); // skip empty line
    reader.read_line(&mut program).unwrap();

    let re = Regex::new(r"[\d,]+").unwrap();

    let a = re.captures(&register_a).unwrap().get(0).unwrap().as_str();
    let a = a.parse::<u64>().unwrap();
    let b = re.captures(&register_b).unwrap().get(0).unwrap().as_str();
    let b = b.parse::<u64>().unwrap();
    let c = re.captures(&register_c).unwrap().get(0).unwrap().as_str();
    let c = c.parse::<u64>().unwrap();

    let program = re.captures(&program).unwrap().get(0).unwrap().as_str();
    let program = program
        .split(",")
        .map(|x| x.parse::<u8>().unwrap())
        .collect();

    Computer {
        a,
        b,
        c,
        ptr: 0,
        program,
        output: vec![],
    }
}

#[allow(dead_code)]
pub fn puzzle_1() -> String {
    // Get input
    let mut computer = input();

    // Keep executing instructions until the computer halts
    while computer.advance() {}

    // Join the output of the program with commas
    computer
        .output
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

impl Computer {
    fn advance(&mut self) -> bool {
        // Halt when trying to read out of bounds
        if self.ptr > self.program.len() - 2 {
            return false;
        }

        let opcode = self.program[self.ptr];
        let operand = self.program[self.ptr + 1];

        match opcode {
            0 => {
                // ADV
                self.a >>= self.combo(operand);
                self.ptr += 2;
            }
            1 => {
                // BXL
                self.b ^= self.literal(operand);
                self.ptr += 2;
            }
            2 => {
                // BST
                self.b = self.combo(operand) % 8;
                self.ptr += 2;
            }
            3 => {
                // JNZ
                if self.a != 0 {
                    self.ptr = self.literal(operand) as usize;
                } else {
                    self.ptr += 2;
                }
            }
            4 => {
                // BXC
                self.b ^= self.c;
                self.ptr += 2;
            }
            5 => {
                // OUT
                self.output.push((self.combo(operand) % 8) as u8);
                self.ptr += 2;
            }
            6 => {
                // BDV
                self.b = self.a >> self.combo(operand);
                self.ptr += 2;
            }
            7 => {
                // CDV
                self.c = self.a >> self.combo(operand);
                self.ptr += 2;
            }
            _ => panic!("invalid opcode"),
        }

        true
    }

    fn combo(&self, x: u8) -> u64 {
        match x {
            0 | 1 | 2 | 3 => x as u64,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            7 | _ => panic!("invalid combo operand"),
        }
    }

    fn literal(&self, x: u8) -> u64 {
        x as u64
    }
}

#[allow(dead_code)]
pub fn puzzle_2() -> u64 {
    // Get input
    let computer = input();

    // It is probably not possible to solve this puzzle in full generality,
    // so we only consider specifically the given program:
    //  "2,4,1,1,7,5,0,3,4,7,1,6,5,5,3,0"
    // Decoded, its instructions are:
    // - BST (4): b = a % 8
    // - BXL (1): b ^= 1
    // - CDV (5): c = a >> b
    // - ADV (3): a >>= 3
    // - BXC (7): b ^= c
    // - BXL (6): b ^= 6
    // - OUT (5): print(b % 8)
    // - JNZ (0): if a != 0, loop
    // The program prints numbers between 0 and 8 depending on the value of a.
    // Every step, `a` is shifted three bits to the right.
    // Note that the three MSBs of `a` determine the final output of the program,
    // the three MSBs before that (in combination with the first three) determine the first to last output of the program,
    // etc. Hence, we can iteratively find the values of `a` that produces a desired output.
    // Finally, take the minimum value.
    let a = *invert(&computer.program).iter().min().unwrap();

    a
}

fn invert(output: &[u8]) -> Vec<u64> {
    // Base case, for no output, we have a = 0
    if output.len() == 0 {
        return vec![0];
    }

    // Look for values of `a` that output the last numbers.
    // Combine these values with three new bits for `a` and check if
    // that new value of `a` produces the full output.
    let mut solutions = vec![];
    for a in invert(&output[1..]) {
        for x in 0..8 {
            let a = (a << 3) + x;
            let b = a % 8;
            let b = b ^ 1;
            let c = a >> b;
            let b = b ^ c ^ 6;

            if (b % 8) as u8 == output[0] {
                solutions.push(a);
            }
        }
    }

    solutions
}
