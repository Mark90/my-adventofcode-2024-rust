use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
fn part1(content: &str) -> i32 {
    0
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
}
