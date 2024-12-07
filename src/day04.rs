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

impl Point {
    fn advance(&self, direction: &Point, distance: i32) -> Point {
        Point {
            x: self.x + distance * direction.x,
            y: self.y + distance * direction.y,
        }
    }
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
        let next_point = match expected_letter {
            Letter::X => point.advance(direction, 0),
            Letter::M => point.advance(direction, 1),
            Letter::A => point.advance(direction, 2),
            Letter::S => point.advance(direction, 3),
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

fn parse_letter(c: char) -> Letter {
    match c {
        'X' => Letter::X,
        'M' => Letter::M,
        'A' => Letter::A,
        'S' => Letter::S,
        _ => panic!("Invalid character '{}'", c),
    }
}

fn create_grid(content: &str) -> HashMap<Point, Letter> {
    let mut grid: HashMap<Point, Letter> = HashMap::new();
    for (y, line) in content.lines().enumerate() {
        for (x, xmas) in line.chars().map(parse_letter).enumerate() {
            let point = Point {
                x: x as i32,
                y: y as i32,
            };
            grid.insert(point, xmas);
        }
    }
    grid
}

#[aoc(day4, part1)]
fn part1(content: &str) -> i32 {
    let grid = create_grid(content);

    grid.iter()
        .clone()
        .filter(|(_, tile)| tile == &&Letter::X)
        .map(|(point, _)| count_xmas_words(&grid, &point))
        .sum()
}

fn count_x_mas_words(grid: &HashMap<Point, Letter>, point: &Point) -> i32 {
    let nw = grid.get(&point.advance(&Point { x: -1, y: -1 }, 1));
    let ne = grid.get(&point.advance(&Point { x: 1, y: -1 }, 1));
    let se = grid.get(&point.advance(&Point { x: 1, y: 1 }, 1));
    let sw = grid.get(&point.advance(&Point { x: -1, y: 1 }, 1));

    match [nw, sw, ne, se] {
        [Some(Letter::M), Some(Letter::M), Some(Letter::S), Some(Letter::S)] => 1,
        [Some(Letter::M), Some(Letter::S), Some(Letter::M), Some(Letter::S)] => 1,
        [Some(Letter::S), Some(Letter::S), Some(Letter::M), Some(Letter::M)] => 1,
        [Some(Letter::S), Some(Letter::M), Some(Letter::S), Some(Letter::M)] => 1,
        _ => 0,
    }
}

#[aoc(day4, part2)]
fn part2(content: &str) -> i32 {
    let grid = create_grid(content);

    grid.iter()
        .clone()
        .filter(|(_, tile)| tile == &&Letter::A)
        .map(|(point, _)| count_x_mas_words(&grid, &point))
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

    #[test]
    fn test_part2() {
        assert_eq!(part2(&INPUT), 9);
    }
}
