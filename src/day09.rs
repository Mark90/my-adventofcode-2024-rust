use aoc_runner_derive::aoc;
use itertools::Itertools;

#[derive(Copy, Clone, Debug)]
struct Node {
    file_id: Option<usize>,
}

struct File {
    id: usize,
    size: usize,
}

fn seek_free(disk: &Vec<Node>, start_pos: usize) -> Option<usize> {
    for position in start_pos..disk.len() {
        if disk[position].file_id.is_none() {
            return Some(position);
        }
    }
    return None;
}

fn seek_file(disk: &Vec<Node>, start_pos: usize) -> Option<usize> {
    for position in start_pos..disk.len() {
        if disk[position].file_id.is_some() {
            return Some(position);
        }
    }
    return None;
}

fn rseek_file(disk: &Vec<Node>, start_pos: usize) -> Option<usize> {
    for position in (0..=start_pos).rev() {
        if disk[position].file_id.is_some() {
            return Some(position);
        }
    }
    return None;
}

fn rseek_file_by_id(disk: &Vec<Node>, start_pos: usize, file_id: usize) -> Option<usize> {
    for position in (0..=start_pos).rev() {
        if disk[position].file_id.is_some_and(|id| id == file_id) {
            return Some(position);
        }
    }
    return None;
}

fn to_disk(content: &str) -> Vec<Node> {
    // Parse disk map to digits - add extra 0 as final empty space
    let digits = format!("{}0", content)
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<_>>();

    let mut disk = Vec::<Node>::new();

    for (file_id, space) in digits.chunks(2).enumerate() {
        let file_length = space[0];
        let free_length = space[1];

        for _ in 0..file_length {
            disk.push(Node {
                file_id: Some(file_id),
            })
        }
        for _ in 0..free_length {
            disk.push(Node { file_id: None })
        }
    }

    disk
}

fn to_checksum(disk: &Vec<Node>) -> u64 {
    disk.iter()
        .enumerate()
        .filter_map(|(i, n)| n.file_id.map(|id| (i as u64) * id as u64))
        .sum()
}

#[aoc(day9, part1)]
fn part1(content: &str) -> u64 {
    let mut disk = to_disk(content);

    let mut first_free = seek_free(&disk, 0).unwrap();
    let mut last_file = rseek_file(&disk, &disk.len() - 1).unwrap();
    while first_free < last_file {
        disk.swap(first_free, last_file);
        first_free = seek_free(&disk, first_free).unwrap();
        last_file = rseek_file(&disk, last_file).unwrap();
    }

    to_checksum(&disk)
}

fn to_files(disk: &Vec<Node>) -> Vec<File> {
    let mut files: Vec<File> = Vec::<File>::new();

    for (key, chunk) in &disk
        .iter()
        .filter(|n| n.file_id.is_some())
        .chunk_by(|node| node.file_id.unwrap())
    {
        let file = File {
            id: key,
            size: chunk.count(),
        };
        files.push(file);
    }
    files
}

#[aoc(day9, part2)]
fn part2(content: &str) -> u64 {
    let mut disk = to_disk(content);
    let files: Vec<File> = to_files(&disk);

    let mut first_free = 0 as usize;
    let mut file_start = disk.len() - 1;
    for file in files.iter().rev() {
        first_free = seek_free(&disk, first_free).unwrap();
        // move pointer backwards to start of current file
        file_start = rseek_file_by_id(&disk, file_start, file.id).unwrap();
        file_start -= file.size - 1;

        if file_start < first_free {
            break;
        }

        let mut next_free = first_free;
        while file_start > next_free {
            // From the last known free position, find the next file (which may be the current file) and
            // calculate the free space
            let next_file = seek_file(&disk, next_free).unwrap();
            let free_space = next_file - next_free;
            if free_space >= file.size {
                let offset = file_start - next_free;
                for pos in file_start..(file_start + file.size) {
                    disk.swap(pos, pos - offset);
                }
                break;
            }
            next_free = seek_free(&disk, next_file).unwrap();
        }
    }

    to_checksum(&disk)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT), 1928);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&INPUT), 2858);
    }
}
