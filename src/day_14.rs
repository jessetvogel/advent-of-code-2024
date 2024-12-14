use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

#[derive(Debug)]
struct Robot {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

fn input() -> Vec<Robot> {
    let file = File::open("input/14.txt").unwrap();
    let reader = BufReader::new(file);

    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

    reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let capture = re.captures(&line);
            let (_, [x, y, vx, vy]) = capture.unwrap().extract();
            Robot {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
                vx: vx.parse().unwrap(),
                vy: vy.parse().unwrap(),
            }
        })
        .collect()
}

#[allow(dead_code)]
pub fn puzzle_1() -> usize {
    // Get input
    let robots = input();

    // Size of the space the robots are in
    let width = 101;
    let height = 103;

    let mut quadrant_1 = 0; // top left
    let mut quadrant_2 = 0; // top right
    let mut quadrant_3 = 0; // bottom left
    let mut quadrant_4 = 0; // bottom right

    for robot in &robots {
        // Compute the position of the robot after 100 seconds
        let x = (robot.x + robot.vx * 100).rem_euclid(width);
        let y = (robot.y + robot.vy * 100).rem_euclid(height);

        let left = x < width / 2;
        let top = y < height / 2;
        let bottom = y > height / 2;
        let right = x > width / 2;

        if top && left {
            quadrant_1 += 1;
        }
        if top && right {
            quadrant_2 += 1;
        }
        if bottom && left {
            quadrant_3 += 1;
        }
        if bottom && right {
            quadrant_4 += 1;
        }
    }

    // The safety score is the product of the number of robots in each quadrant
    quadrant_1 * quadrant_2 * quadrant_3 * quadrant_4
}

#[allow(dead_code)]
pub fn puzzle_2() -> usize {
    // The following code creates a visual tool.
    // There appears to be a pattern at every time t = 63 mod 103 and at every t = 82 mod 101
    // Hence, there is expected to happen something when both are true, at t = 6243.
    // Indeed, a christmas tree is formed!

    // ###############################
    // #                             #
    // #                             #
    // #                             #
    // #                             #
    // #              #              #
    // #             ###             #
    // #            #####            #
    // #           #######           #
    // #          #########          #
    // #            #####            #
    // #           #######           #
    // #          #########          #
    // #         ###########         #
    // #        #############        #
    // #          #########          #
    // #         ###########         #
    // #        #############        #
    // #       ###############       #
    // #      #################      #
    // #        #############        #
    // #       ###############       #
    // #      #################      #
    // #     ###################     #
    // #    #####################    #
    // #             ###             #
    // #             ###             #
    // #             ###             #
    // #                             #
    // #                             #
    // #                             #
    // #                             #
    // ###############################

    6243

    // // Get input
    // let mut robots = input();

    // // Size of the space the robots are in
    // let width = 101;
    // let height = 103;

    // let mut time = 0;

    // loop {
    //     // Clear screen
    //     println!("\x1B[2J\x1B[1;1H");

    //     // Print current time
    //     println!("Time: {time} seconds");

    //     // Create a 2D array of the number of robots
    //     let mut space = [[0; 101]; 103];
    //     for robot in &robots {
    //         space[robot.y as usize][robot.x as usize] += 1;
    //     }

    //     // Print the 2D array
    //     for y in 0..height as usize {
    //         let row: String = space[y]
    //             .map(|x| if x > 0 { '#' } else { ' ' })
    //             .iter()
    //             .collect();
    //         println!("{}", row);
    //     }

    //     // If 'exit' is typed, stop otherwise go to next step
    //     let mut buffer = String::new();
    //     let stdin = std::io::stdin();
    //     stdin.read_line(&mut buffer).unwrap();
    //     let dt = match buffer.trim().parse::<i32>() {
    //         Ok(x) => x,
    //         Err(_) => 1,
    //     };
    //     for robot in robots.iter_mut() {
    //         robot.x = (robot.x + dt * robot.vx).rem_euclid(width);
    //         robot.y = (robot.y + dt * robot.vy).rem_euclid(height);
    //     }
    //     time += dt;
    // }
}
