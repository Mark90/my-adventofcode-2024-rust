use aoc_runner_derive::aoc;

const DIRECTIONS: [(i32, i32); 4] = [
    (0, -1), // North
    (1, 0),  // East
    (0, 1),  // South
    (-1, 0), // West
];

fn is_equal<T: PartialEq>(grid: &Vec<Vec<T>>, node: &(i32, i32), value: &T) -> bool {
    grid.get(node.1 as usize)
        .and_then(|row| row.get(node.0 as usize))
        .map_or(false, |v| v == value)
}

fn valid_next_node(
    grid: &Vec<Vec<u8>>,
    seen: &Vec<Vec<bool>>,
    node: &(i32, i32),
    level: &u8,
) -> bool {
    !is_equal(seen, node, &true) && is_equal(grid, node, &(level + 1))
}

fn get_level<'a>(grid: &'a Vec<Vec<u8>>, node: &'a (i32, i32)) -> Option<&'a u8> {
    grid.get(node.1 as usize)
        .map_or(None, |row| row.get(node.0 as usize))
}

fn get_next_node(node: &(i32, i32), direction: &(i32, i32)) -> (i32, i32) {
    (node.0 + direction.0, node.1 + direction.1)
}

fn count_tops(
    grid: &Vec<Vec<u8>>,
    seen: &mut Vec<Vec<bool>>,
    node: &(i32, i32),
    duplicate_paths: bool,
) -> u32 {
    seen[node.1 as usize][node.0 as usize] = true ^ duplicate_paths;

    // Check if we found a top, if so return score 1. At this point we know the node exists
    // so .unwrap is fine
    let level = get_level(grid, node).unwrap();
    if level == &9 {
        return 1;
    }

    // Return total score of all valid neighbors
    DIRECTIONS
        .iter()
        .filter_map(|direction| {
            let next_node = get_next_node(node, &direction);
            if valid_next_node(&grid, seen, &next_node, level) {
                Some(count_tops(grid, seen, &next_node, duplicate_paths))
            } else {
                None
            }
        })
        .sum()
}

fn get_scores(grid: &mut Vec<Vec<u8>>, duplicate_paths: bool) -> u32 {
    let heads: Vec<(i32, i32)> = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, value)| {
                if value == &0 {
                    Some((x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .collect();

    let height = grid.len();
    let width = grid[0].len();

    let mut score: u32 = 0;
    let mut seen = vec![vec![false; width]; height];
    // For each starting head recurse through the grid until a top is reached.
    // If duplicate_paths=true (part1) then only every unique top is included in the score.
    // If duplicate_paths=false (part2) then each top can be returned for each distinct trail.
    for start_node in heads.iter() {
        score += count_tops(grid, &mut seen, start_node, duplicate_paths);
        if !duplicate_paths {
            seen = vec![vec![false; width]; height];
        }
    }

    score
}

fn make_grid(content: &str) -> Vec<Vec<u8>> {
    let mut grid = Vec::<Vec<u8>>::new();

    for line in content.lines() {
        grid.push(
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>(),
        );
    }

    grid
}

#[aoc(day10, part1)]
fn part1(content: &str) -> u32 {
    let mut grid = make_grid(content);

    get_scores(&mut grid, false)
}

#[aoc(day10, part2)]
fn part2(content: &str) -> u32 {
    let mut grid = make_grid(content);

    get_scores(&mut grid, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&INPUT), 36);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&INPUT), 81);
    }
}
