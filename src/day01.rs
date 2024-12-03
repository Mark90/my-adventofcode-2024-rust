use std::collections::HashMap;

use aoc_runner_derive::aoc;

fn to_pair(line: &str) -> (i32, i32) {
    let numbers: Vec<i32> = line
        .split_whitespace()
        .map(|p| p.parse::<i32>().unwrap())
        .collect();
    (numbers[0], numbers[1])
}

#[aoc(day1, part1)]
fn part1(content: &str) -> i32 {
    let number_pairs: Vec<(i32, i32)> = content.lines().map(to_pair).collect();

    let mut list1: Vec<i32> = number_pairs.iter().map(|pair| pair.0).collect();
    let mut list2: Vec<i32> = number_pairs.iter().map(|pair| pair.1).collect();
    list1.sort();
    list2.sort();

    list1
        .iter()
        .enumerate()
        .map(|(position, value)| (list2[position] - value).abs())
        .sum()
}

#[aoc(day1, part2)]
fn part2(content: &str) -> i32 {
    let number_pairs: Vec<(i32, i32)> = content.lines().map(to_pair).collect();

    let mut counts: HashMap<i32, i32> = HashMap::new();
    for number in number_pairs.iter().map(|pair| pair.1) {
        match counts.get(&number) {
            Some(count) => counts.insert(number, count + 1),
            None => counts.insert(number, 1),
        };
    }

    number_pairs
        .iter()
        .map(|pair| pair.0 * counts.get(&pair.0).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = "3   4
    4   3
    2   5
    1   3
    3   9
    3   3";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(&INPUT), 11);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(&INPUT), 31);
    }
}
