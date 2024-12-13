use std::collections::HashSet;

use aoc_runner_derive::aoc;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
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

const DIRECTIONS: [Point; 4] = [
    Point { x: 0, y: -1 }, // North
    Point { x: 1, y: 0 },  // East
    Point { x: 0, y: 1 },  // South
    Point { x: -1, y: 0 }, // West
];

fn out_of_bounds(next: &Point, width: i32, height: i32) -> bool {
    next.x < 0 || next.y < 0 || next.x == width || next.y == height
}

#[aoc(day6, part1)]
fn part1(content: &str) -> i32 {
    let mut obstructions = HashSet::<Point>::new();
    let mut start: Option<Point> = None;

    let (mut max_x, mut max_y) = (0, 0);
    for (y, row) in content.lines().enumerate() {
        max_y = max_y.max(y);
        for (x, cell) in row.chars().enumerate() {
            max_x = max_x.max(x);
            match cell {
                '^' => {
                    start = Some(Point {
                        x: x as i32,
                        y: y as i32,
                    });
                }
                '#' => {
                    obstructions.insert(Point {
                        x: x as i32,
                        y: y as i32,
                    });
                }
                _ => (),
            };
        }
    }

    let mut visited = HashSet::<Point>::new();
    let mut direction = 0;
    let mut current = start.unwrap();
    visited.insert(current.clone());

    let height = max_y as i32 + 1;
    let width = max_x as i32 + 1;
    loop {
        let next = current.advance(&DIRECTIONS[direction], 1);
        if obstructions.contains(&next) {
            direction = (direction + 1) % 4;
            continue;
        }
        if out_of_bounds(&next, width, height) {
            break;
        }
        current = next.clone();
        visited.insert(next);
    }

    visited.len() as i32
}

fn enters_a_loop(
    candidate: &Point,
    obstructions1: &HashSet<Point>,
    start: &Point,
    height: i32,
    width: i32,
) -> bool {
    let mut direction: usize = 0;
    let mut current: Point = start.clone();
    let mut visited: HashSet<(Point, usize)> = HashSet::from([(start.clone(), direction)]);

    let mut obstructions: HashSet<Point> = obstructions1.clone();
    obstructions.insert(candidate.clone());

    loop {
        let next = current.advance(&DIRECTIONS[direction], 1);
        if obstructions.contains(&next) {
            direction = (direction + 1) % 4;
            continue;
        }

        if visited.contains(&(next.clone(), direction)) {
            return true;
        }

        if out_of_bounds(&next, width, height) {
            return false;
        }

        current = next.clone();
        visited.insert((next, direction));
    }
}

#[aoc(day6, part2)]
fn part2(content: &str) -> i32 {
    let mut obstructions = HashSet::<Point>::new();
    let mut candidates = HashSet::<Point>::new();
    let mut start: Option<Point> = None;

    let (mut max_x, mut max_y) = (0, 0);
    for (y, row) in content.lines().enumerate() {
        max_y = max_y.max(y);
        for (x, cell) in row.chars().enumerate() {
            max_x = max_x.max(x);

            let point = Point {
                x: x as i32,
                y: y as i32,
            };
            match cell {
                '^' => {
                    start = Some(point);
                }
                '#' => {
                    obstructions.insert(point);
                }
                _ => {
                    candidates.insert(point);
                }
            };
        }
    }
    let height = max_y as i32 + 1;
    let width = max_x as i32 + 1;

    let mut start1 = start.unwrap();

    candidates
        .iter()
        .filter(|c| enters_a_loop(*c, &obstructions, &start1, height, width))
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT), 41);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&INPUT), 6);
    }
}
