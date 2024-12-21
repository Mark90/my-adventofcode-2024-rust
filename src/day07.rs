use std::collections::HashMap;

use aoc_runner_derive::aoc;

use itertools::{repeat_n, Itertools};

fn check_equation(line: &str) -> i64 {
    // TODO's
    //  document/rename stuff
    //  try small optimization by sorting equations by length so as to reuse all_ops for multiple equations
    let (total, numbers) = line
        .split_once(':')
        .map(|(x, y)| {
            (
                x.parse::<i64>().unwrap(),
                y.split_whitespace()
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .unwrap();

    // let operators = ['+', '*'];
    let operators = "+*"; // TODO bench this vs vec
    let num_ops = numbers.len();
    // let all_ops: Vec<String> = repeat_n(operators.iter().cloned(), num_ops)
    let all_ops: Vec<String> = repeat_n(operators.chars(), num_ops)
        .multi_cartesian_product()
        .map(|v| v.into_iter().collect::<String>())
        .collect();

    for ops_chars in all_ops {
        let ops = ops_chars.as_bytes();

        let outcome = numbers
            .iter()
            .enumerate()
            .fold(0i64, |acc, (idx, v)| match idx {
                0 => acc + v,
                i => match ops[i - 1] as char {
                    '*' => acc * v,
                    '+' => acc + v,
                    _ => panic!("impossible"),
                },
            });

        if outcome == total {
            return outcome;
        }
    }
    return 0;
}

#[aoc(day7, part1)]
fn part1(content: &str) -> i64 {
    // TODO rewrite to u64 and
    content.lines().map(check_equation).sum()
}

fn number_length2(num: &u64) -> u32 {
    num.ilog10() + 1
}

fn check_equation2(line: &str) -> u64 {
    // TODO's
    //  document/rename stuff
    //  try small optimization by sorting equations by length so as to reuse all_ops for multiple equations
    let (total, numbers) = line
        .split_once(':')
        .map(|(x, y)| {
            (
                x.parse::<u64>().unwrap(),
                y.split_whitespace()
                    .map(|s| s.parse::<u64>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .unwrap();

    // let operators = ['+', '*', '|'];
    let operators = "+*|"; // TODO bench this vs vec
    let num_ops = numbers.len();
    // let all_ops: Vec<String> = repeat_n(operators.iter().cloned(), num_ops)
    let all_ops: Vec<String> = repeat_n(operators.chars(), num_ops)
        .multi_cartesian_product()
        .map(|v| v.into_iter().collect::<String>())
        .collect();

    for ops_chars in all_ops {
        let ops = ops_chars.as_bytes();

        let outcome = numbers
            .iter()
            .enumerate()
            .fold(0u64, |acc, (idx, v)| match idx {
                0 => acc + v,
                i => match ops[i - 1] as char {
                    '*' => acc * v,
                    '+' => acc + v,
                    '|' => (acc * 10u64.pow(number_length2(&v))) + v,
                    _ => panic!("impossible"),
                },
            });

        if outcome == total {
            return outcome;
        }
    }

    return 0;
}

//

// fn check_equations3(num_ops: usize, equations: Vec<(u64, Vec<u64>)>) -> u64 {
//     // TODO's
//     //  document/rename stuff
//     //  try small optimization by sorting equations by length so as to reuse all_ops for multiple equations
//     // let (total, numbers) = line
//     //     .split_once(':')
//     //     .map(|(x, y)| {
//     //         (
//     //             x.parse::<u64>().unwrap(),
//     //             y.split_whitespace()
//     //                 .map(|s| s.parse::<u64>().unwrap())
//     //                 .collect::<Vec<_>>(),
//     //         )
//     //     })
//     //     .unwrap();

//     // let operators = ['+', '*', '|'];
//     let operators = "+*|"; // TODO bench this vs vec
//                            // let num_ops = numbers.len();
//                            // let all_ops: Vec<String> = repeat_n(operators.iter().cloned(), num_ops)
//     let all_ops: Vec<&[u8]> = repeat_n(operators.chars(), num_ops)
//         .multi_cartesian_product()
//         .map(|v| v.into_iter().collect::<String>())
//         .map(|s| s.as_bytes())
//         .collect();

//     for ops_chars in all_ops {
//         // let ops = ops_chars.as_bytes();
//         let ops = ops_chars;

//         let outcome = numbers
//             .iter()
//             .enumerate()
//             .fold(0u64, |acc, (idx, v)| match idx {
//                 0 => acc + v,
//                 i => match ops[i - 1] as char {
//                     '*' => acc * v,
//                     '+' => acc + v,
//                     '|' => (acc * 10u64.pow(number_length2(&v))) + v,
//                     _ => panic!("impossible"),
//                 },
//             });

//         if outcome == total {
//             return outcome;
//         }
//     }

//     return 0;
// }

#[aoc(day7, part2)]
fn part2(content: &str) -> u64 {
    // let mut equations_by_length = HashMap::<usize, Vec<(u64, Vec<u64>)>>::new();

    // for line in content.lines() {
    //     let (total, numbers) = line
    //         .split_once(':')
    //         .map(|(x, y)| {
    //             (
    //                 x.parse::<u64>().unwrap(),
    //                 y.split_whitespace()
    //                     .map(|s| s.parse::<u64>().unwrap())
    //                     .collect::<Vec<_>>(),
    //             )
    //         })
    //         .unwrap();

    //     equations_by_length
    //         .entry(numbers.len())
    //         .or_insert_with(Vec::new)
    //         .push((total, numbers));
    // }

    // println!("MMM {:?}", equations_by_length);

    // let grouped = content.lines().into_iter().map(|l| l.count);

    content.lines().map(check_equation2).sum()

    // equations_by_length.into_iter().map(check_equations3).sum()
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
        assert_eq!(number_length2(&1), 1);
        assert_eq!(number_length2(&9), 1);
        assert_eq!(number_length2(&10), 2);
        assert_eq!(number_length2(&50), 2);
        assert_eq!(number_length2(&99), 2);
        assert_eq!(number_length2(&100), 3);
        assert_eq!(number_length2(&999), 3);
        assert_eq!(number_length2(&1000), 4);
    }
}
