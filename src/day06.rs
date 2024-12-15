use aoc_runner_derive::aoc;

const DIRECTIONS: [(i32, i32); 4] = [
    (0, -1), // North
    (1, 0),  // East
    (0, 1),  // South
    (-1, 0), // West
];

fn out_of_bounds(coord: &(i32, i32), width: i32, height: i32) -> bool {
    coord.0 < 0 || coord.1 < 0 || coord.0 == width || coord.1 == height
}

fn move_in_direction(coord: (i32, i32), direction: usize) -> (i32, i32) {
    let d = DIRECTIONS[direction];
    (coord.0 + d.0 as i32, coord.1 + d.1 as i32)
}

fn get_grid(grid: &Vec<Vec<char>>, coord: (i32, i32)) -> char {
    grid[coord.1 as usize][coord.0 as usize]
}

fn set_grid(grid: &mut Vec<Vec<char>>, coord: (i32, i32), new_char: char) -> () {
    grid[coord.1 as usize][coord.0 as usize] = new_char
}

fn get_guard(direction: usize) -> char {
    match direction {
        0 => '^',
        1 => '>',
        2 => 'v',
        3 => '<',
        _ => panic!("non"),
    }
}

fn set_guard(grid: &mut Vec<Vec<char>>, coord: (i32, i32), direction: usize) -> () {
    set_grid(grid, coord, get_guard(direction));
}

#[aoc(day6, part1)]
fn part1(content: &str) -> i32 {
    let mut grid: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();

    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    let start = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .position(|&c| c == '^')
                .map(|x| (x as i32, y as i32))
        })
        .unwrap();

    let mut current: (i32, i32) = start;
    let mut next: (i32, i32);
    let mut direction = 0;

    let mut visited = 1;
    loop {
        next = move_in_direction(current, direction);
        if out_of_bounds(&next, width, height) {
            break;
        }
        let field = get_grid(&grid, next);
        if field == '#' {
            direction = (direction + 1) % 4;
            set_guard(&mut grid, current, direction);
            continue;
        }
        if field == '.' {
            visited += 1;
        }
        set_grid(&mut grid, current, 'X');
        set_guard(&mut grid, next, direction);
        current = next;
    }

    visited
}

fn _print_grid(g: &Vec<Vec<char>>) {
    for row in g.iter() {
        println!("{}", row.iter().collect::<String>());
    }
}

fn walk_until_loop(grid: &mut Vec<Vec<char>>, start: (i32, i32), height: i32, width: i32) -> i32 {
    let mut current: (i32, i32) = start;
    let mut next: (i32, i32);
    let mut direction = 0;
    let mut guard = get_guard(direction);
    let mut steps_taken = 0;

    loop {
        next = move_in_direction(current, direction);
        if out_of_bounds(&next, width, height) {
            return 0;
        }
        let field = get_grid(&grid, next);
        if "O#".contains(field) {
            direction = (direction + 1) % 4;
            guard = get_guard(direction);
            set_grid(grid, current, guard);
            continue;
        }
        steps_taken += 1;

        // We are looping if either:
        // - the guard passed next field in same direction (probably does not capture all cases)
        // - number of steps exceeds the grid size (lazy, but only happens 26 times on real input so eh)
        if field == guard || steps_taken > (height * width) {
            return 1;
        }
        set_grid(grid, next, guard);
        current = next;
    }
}

#[aoc(day6, part2)]
fn part2(content: &str) -> i32 {
    let orig_grid: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();

    let height = orig_grid.len() as i32;
    let width = orig_grid[0].len() as i32;

    let start = orig_grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .position(|&c| c == '^')
                .map(|x| (x as i32, y as i32))
        })
        .unwrap();

    let mut loops_found = 0;
    for (y, row) in orig_grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if "#^".contains(*c) {
                continue;
            }
            let mut grid = orig_grid.clone();
            grid[y][x] = 'O';

            loops_found += walk_until_loop(&mut grid, start, height, width);
        }
    }
    loops_found
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
