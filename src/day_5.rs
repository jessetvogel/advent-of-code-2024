use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn input() -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let file = File::open("input/5.txt").unwrap();
    let reader = BufReader::new(file);

    let mut rules = Vec::new();
    let mut updates = Vec::new();

    let mut rules_done = false;
    for line in reader.lines() {
        let line = line.unwrap();
        if line == "" {
            rules_done = true;
            continue;
        }
        if !rules_done {
            let parts: Vec<u32> = line.split("|").map(|x| x.parse().unwrap()).collect();
            rules.push((parts[0], parts[1]));
        } else {
            updates.push(line.split(",").map(|x| x.parse().unwrap()).collect());
        }
    }

    (rules, updates)
}

#[allow(dead_code)]
pub fn puzzle_1() -> u32 {
    // Get input
    let (rules, updates) = input();

    // Store the rules in a HashSet
    let rules: HashSet<(u32, u32)> = HashSet::from_iter(rules.into_iter());

    // Filter the updates by correctness, map to the middle page numbers and add them
    let total = updates
        .into_iter()
        .filter(|update| is_correct(update, &rules))
        .map(|ordering| ordering[ordering.len() / 2])
        .sum();

    total
}

fn is_correct(update: &Vec<u32>, rules: &HashSet<(u32, u32)>) -> bool {
    // For the update to be correct, there must be, for every i < j,
    // *no* rule which states that the j'th page comes before the i'th page
    let n = update.len();
    for i in 0..n {
        for j in i + 1..n {
            if rules.contains(&(update[j], update[i])) {
                return false;
            }
        }
    }
    return true;
}

#[allow(dead_code)]
pub fn puzzle_2() -> u32 {
    // Get input
    let (rules, updates) = input();

    // Store the rules in a HashSet
    let rules: HashSet<(u32, u32)> = HashSet::from_iter(rules.into_iter());

    // Filter out the correct updates, sort updates, map to the middle page numbers and add them
    let total = updates
        .into_iter()
        .filter(|update| !is_correct(update, &rules))
        .map(|mut update| {
            sort_pages(&mut update, &rules);
            update[update.len() / 2]
        })
        .sum();

    total
}

fn sort_pages(update: &mut Vec<u32>, rules: &HashSet<(u32, u32)>) {
    let n = update.len();

    // For i = 0, ..., n - 1, we determine the page at position i as follows:
    // If the page at position j >= i is such that, for all k > j, there is *no* rule that the k'th page
    // should come before the j'th page, then we may replace page j with the page at  position i.
    for i in 0..n {
        for j in i..n {
            if (j + 1..n).all(|k| !rules.contains(&(update[k], update[j]))) {
                update.swap(i, j);
                break;
            }
        }
    }
}
