use std::{collections::HashMap, thread::current};

use aoc_runner_derive::aoc;

fn number_of_digits(num: u64) -> u32 {
    num.ilog10() + 1
}

fn split_number(num: u64) -> (u64, u64) {
    let m = 10_u64.pow(number_of_digits(num) / 2);
    (num / m, num % m)
}

fn count_stones(stones: Vec<u64>, blinks: usize) -> u64 {
    let mut cur_register = HashMap::<u64, u64>::new();
    for stone in stones {
        cur_register.insert(stone, 1);
    }
    let mut new_register;

    for _ in 0..blinks {
        new_register = HashMap::<u64, u64>::new();

        // for each stone in the register, calculate the new value(s) and insert in new register with old count
        for (stone, count) in cur_register.iter() {
            if stone == &0 {
                *new_register.entry(1).or_insert(0) += count;
            } else if (number_of_digits(*stone) % 2) == 0 {
                let (left, right) = split_number(*stone);

                *new_register.entry(left).or_insert(0) += count;
                *new_register.entry(right).or_insert(0) += count;
            } else {
                *new_register.entry(stone * &2024).or_insert(0) += count;
            }
        }

        cur_register = new_register;
    }

    cur_register.iter().map(|(_, count)| *count as u64).sum()
}

#[aoc(day11, part1)]
fn part1(content: &str) -> u64 {
    let stones = content
        .split_whitespace()
        .map(|w| w.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    count_stones(stones, 25)
}

#[aoc(day11, part2)]
fn part2(content: &str) -> u64 {
    let stones = content
        .split_whitespace()
        .map(|w| w.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    count_stones(stones, 75)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "125 17";

    #[test]
    fn test_part1() {
        assert_eq!(count_stones(vec![0, 1, 10, 99, 999], 1), 7);
        assert_eq!(count_stones(vec![125, 17], 6), 22);
        assert_eq!(part1(&INPUT), 55312);
    }
}
