use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

fn input() -> Vec<(usize, usize)> {
    let file = File::open("input/18.txt").unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            let xy: Vec<usize> = line
                .unwrap()
                .trim()
                .split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            (xy[0], xy[1])
        })
        .collect()
}

#[derive(Copy, Clone, PartialEq)]
enum Cell {
    Safe,
    Corrupt,
}

struct Map {
    cells: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
}

impl Map {
    fn from(width: usize, height: usize, bytes: &[(usize, usize)]) -> Self {
        let mut map = Map {
            cells: vec![vec![Cell::Safe; width]; height],
            width,
            height,
        };
        for &(x, y) in bytes {
            map.cells[y][x] = Cell::Corrupt;
        }
        map
    }
}

#[allow(dead_code)]
pub fn puzzle_1() -> usize {
    // Get input
    let bytes = input();

    // Create map (71 x 71) from first kilobyte (1024 bytes)
    let map = Map::from(71, 71, &bytes[0..1024]);

    // Find shortest path length
    map.shortest_path_length()
}

impl Map {
    fn shortest_path_length(&self) -> usize {
        // In a queue, keep track of the cells to look at
        // In a hashmap, keep track of the minimal path lengths to reach each cell
        let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();
        let mut scores: HashMap<(usize, usize), usize> = HashMap::new();

        // Initially, we should look at the starting cell (0, 0) with score 0
        queue.push_back((0, 0, 0));

        // While the queue is non-empty, pick a cell and look around
        while !queue.is_empty() {
            let (x, y, score) = queue.pop_front().unwrap();
            self.find_paths(x, y, score, &mut scores, &mut queue);
        }

        // Finally, look up the score at the end position
        *scores.get(&(self.width - 1, self.height - 1)).unwrap()
    }

    fn find_paths(
        &self,
        x: usize,
        y: usize,
        score: usize,
        scores: &mut HashMap<(usize, usize), usize>,
        queue: &mut VecDeque<(usize, usize, usize)>,
    ) {
        let cell = self.cells[y][x];

        // If the cell is corrupt, we cannot continue
        if cell == Cell::Corrupt {
            return;
        }

        // If the current score is already at least as good as the new score, nothing to do
        if scores.get(&(x, y)).is_some_and(|&s| s <= score) {
            return;
        }

        // Insert or update new score
        scores.insert((x, y), score);

        // Add new positions to consider to the queue
        if x > 0 {
            queue.push_back((x - 1, y, score + 1));
        }
        if y > 0 {
            queue.push_back((x, y - 1, score + 1));
        }
        if x < self.width - 1 {
            queue.push_back((x + 1, y, score + 1));
        }
        if y < self.height - 1 {
            queue.push_back((x, y + 1, score + 1));
        }
    }

    fn is_escape_possible(&self) -> bool {
        // Do a floodfill to detect if the end is reachable from the start
        let mut reachable: HashSet<(usize, usize)> = HashSet::new();
        self.fill(0, 0, &mut reachable);
        reachable.contains(&(self.width - 1, self.height - 1))
    }

    fn fill(&self, x: usize, y: usize, reachable: &mut HashSet<(usize, usize)>) {
        if self.cells[y][x] == Cell::Corrupt {
            return;
        }
        if reachable.contains(&(x, y)) {
            return;
        }
        reachable.insert((x, y));
        if x > 0 {
            self.fill(x - 1, y, reachable);
        }
        if y > 0 {
            self.fill(x, y - 1, reachable);
        }
        if x < self.width - 1 {
            self.fill(x + 1, y, reachable);
        }
        if y < self.height - 1 {
            self.fill(x, y + 1, reachable);
        }
    }
}

#[allow(dead_code)]
pub fn puzzle_2() -> String {
    // Get input
    let bytes = input();

    // Create map (71 x 71) from first kilobyte (1024 bytes) - escape is still possible
    let mut map = Map::from(71, 71, &bytes[0..1024]);

    // Corrupt the coordinates one by one, until there is no escape possible
    for &(x, y) in &bytes[1024..] {
        map.cells[y][x] = Cell::Corrupt;
        if !map.is_escape_possible() {
            return format!("{x},{y}");
        }
    }

    return "?".into();
}
