use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(PartialEq)]
enum Move {
    Left,
    Right,
    Up,
    Down,
}

#[derive(PartialEq, Clone, Copy)]
enum Cell1 {
    Empty,
    Box,
    Wall,
}

struct Map1 {
    cells: Vec<Vec<Cell1>>,
    width: usize,
    height: usize,
    robot_x: i32,
    robot_y: i32,
}

#[derive(PartialEq, Clone, Copy)]
enum Cell2 {
    Empty,
    BoxLeft,
    BoxRight,
    Wall,
}

struct Map2 {
    cells: Vec<Vec<Cell2>>,
    width: usize,
    height: usize,
    robot_x: i32,
    robot_y: i32,
}

fn input() -> (Map1, Vec<Move>) {
    let file = File::open("input/15.txt").unwrap();
    let reader = BufReader::new(file);

    let mut cells: Vec<Vec<Cell1>> = Vec::new();
    let mut robot_x = 0;
    let mut robot_y = 0;
    let mut moves: Vec<Move> = Vec::new();

    let mut cell_x;
    let mut cell_y = 0;
    let mut is_reading_cells = true;

    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.trim();
        if line.is_empty() {
            is_reading_cells = false;
            continue;
        }
        if is_reading_cells {
            cell_x = -1;
            cells.push(
                line.chars()
                    .into_iter()
                    .map(|c| {
                        cell_x += 1;
                        match c {
                            '.' => Cell1::Empty,
                            'O' => Cell1::Box,
                            '#' => Cell1::Wall,
                            '@' => {
                                robot_x = cell_x;
                                robot_y = cell_y;
                                Cell1::Empty
                            }
                            _ => panic!("invalid char {}", c),
                        }
                    })
                    .collect(),
            );
            cell_y += 1;
        } else {
            moves.extend(line.chars().map(|c| match c {
                '<' => Move::Left,
                '>' => Move::Right,
                '^' => Move::Up,
                'v' => Move::Down,
                _ => panic!("invalid char {}", c),
            }));
        }
    }

    let height = cells.len();
    let width = cells[0].len();

    (
        Map1 {
            cells,
            width,
            height,
            robot_x,
            robot_y,
        },
        moves,
    )
}

#[allow(dead_code)]
pub fn puzzle_1() -> usize {
    // Get input
    let (mut map, moves) = input();

    // Apply robot moves
    for mov in &moves {
        map.apply_move(mov);
    }

    // Return the sum of GPS coordinates
    map.gps_total()
}

#[allow(dead_code)]
pub fn puzzle_2() -> usize {
    // Get input
    let (map, moves) = input();

    // Transform the map to the doubly-sized map
    let mut map = Map2::from(map);

    // Apply robot moves
    for mov in &moves {
        map.apply_move(mov);
    }

    // Return the sum of GPS coordinates
    map.gps_total()
}

impl Map1 {
    fn get(&self, x: i32, y: i32) -> Cell1 {
        self.cells[y as usize][x as usize]
    }

    fn set(&mut self, x: i32, y: i32, cell: Cell1) {
        self.cells[y as usize][x as usize] = cell;
    }

    fn apply_move(&mut self, mov: &Move) {
        let (dx, dy) = match mov {
            Move::Left => (-1, 0),
            Move::Right => (1, 0),
            Move::Up => (0, -1),
            Move::Down => (0, 1),
        };

        match self.get(self.robot_x + dx, self.robot_y + dy) {
            Cell1::Empty => {
                // Robot is unobstructed, so update its position
                self.robot_x += dx;
                self.robot_y += dy;
            }
            Cell1::Wall => {
                // Robot cannot move
            }
            Cell1::Box => {
                // Look ahead until encountering a wall or empty space
                let mut t = 2;
                loop {
                    match self.get(self.robot_x + t * dx, self.robot_y + t * dy) {
                        Cell1::Wall => {
                            // Robot cannot move
                            return;
                        }
                        Cell1::Box => {
                            // Continue looking ahead
                            t += 1;
                        }
                        Cell1::Empty => {
                            // Robot can move the boxes!
                            // We simply add a box at the end, and remove the box at the beginning
                            self.set(self.robot_x + t * dx, self.robot_y + t * dy, Cell1::Box);
                            self.robot_x += dx;
                            self.robot_y += dy;
                            self.set(self.robot_x, self.robot_y, Cell1::Empty);
                            return;
                        }
                    }
                }
            }
        }
    }

    fn gps_total(&self) -> usize {
        let mut gps_total = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if self.cells[y][x] == Cell1::Box {
                    gps_total += 100 * y + x;
                }
            }
        }
        gps_total
    }
}

