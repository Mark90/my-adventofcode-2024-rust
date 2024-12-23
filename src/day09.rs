use aoc_runner_derive::aoc;

#[derive(Copy, Clone, Debug)]
struct Node {
    file_id: Option<usize>,
}

fn seek_free(disk: &Vec<Node>, start_pos: usize) -> Option<usize> {
    for position in start_pos..disk.len() {
        if disk[position].file_id.is_none() {
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

#[aoc(day9, part1)]
fn part1(content: &str) -> u64 {
    let mut disk = Vec::<Node>::new();

    let padded = format!("{}0", content);
    let digits = padded
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<_>>();

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

    let mut first_free = seek_free(&disk, 0).unwrap();
    let mut last_file = rseek_file(&disk, &disk.len() - 1).unwrap();
    while first_free < last_file {
        disk.swap(first_free, last_file);
        first_free = seek_free(&disk, first_free).unwrap();
        last_file = rseek_file(&disk, last_file).unwrap();
    }

    disk.iter()
        .enumerate()
        .filter_map(|(i, n)| n.file_id.map(|id| (i as u64) * id as u64))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT), 1928);
    }
}
