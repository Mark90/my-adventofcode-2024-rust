use aoc_runner_derive::aoc;

use regex::Regex;

fn find_and_sum_muls(re: &Regex, memory: &str) -> i32 {
    let mut result = 0;
    for (_, [number1, number2]) in re.captures_iter(memory).map(|c| c.extract()) {
        let number1 = number1.parse::<i32>().unwrap();
        let number2 = number2.parse::<i32>().unwrap();
        result += number1 * number2;
    }
    result
}

#[aoc(day3, part1)]
fn part1(content: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    find_and_sum_muls(&re, content)
}

#[aoc(day3, part2)]
fn part2(content: &str) -> i32 {
    let re_enabled = Regex::new(r"(?s)do\(\)(.*?)don't\(\)").unwrap();
    let re_mul: Regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    // Wrap content to make finding enabled sections easier
    let content_padded_ = format!("{}{}{}", "do()", content, "don't()");
    let content_padded = content_padded_.as_str();

    re_enabled
        .captures_iter(content_padded)
        .map(|s| s.extract())
        .map(|(_, [enabled_memory])| find_and_sum_muls(&re_mul, enabled_memory))
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT1: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const INPUT2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    const INPUT3: &str = "mul(1,1)don't()do()mul(2,2)do()mul(3,3)don't()mul(4,4)";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(&INPUT1), 161);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(&INPUT2), 48);
        assert_eq!(part2(&INPUT3), 14);
    }
}
