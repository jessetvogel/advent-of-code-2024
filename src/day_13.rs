use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

#[derive(Debug)]

struct Point {
    x: i64,
    y: i64,
}

struct Game {
    a: Point,
    b: Point,
    prize: Point,
}

fn read_point(reader: &mut BufReader<File>, re: &Regex) -> Option<Point> {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let capture = re.captures(&line);
    if capture.is_none() {
        return None;
    }
    let (_, [x, y]) = capture.unwrap().extract();
    Some(Point {
        x: x.parse().unwrap(),
        y: y.parse().unwrap(),
    })
}

fn input() -> Vec<Game> {
    let file = File::open("input/13.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut games = Vec::new();
    let re = Regex::new(r"X[\+=](\d+), Y[\+=](\d+)").unwrap();
    loop {
        let a = match read_point(&mut reader, &re) {
            Some(a) => a,
            None => break,
        };
        let b = read_point(&mut reader, &re).unwrap();
        let prize = read_point(&mut reader, &re).unwrap();
        read_point(&mut reader, &re); // read empty line
        games.push(Game { a, b, prize });
    }

    games
}

#[allow(dead_code)]
pub fn puzzle_1() -> usize {
    // Get input
    let games = input();

    count_tokens(&games)
}

fn count_tokens(games: &Vec<Game>) -> usize {
    // The problem translates to a linear system of equations:
    //
    // (a.x b.x) (press_a)   (prize.x)
    // (a.y b.y) (press_b) = (prize.y)
    //
    // We obtain (press_a, press_b) by inverting the matrix. Then keep track of the number of tokens.
    // Note: the solution must be integral, so perform integer division, and check if the solution is valid.
    let mut tokens = 0;
    for game in games {
        let det = game.a.x * game.b.y - game.b.x * game.a.y;
        let press_a = (game.b.y * game.prize.x - game.b.x * game.prize.y) / det;
        let press_b = (-game.a.y * game.prize.x + game.a.x * game.prize.y) / det;
        if game.a.x * press_a + game.b.x * press_b == game.prize.x
            && game.a.y * press_a + game.b.y * press_b == game.prize.y
        {
            tokens += (press_a as usize) * 3 + (press_b as usize) * 1;
        }
    }

    tokens
}

#[allow(dead_code)]
pub fn puzzle_2() -> usize {
    // Get input
    let mut games = input();

    // Modify the prize coordinates
    for game in games.iter_mut() {
        game.prize.x += 10000000000000;
        game.prize.y += 10000000000000;
    }

    count_tokens(&games)
}
