use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};
struct Map {
    plants: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

fn input() -> Map {
    let file = File::open("input/12.txt").unwrap();
    let reader = BufReader::new(file);

    let plants: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap().trim().chars().collect())
        .collect();

    let height = plants.len();
    let width = plants[0].len();

    Map {
        plants,
        width,
        height,
    }
}

#[allow(dead_code)]
pub fn puzzle_1() -> usize {
    // Get input
    let map = input();

    // Iterate over the whole map. Whenever we encounter a place we have not visited yet (keep track of this in a hashset),
    // we use a floodfill-like technique to compute the area and the perimeter of that region. From this, we compute the cost
    // for that region.
    let mut cost = 0;
    let mut visited = HashSet::new();
    for x in 0..map.width {
        for y in 0..map.height {
            // If we have already visited (x, y), we continue
            if visited.contains(&(x, y)) {
                continue;
            }

            // If not, compute area and perimeter of this region
            let plant = map.plants[y][x];
            let mut area = 0;
            let mut perimeter = 0;
            let mut region = HashSet::new();
            discover(&map, plant, x, y, &mut area, &mut perimeter, &mut region);

            // Compute cost due to "modern" business practices, and add it to the total
            cost += area * perimeter;

            // Mark the whole region as visited
            visited.extend(region.into_iter());
        }
    }

    cost
}

fn discover(
    map: &Map,
    plant: char,
    x: usize,
    y: usize,
    area: &mut usize,
    perimeter: &mut usize,
    region: &mut HashSet<(usize, usize)>,
) {
    // Check if the plant type matches the given plant type
    // If not, this must be a neighbor of the region, so increment the perimeter by one
    if map.plants[y][x] != plant {
        *perimeter += 1;
        return;
    }

    // If this part of the region was already considered before, skip it
    if region.contains(&(x, y)) {
        return;
    }

    // If this is a new part of the region, increment the area by one ..
    region.insert((x, y));
    *area += 1;

    // .. and check the neighbors
    // Note: for parts of the region on the border, we should always increment the perimeter by one
    if x > 0 {
        discover(map, plant, x - 1, y, area, perimeter, region);
    } else {
        *perimeter += 1;
    }
    if x < map.width - 1 {
        discover(map, plant, x + 1, y, area, perimeter, region);
    } else {
        *perimeter += 1;
    }
    if y > 0 {
        discover(map, plant, x, y - 1, area, perimeter, region);
    } else {
        *perimeter += 1;
    }
    if y < map.height - 1 {
        discover(map, plant, x, y + 1, area, perimeter, region);
    } else {
        *perimeter += 1;
    }
}

#[allow(dead_code)]
pub fn puzzle_2() -> usize {
    // Get input
    let map = input();

    // Again, iterate over the whole map. For each region, which we discover as done above,
    // we compute the number of sides from the hashset containing the parts of the region.
    // From this, we compute the cost for that region.
    let mut cost = 0;
    let mut visited = HashSet::new();
    for x in 0..map.width {
        for y in 0..map.height {
            // If we have already visited (x, y), we continue
            if visited.contains(&(x, y)) {
                continue;
            }

            // If not, compute area of this region and find border parts
            let plant = map.plants[y][x];
            let mut area = 0;
            let mut perimeter = 0;
            let mut region = HashSet::new();
            discover(&map, plant, x, y, &mut area, &mut perimeter, &mut region);

            // Compute number of sides from the region
            let sides = count_sides(&map, &region);

            // Compute cost under the bulk discount, and add it to the total
            cost += area * sides;

            // Mark the whole region as visited
            visited.extend(region.into_iter());
        }
    }

    cost
}

fn count_sides(map: &Map, region: &HashSet<(usize, usize)>) -> usize {
    // Def: 'fence coordinates' are the coordinates in between the grid points, multiplied by two to become integral, and plus (1, 1) to become positive
    // Ex: the fence between (0, 0) and (1, 0) is (1, 0) since it is twice (0.5, 0). In other words, it is the sum of the two coordinates
    let mut fences: HashSet<(usize, usize)> = HashSet::new();
    for &(x, y) in region {
        // Check left
        if x == 0 || !region.contains(&(x - 1, y)) {
            fences.insert((1 + x + x - 1, 1 + y + y));
        }
        // Check right
        if x == map.width - 1 || !region.contains(&(x + 1, y)) {
            fences.insert((1 + x + x + 1, 1 + y + y));
        }
        // Check up
        if y == 0 || !region.contains(&(x, y - 1)) {
            fences.insert((1 + x + x, 1 + y + y - 1));
        }
        // Check down
        if y == map.height - 1 || !region.contains(&(x, y + 1)) {
            fences.insert((1 + x + x, 1 + y + y + 1));
        }
    }

    // Count the number of sides as follows:
    // While there are still fences left, pick a random point, and keep removing the fence in both directions
    // Note that either the x/u-fence-coordinate is odd (horizontal fence) or the y/v-fence-coordinate is odd (vertical fence)
    let mut sides = 0;
    while !fences.is_empty() {
        let &(u, v) = fences.iter().next().unwrap();

        // println!("Removing fence, starting at (u, v) = ({u}, {v})");

        if u % 2 == 1 {
            // Horizontal fence
            let x = (u - 1) / 2;
            let y = if region.contains(&(x, v / 2)) {
                v / 2
            } else {
                v / 2 - 1
            };
            {
                let mut u = u;
                let mut x = x;
                while region.contains(&(x, y)) && fences.contains(&(u, v)) {
                    fences.remove(&(u, v));
                    // println!("- removing (u, v) = ({u}, {v})");
                    if u >= 2 {
                        u -= 2;
                        x -= 1;
                    } else {
                        break;
                    }
                }
            }
            {
                let mut u = u + 2;
                let mut x = x + 1;
                while region.contains(&(x, y)) && fences.contains(&(u, v)) {
                    fences.remove(&(u, v));
                    // println!("- removing (u, v) = ({u}, {v})");
                    u += 2;
                    x += 1;
                }
            }
        } else {
            // Vertical fence
            let y = (v - 1) / 2;
            let x = if region.contains(&(u / 2, y)) {
                u / 2
            } else {
                u / 2 - 1
            };
            {
                let mut v = v;
                let mut y = y;
                while region.contains(&(x, y)) && fences.contains(&(u, v)) {
                    fences.remove(&(u, v));
                    // println!("- removing (u, v) = ({u}, {v})");
                    if v >= 2 {
                        v -= 2;
                        y -= 1;
                    } else {
                        break;
                    }
                }
            }
            {
                let mut v = v + 2;
                let mut y = y + 1;
                while region.contains(&(x, y)) && fences.contains(&(u, v)) {
                    fences.remove(&(u, v));
                    // println!("- removing (u, v) = ({u}, {v})");
                    v += 2;
                    y += 1;
                }
            }
        }

        sides += 1;
    }

    sides
}
