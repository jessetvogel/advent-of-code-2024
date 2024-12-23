use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn input() -> Vec<(String, String)> {
    let file = File::open("input/23.txt").unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let parts: Vec<&str> = line.trim().split("-").collect();
            (parts[0].into(), parts[1].into())
        })
        .collect()
}

#[allow(dead_code)]
pub fn puzzle_1() -> usize {
    // Get input
    let connections = input();

    // Make list of all computers
    let computers: HashSet<String> = connections
        .iter()
        .map(|(a, b)| vec![a.clone(), b.clone()])
        .flatten()
        .collect();

    // Convert vector to hashset for quick lookup
    let connections: HashSet<(String, String)> = HashSet::from_iter(connections);

    // Find groups of three connected computers, in which at least one starts with a 't'
    let mut count = 0;
    for (a, b) in &connections {
        // Find all computers `c` such that a - b, b - c and c -a
        for c in &computers {
            if (a.starts_with('t') || b.starts_with('t') || c.starts_with('t'))
                && is_connected(a, c, &connections)
                && is_connected(b, c, &connections)
            {
                count += 1;
            }
        }
    }
    count /= 3; // every group of three was counted trice

    count
}

fn is_connected(a: &String, b: &String, connections: &HashSet<(String, String)>) -> bool {
    connections.contains(&(a.clone(), b.clone())) || connections.contains(&(b.clone(), a.clone()))
}

#[allow(dead_code)]
pub fn puzzle_2() -> String {
    // Get input
    let connections = input();

    // Make list of all computers
    let computers: HashSet<String> = connections
        .iter()
        .map(|(a, b)| vec![a.clone(), b.clone()])
        .flatten()
        .collect();

    // Convert vector to hashset for quick lookup
    let connections: HashSet<(String, String)> = HashSet::from_iter(connections);

    // List of all interconnected groups
    let mut groups: Vec<Vec<String>> = vec![vec![]];

    // For all computers `a` and interconnected groups,
    // if `a` is connected to all computers in that group,
    // create a new group including `a`
    for a in &computers {
        let mut new_groups = vec![];
        for group in &groups {
            if group.iter().all(|b| is_connected(a, b, &connections)) {
                let mut new_group = group.clone();
                new_group.push(a.clone());
                new_groups.push(new_group);
            }
        }
        groups.extend(new_groups);
    }

    // Find the largest interconnected group
    let largest_group = groups
        .iter_mut()
        .max_by(|a, b| a.len().cmp(&b.len()))
        .unwrap();
    
    // Sort by computernames
    largest_group.sort();

    // Join with commas
    largest_group.join(",")
}
