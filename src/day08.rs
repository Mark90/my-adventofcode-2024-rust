use std::collections::{HashMap, HashSet};

use aoc_runner_derive::aoc;

fn valid_antinode(antinode: (i32, i32), width: i32, height: i32) -> bool {
    antinode.0 >= 0 && antinode.1 >= 0 && antinode.0 < width && antinode.1 < height
}

#[aoc(day8, part1)]
fn part1(content: &str) -> i32 {
    let width = content.lines().next().unwrap().len() as i32;
    let height = content.lines().count() as i32;

    let mut antennas = HashMap::<char, Vec<(i32, i32)>>::new();

    let mut antinodes = HashSet::<(i32, i32)>::new();
    for (y, line) in content.lines().enumerate() {
        for (x, freq) in line.chars().enumerate() {
            if freq == '.' {
                continue;
            }
            let node = (x as i32, y as i32);

            antennas
                .entry(freq)
                .or_insert_with(Vec::new)
                .iter_mut()
                .for_each(|other| {
                    let xdiff = node.0 - other.0;
                    let ydiff = node.1 - other.1;

                    let antinode1 = (node.0 + xdiff, node.1 + ydiff);
                    let antinode2 = (other.0 - xdiff, other.1 - ydiff);
                    if valid_antinode(antinode1, width, height) {
                        antinodes.insert(antinode1);
                    }
                    if valid_antinode(antinode2, width, height) {
                        antinodes.insert(antinode2);
                    }
                });

            antennas.get_mut(&freq).unwrap().push(node);
        }
    }

    antinodes.len() as i32
}

#[aoc(day8, part2)]
fn part2(content: &str) -> i32 {
    let width = content.lines().next().unwrap().len() as i32;
    let height = content.lines().count() as i32;

    let mut antennas = HashMap::<char, Vec<(i32, i32)>>::new();
    let mut antinodes = HashSet::<(i32, i32)>::new();
    for (y, line) in content.lines().enumerate() {
        for (x, freq) in line.chars().enumerate() {
            if freq == '.' {
                continue;
            }
            let node = (x as i32, y as i32);
            // Different to part1; also add antennas as antinodes
            antinodes.insert(node);

            antennas
                .entry(freq)
                .or_insert_with(Vec::new)
                .iter_mut()
                .for_each(|other| {
                    let xdiff = node.0 - other.0;
                    let ydiff = node.1 - other.1;

                    let mut dist: i32 = 1;
                    loop {
                        // Find all antinodes along the line
                        let antinode1 = (node.0 + (xdiff * dist), node.1 + (ydiff * dist));
                        if valid_antinode(antinode1, width, height) {
                            antinodes.insert(antinode1);
                            dist += 1;
                        } else {
                            break;
                        }
                    }

                    dist = 1;
                    loop {
                        // Find all antinodes along the line in the opposite direction
                        let antinode2 = (other.0 - (xdiff * dist), other.1 - (ydiff * dist));
                        if valid_antinode(antinode2, width, height) {
                            antinodes.insert(antinode2);
                            dist += 1;
                        } else {
                            break;
                        }
                    }
                });

            antennas.get_mut(&freq).unwrap().push(node);
        }
    }

    antinodes.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT), 14);
    }
    #[test]
    fn test_part() {
        assert_eq!(part2(&INPUT), 34);
    }
}
