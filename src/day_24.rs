use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

#[derive(Debug)]
enum Op {
    AND,
    OR,
    XOR,
}

impl From<&str> for Op {
    fn from(value: &str) -> Self {
        match value {
            "AND" => Op::AND,
            "OR" => Op::OR,
            "XOR" => Op::XOR,
            _ => panic!("invalid op"),
        }
    }
}

fn input() -> (HashMap<String, bool>, HashMap<String, (String, Op, String)>) {
    let file = File::open("input/24.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut values = HashMap::new();
    let mut formulas = HashMap::new();

    // Read values
    let mut line = String::new();
    let re = Regex::new(r"(\w+): (\d)").unwrap();
    loop {
        line.clear();
        reader.read_line(&mut line).unwrap();
        let line = line.trim();
        if line.is_empty() {
            break;
        }
        let (_, [var, val]) = re.captures(line).expect("expected match").extract();
        let val = val.parse::<i32>().unwrap() > 0;
        values.insert(var.into(), val);
    }

    // Read operations
    let re = Regex::new(r"(\w+) (\w+) (\w+) -> (\w+)").unwrap();
    loop {
        line.clear();
        reader.read_line(&mut line).unwrap();
        let line = line.trim();
        if line.is_empty() {
            break;
        }
        let (_, [left, op, right, res]) = re.captures(line).expect("expected match").extract();
        let op = Op::from(op);
        formulas.insert(res.into(), (left.into(), op, right.into()));
    }

    (values, formulas)
}

#[allow(dead_code)]
pub fn puzzle_1() -> u64 {
    // Get input
    let (mut values, formulas) = input();

    // Resolve z00, ... , z46 and convert to integer
    let mut answer = 0;
    for k in 0..46 {
        if resolve(&format!("z{k:0>2}"), &mut values, &formulas) {
            answer += 1 << k;
        }
    }
    answer
}

fn resolve(
    var: &str,
    values: &mut HashMap<String, bool>,
    formulas: &HashMap<String, (String, Op, String)>,
) -> bool {
    if let Some(value) = values.get(var) {
        return *value;
    }

    let (left, op, right) = formulas.get(var).unwrap();

    let left = resolve(left, values, formulas);
    let right = resolve(right, values, formulas);

    let value = match op {
        Op::AND => left && right,
        Op::OR => left || right,
        Op::XOR => left != right,
    };

    values.insert(var.into(), value);

    value
}

#[allow(dead_code)]
pub fn puzzle_2() -> String {
    // Get input
    let (_values, _formulas) = input();

    // Visualize compute graph by hand and identify the incorrect nodes
    return "css,cwt,gdd,jmv,pqt,z05,z09,z37".into();
}
