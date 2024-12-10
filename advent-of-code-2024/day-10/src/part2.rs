fn is_valid(x: i32, y: i32, max_row: usize, max_col: usize) -> bool {
    x >= 0 && x < max_row as i32 && y >= 0 && y < max_col as i32
}

fn find_valid_trail(i: i32, j: i32, curr: u32, grid: &Vec<Vec<u32>>) -> i64 {
    if is_valid(i, j, grid.len(), grid[0].len()) && curr == 9 && grid[i as usize][j as usize] == 9 {
        return 1;
    }

    if !is_valid(i, j, grid.len(), grid[0].len()) || grid[i as usize][j as usize] != curr {
        return 0;
    } else {
        [(-1, 0), (0, 1), (1, 0), (0, -1)]
            .iter()
            .map(|dir| find_valid_trail(i + dir.0, j + dir.1, curr + 1, grid))
            .reduce(|acc, ele| acc + ele)
            .unwrap()
    }
}

pub fn process(input: &str) -> i64 {
    let mut res: i64 = 0;
    let mut grid: Vec<Vec<u32>> = vec![];

    for line in input.split("\r\n") {
        grid.push(line.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
    }

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 0 {
                res += find_valid_trail(i as i32, j as i32, 0, &grid);
                println!("res {:?} {:?}", res, (i, j));
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input1.txt");
        let result = process(input);
        assert_eq!(result, 1289);
    }
}