impl From<Map1> for Map2 {
    fn from(map: Map1) -> Self {
        let width = 2 * map.width;
        let height = map.height;
        let cells = map
            .cells
            .iter()
            .map(|row| {
                let mut new_row = Vec::with_capacity(width);
                for cell in row {
                    match cell {
                        Cell1::Empty => {
                            new_row.push(Cell2::Empty);
                            new_row.push(Cell2::Empty);
                        }
                        Cell1::Box => {
                            new_row.push(Cell2::BoxLeft);
                            new_row.push(Cell2::BoxRight);
                        }
                        Cell1::Wall => {
                            new_row.push(Cell2::Wall);
                            new_row.push(Cell2::Wall);
                        }
                    }
                }
                new_row
            })
            .collect();
        let robot_x = map.robot_x * 2;
        let robot_y = map.robot_y;

        Map2 {
            cells,
            width,
            height,
            robot_x,
            robot_y,
        }
    }
}

impl Map2 {
    fn get(&self, x: i32, y: i32) -> Cell2 {
        self.cells[y as usize][x as usize]
    }

    fn set(&mut self, x: i32, y: i32, cell: Cell2) {
        self.cells[y as usize][x as usize] = cell;
    }

    fn apply_move(&mut self, mov: &Move) {
        // Cases left/right and up/down are treated separately
        match mov {
            Move::Left | Move::Right => {
                let dx = if *mov == Move::Left { -1 } else { 1 };
                match self.get(self.robot_x + dx, self.robot_y) {
                    Cell2::Empty => {
                        // Robot is unobstructed, so update its position
                        self.robot_x += dx;
                    }
                    Cell2::Wall => {
                        // Robot cannot move
                    }
                    Cell2::BoxLeft | Cell2::BoxRight => {
                        // Look ahead until encountering a wall or empty space
                        let mut t = 2;
                        loop {
                            match self.get(self.robot_x + t * dx, self.robot_y) {
                                Cell2::Wall => {
                                    // Robot cannot move
                                    return;
                                }
                                Cell2::BoxLeft | Cell2::BoxRight => {
                                    // Continue looking ahead
                                    t += 1;
                                }
                                Cell2::Empty => {
                                    // Robot can move the boxes!
                                    // Shift cells one place
                                    for i in (1..t + 1).rev() {
                                        self.set(
                                            self.robot_x + i * dx,
                                            self.robot_y,
                                            self.get(self.robot_x + (i - 1) * dx, self.robot_y),
                                        );
                                    }
                                    self.robot_x += dx;
                                    return;
                                }
                            }
                        }
                    }
                }
            }
            Move::Up | Move::Down => {
                let dy = if *mov == Move::Up { -1 } else { 1 };
                match self.get(self.robot_x, self.robot_y + dy) {
                    Cell2::Empty => {
                        // Robot is unobstructed, so update its position
                        self.robot_y += dy;
                    }
                    Cell2::Wall => {
                        // Robot cannot move
                    }
                    Cell2::BoxLeft | Cell2::BoxRight => {
                        // Find out if, and if so which, boxes can be moved
                        let mut boxes: Vec<(i32, i32)> = Vec::new();
                        if can_boxes_be_moved(self, self.robot_x, self.robot_y + dy, dy, &mut boxes)
                        {
                            // Sort the boxes, so that the boxes furthest away are first.
                            // Then we move the boxes, and leave empty cells in their place.
                            // Sorting guarantees we do not overwrite in a bad way
                            boxes.sort_by(|&(_, y1), &(_, y2)| (dy * y1).cmp(&(dy * y2)));
                            for &(box_x, box_y) in boxes.iter().rev() {
                                self.set(box_x, box_y + dy, self.get(box_x, box_y));
                                self.set(box_x, box_y, Cell2::Empty);
                            }
                            // Update robot position
                            self.robot_y += dy;
                        }
                    }
                }
            }
        }
    }

    fn gps_total(&self) -> usize {
        let mut gps_total = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if self.cells[y][x] == Cell2::BoxLeft {
                    gps_total += 100 * y + x;
                }
            }
        }
        gps_total
    }
}

fn can_boxes_be_moved(map: &Map2, x: i32, y: i32, dy: i32, boxes: &mut Vec<(i32, i32)>) -> bool {
    if boxes.contains(&(x, y)) {
        return true;
    }

    match map.get(x, y) {
        Cell2::Empty => return true,
        Cell2::Wall => return false,
        Cell2::BoxLeft => {
            boxes.push((x, y));
            let box_can_move = can_boxes_be_moved(map, x, y + dy, dy, boxes);
            return box_can_move && can_boxes_be_moved(map, x + 1, y, dy, boxes);
        }
        Cell2::BoxRight => {
            boxes.push((x, y));
            let box_can_move = can_boxes_be_moved(map, x, y + dy, dy, boxes);
            return box_can_move && can_boxes_be_moved(map, x - 1, y, dy, boxes);
        }
    }
}
