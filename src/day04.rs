use std::collections::HashMap;

use aoc_runner_derive::aoc;

#[derive(PartialEq, Clone)]
enum Letter {
    X,
    M,
    A,
    S,
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Point {
    x: i32,
    y: i32,
}

const DIRECTIONS: [Point; 8] = [
    Point { x: 0, y: -1 },
    Point { x: 1, y: -1 },
    Point { x: 1, y: 0 },
    Point { x: 1, y: 1 },
    Point { x: 0, y: 1 },
    Point { x: -1, y: 1 },
    Point { x: -1, y: 0 },
    Point { x: -1, y: -1 },
];

fn is_xmas(grid: &HashMap<Point, Letter>, point: &Point, direction: &Point) -> i32 {
    for expected_letter in [Letter::M, Letter::A, Letter::S].iter() {
        let distance = match expected_letter {
            Letter::X => 0,
            Letter::M => 1,
            Letter::A => 2,
            Letter::S => 3,
        };
        let next_point = Point {
            x: point.x + distance * direction.x,
            y: point.y + distance * direction.y,
        };

        match grid.get(&next_point) {
            Some(actual_tile) if actual_tile == expected_letter => {
                continue;
            }
            _ => return 0,
        }
    }
    return 1;
}

fn count_xmas_words(grid: &HashMap<Point, Letter>, point: &Point) -> i32 {
    DIRECTIONS
        .iter()
        .map(|direction| is_xmas(&grid, point, direction))
        .sum()
}

fn to_xmas(c: char) -> Letter {
    match c {
        'X' => Letter::X,
        'M' => Letter::M,
        'A' => Letter::A,
        'S' => Letter::S,
        _ => panic!("Invalid character '{}'", c),
    }
}

#[aoc(day4, part1)]
fn part1(content: &str) -> i32 {
    let mut grid: HashMap<Point, Letter> = HashMap::new();
    for (y, line) in content.lines().enumerate() {
        for (x, xmas) in line.chars().map(to_xmas).enumerate() {
            let point = Point {
                x: x as i32,
                y: y as i32,
            };
            grid.insert(point, xmas);
        }
    }

    grid.iter()
        .clone()
        .filter(|(_, tile)| tile == &&Letter::X)
        .map(|(point, _)| count_xmas_words(&grid, &point))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT), 18);
    }
}
