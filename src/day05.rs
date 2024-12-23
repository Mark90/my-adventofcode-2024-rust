use aoc_runner_derive::aoc;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;

fn is_valid_update(rules: &HashMap<i32, HashSet<i32>>, update: &Vec<i32>) -> bool {
    let mut remaining = HashSet::<i32>::from_iter(update.clone());

    for (position, page) in update.iter().enumerate() {
        remaining.remove(page);

        if !match rules.get(&page) {
            Some(allowed) => remaining.is_subset(allowed),
            None => position == update.len() - 1,
        } {
            return false;
        }
    }

    true
}

fn to_update(update_line: &str) -> Vec<i32> {
    update_line
        .split(",")
        .map(|c| c.parse::<i32>().unwrap())
        .collect()
}

fn to_rule(rule_line: &str) -> (i32, i32) {
    let pages: Vec<i32> = rule_line
        .split('|')
        .map(|y| y.parse::<i32>().unwrap())
        .collect();
    (pages[0], pages[1])
}

#[aoc(day5, part1)]
fn part1(content: &str) -> i32 {
    let mut parts = content.split("\n\n");
    let rules_ = parts.next().unwrap();
    let updates = parts.next().unwrap();

    let mut rules = HashMap::<i32, HashSet<i32>>::new();
    for rule_line in rules_.lines() {
        let (from, to) = to_rule(rule_line);
        rules.entry(from).or_insert(HashSet::new()).insert(to);
    }

    updates
        .lines()
        .map(to_update)
        .filter(|update| is_valid_update(&rules, update))
        .map(|update| update[update.len() / 2])
        .sum()
}

fn sort_update(rules: &HashMap<i32, HashSet<i32>>, update: &mut Vec<i32>) -> () {
    update.sort_by(|a, b| {
        if rules
            .get(a)
            .map_or(false, |next_pages| next_pages.contains(b))
        {
            Ordering::Greater
        } else if rules
            .get(b)
            .map_or(false, |next_pages| next_pages.contains(a))
        {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    });
}

#[aoc(day5, part2)]
fn part2(content: &str) -> i32 {
    let mut parts = content.split("\n\n");
    let rules_ = parts.next().unwrap();
    let updates = parts.next().unwrap();

    let mut rules = HashMap::<i32, HashSet<i32>>::new();
    for rule_line in rules_.lines() {
        let (from, to) = to_rule(rule_line);
        rules.entry(from).or_insert(HashSet::new()).insert(to);
    }

    updates
        .lines()
        .map(to_update)
        .filter(|update| !is_valid_update(&rules, update))
        .map(|mut update| {
            sort_update(&rules, &mut update);
            update[update.len() / 2]
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT), 143);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&INPUT), 123);
    }
}
