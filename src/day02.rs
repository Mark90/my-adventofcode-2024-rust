use aoc_runner_derive::aoc;

fn is_curve(delta1: i32, delta2: i32) -> bool {
    delta1 * delta2 < 0
}

fn is_unsafe_increase(increase: i32) -> bool {
    increase > 3 || increase < 1
}

fn to_report(report: &str) -> Vec<i32> {
    report
        .split_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect()
}

fn is_safe_report(report: &Vec<i32>) -> bool {
    let mut levels = report.iter();

    let mut prev_level = levels.next().unwrap();
    let mut curr_level = levels.next().unwrap();
    let mut prev_delta;
    let mut curr_delta = curr_level - prev_level;

    if is_unsafe_increase(curr_delta.abs()) {
        return false;
    }

    for level in levels {
        prev_delta = curr_delta;

        (prev_level, curr_level) = (curr_level, level);
        curr_delta = curr_level - prev_level;
        if is_unsafe_increase(curr_delta.abs()) || is_curve(curr_delta, prev_delta) {
            return false;
        }
    }

    return true;
}

#[aoc(day2, part1)]
fn part1(content: &str) -> i32 {
    content
        .lines()
        .map(to_report)
        .map(|r| is_safe_report(&r))
        .filter(|v| *v)
        .count() as i32
}

fn is_safeish_report(report: Vec<i32>) -> bool {
    for skip_level in 0..report.len() {
        let subset: Vec<i32> = report
            .iter()
            .enumerate()
            .filter(|(level, _v)| level != &skip_level)
            .map(|(_i, v)| v)
            .cloned()
            .collect();
        if is_safe_report(&subset) {
            return true;
        }
    }
    return false;
}

#[aoc(day2, part2)]
fn part2(content: &str) -> i32 {
    content
        .lines()
        .map(to_report)
        .map(is_safeish_report)
        .filter(|v| *v)
        .count() as i32
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = "7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(&INPUT), 2);
    }
    #[test]
    fn test_part_2() {
        assert_eq!(part2(&INPUT), 4);
    }
}
