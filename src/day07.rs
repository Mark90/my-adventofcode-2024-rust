use std::collections::HashMap;

use aoc_runner_derive::aoc;

use itertools::{repeat_n, Itertools};

const MULTIPLY: u8 = '*' as u8;
const ADD: u8 = '+' as u8;
const COMBINE: u8 = '|' as u8;

const OPERATORS_PART1: [u8; 2] = [MULTIPLY, ADD];
const OPERATORS_PART2: [u8; 3] = [MULTIPLY, ADD, COMBINE];

fn parse_equation(line: &str) -> (u64, Vec<u64>) {
    line.split_once(':')
        .map(|(x, y)| {
            (
                x.parse::<u64>().unwrap(),
                y.split_whitespace()
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .unwrap()
}

fn parse_equations(content: &str) -> HashMap<usize, Vec<(u64, Vec<u64>)>> {
    // Parse equations, grouping them by the amount of numbers
    let mut equations = HashMap::<usize, Vec<(u64, Vec<u64>)>>::new();
    for line in content.lines() {
        let (total, numbers) = parse_equation(line);

        equations
            .entry(numbers.len())
            .or_insert_with(Vec::new)
            .push((total, numbers));
    }
    equations
}

fn calibrate_equation(op_sequences: &Vec<Vec<&u8>>, total: &u64, numbers: &Vec<u64>) -> u64 {
    // 'Calibrate' the given numbers by trying out all operator sequences until one matches the total
    for op_sequence in op_sequences {
        let result = numbers
            .iter()
            .enumerate()
            .fold(0u64, |acc, (idx, number)| match idx {
                0 => acc + number,
                i => match op_sequence[i - 1] {
                    &MULTIPLY => acc * number,
                    &ADD => acc + number,
                    &COMBINE => (acc * 10u64.pow(number_of_digits(&number))) + number,
                    _ => panic!("impossible.. or is it? *tun tun tun*"),
                },
            });

        if &result == total {
            return result;
        }
    }

    return 0;
}

fn check_equations_part1(num_ops: usize, equations: Vec<(u64, Vec<u64>)>) -> u64 {
    // Create all operator sequence permutations, i.e. +++, ++*, +**, etc
    let op_sequences: Vec<Vec<&u8>> = repeat_n(OPERATORS_PART1.iter(), num_ops)
        .multi_cartesian_product()
        .collect();

    equations
        .iter()
        .map(|(total, numbers)| calibrate_equation(&op_sequences, total, numbers))
        .sum()
}

#[aoc(day7, part1)]
fn part1(content: &str) -> u64 {
    let equations = parse_equations(content);

    equations
        .into_iter()
        .map(|(total, numbers)| check_equations_part1(total, numbers))
        .sum()
}

fn number_of_digits(num: &u64) -> u32 {
    num.ilog10() + 1
}

fn calibrate_part2(num_ops: usize, equations: Vec<(u64, Vec<u64>)>) -> u64 {
    // Create all operator sequence permutations, i.e. +++, ++*, +|*, etc
    let op_sequences: Vec<Vec<&u8>> = repeat_n(OPERATORS_PART2.iter(), num_ops)
        .multi_cartesian_product()
        .collect();

    equations
        .iter()
        .map(|(total, numbers)| calibrate_equation(&op_sequences, total, numbers))
        .sum()
}

#[aoc(day7, part2)]
fn part2(content: &str) -> u64 {
    let equations = parse_equations(content);

    equations
        .into_iter()
        .map(|(total, numbers)| calibrate_part2(total, numbers))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT), 3749);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&INPUT), 11387);
    }

    #[test]
    fn test_number_length() {
        assert_eq!(number_of_digits(&1), 1);
        assert_eq!(number_of_digits(&9), 1);
        assert_eq!(number_of_digits(&10), 2);
        assert_eq!(number_of_digits(&50), 2);
        assert_eq!(number_of_digits(&99), 2);
        assert_eq!(number_of_digits(&100), 3);
        assert_eq!(number_of_digits(&999), 3);
        assert_eq!(number_of_digits(&1000), 4);
    }
}
