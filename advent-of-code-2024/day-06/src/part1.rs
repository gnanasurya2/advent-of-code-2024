use std::collections::HashSet;

fn is_valid(grid: &Vec<Vec<char>>, y: isize, x: isize) -> bool {
    x >= 0 && x < grid[0].len() as isize && y >= 0 && y < grid.len() as isize
}

enum Dir {
    Up,
    Down,
    Right,
    Left,
}

pub fn process(input: &str) -> i64 {
    let mut grid: Vec<Vec<char>> = vec![];
    let mut curr: (isize, isize) = (0, 0);
    let mut curr_dir = Dir::Up;

    let mut row_no = 0;
    for line in input.split("\r\n") {
        let mut row: Vec<char> = vec![];
        let mut col = 0;
        for ch in line.chars() {
            if ch == '^' {
                curr = (row_no, col);
            }
            row.push(ch);
            col += 1;
        }
        grid.push(row);
        row_no += 1;
    }

    let mut visited_set: HashSet<(isize, isize)> = HashSet::new();

    while is_valid(&grid, curr.0, curr.1) {
        if grid[curr.0 as usize][curr.1 as usize] == '#' {
            match curr_dir {
                Dir::Up => {
                    curr_dir = Dir::Right;
                    curr = (curr.0 + 1, curr.1 + 1);
                }
                Dir::Down => {
                    curr_dir = Dir::Left;
                    curr = (curr.0 - 1, curr.1 - 1);
                }
                Dir::Right => {
                    curr_dir = Dir::Down;
                    curr = (curr.0 + 1, curr.1 - 1);
                }
                Dir::Left => {
                    curr_dir = Dir::Up;
                    curr = (curr.0 - 1, curr.1 + 1);
                }
            }
        } else {
            visited_set.insert(curr);
            match curr_dir {
                Dir::Up => curr = (curr.0 - 1, curr.1),
                Dir::Down => curr = (curr.0 + 1, curr.1),
                Dir::Right => curr = (curr.0, curr.1 + 1),
                Dir::Left => curr = (curr.0, curr.1 - 1),
            }
        }
    }

    visited_set.len() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input1.txt");
        let result = process(input);
        assert_eq!(result, 4559);
    }
}
