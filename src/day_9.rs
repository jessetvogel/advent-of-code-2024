use std::collections::HashSet;

fn input() -> Vec<usize> {
    std::fs::read_to_string("input/9.txt")
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10))
        .filter(|x| x.is_some())
        .map(|x| x.unwrap() as usize)
        .collect()
}

#[allow(dead_code)]
pub fn puzzle_1() -> usize {
    // Get input
    let mut disk = input();

    // Move file blocks as described in the puzzle description, and keep track of the checksum at every step
    let mut checksum = 0;

    let mut i = 0; // index of the chunk we consider from the start
    let mut j = disk.len() - 1; // index of the file at the end of the disk from which we are taking blocks
    let mut position = 0 as usize; // current block position

    while i <= j {
        // When i is even, `disk[i]` denotes the length of a file
        // In that case, we compute the contribution of that file to the checksum
        if i % 2 == 0 {
            let file_id = i / 2;
            let file_size = disk[i];
            // TODO: compactify with some triangle formula
            for k in 0..file_size {
                checksum += (position + k) * file_id;
            }
            // Go to next chunk and increment the position
            i += 1;
            position += file_size;
            continue;
        }

        // When i is odd, `disk[i]` denotes the length of a chunk of free space
        // If there is no (more) free space, we can go to the next chunk
        if disk[i] == 0 {
            i += 1;
            continue;
        }

        // If the file at the end of the disk is empty (completely transferred), go to the file before that
        if disk[j] == 0 {
            j -= 2;
            continue;
        }

        // The number of blocks to move from the file at the end of the disk into the free space, is the minimum
        // of how large that file is, and how much free space there is
        let blocks_to_move = usize::min(disk[i], disk[j]);
        disk[i] -= blocks_to_move;
        disk[j] -= blocks_to_move;

        // Compute the contribution to the checksum
        let file_id = j / 2;
        for k in 0..blocks_to_move {
            checksum += (position + k) * file_id;
        }

        // Increment the position
        position += blocks_to_move;
    }

    checksum
}

#[allow(dead_code)]
pub fn puzzle_2() -> usize {
    // Get input
    let mut disk = input();

    // Compute the initial checksum of the file system
    let mut checksum = 0;
    let mut position = 0;
    for i in 0..disk.len() {
        if i % 2 == 0 {
            let file_id = i / 2;
            for k in 0..disk[i] {
                checksum += (position + k) * file_id;
            }
        }
        position += disk[i];
    }

    // Move whole files as described in the puzzle description, and keep track changes in the checksum at every step
    let mut j = disk.len() - 1;
    let mut file_id_j = j / 2;
    let mut position_j = disk.iter().sum::<usize>() - disk[j];
    let mut moved_files = HashSet::new();

    while j > 0 {
        // We want to move the file with following size
        let file_size = disk[j];

        let mut did_move = false;
        let mut position_i = 0;
        for i in 0..j {
            // If there is free space, and enough of it, move the file ..
            if i % 2 == 1 && disk[i] >= file_size {
                disk[i] -= file_size; // reduce free space
                disk[j] = 0; // remove file
                disk.insert(i, file_size); // insert file of size `file_size`
                disk.insert(i, 0); // insert free space of size 0
                moved_files.insert(position_i); // keep track of where files where moved

                // .. and update the checksum
                for k in 0..file_size {
                    checksum += (position_i + k) * file_id_j;
                    checksum -= (position_j + k) * file_id_j;
                }

                did_move = true;
                break;
            }

            position_i += disk[i];
        }

        // Move on to the file before this file by updating `j`, `position_j` and `file_id_j`
        // NOTE: If we did move this file, we do not need to change `j` since we inserted two elements in the `disk` vector
        // NOTE: If we encounter a file that was already moved, move to the file before that (but do NOT change `file_id_j`)
        j -= if did_move { 0 } else { 2 };
        position_j -= disk[j + 1] + disk[j];
        file_id_j -= 1;
        while moved_files.contains(&position_j) {
            j -= 2;
            position_j -= disk[j + 1] + disk[j];
        }
    }

    checksum
}
