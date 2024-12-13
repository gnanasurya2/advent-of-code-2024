use std::collections::HashSet;

fn is_valid(i: isize, j: isize, max_r: isize, max_c: isize) -> bool {
    i >= 0 && i < max_r && j >= 0 && j < max_c
}

fn find_area_and_perimeter(
    i: isize,
    j: isize,
    grid: &mut Vec<Vec<char>>,
    visited_set: &mut HashSet<(usize, usize)>,
) -> i64 {
    if !is_valid(i, j, grid.len() as isize, grid[0].len() as isize)
        || visited_set.contains(&(i as usize, j as usize))
    {
        return 0;
    }

    visited_set.insert((i as usize, j as usize));
    let temp = grid[i as usize][j as usize];
    // grid[i as usize][j as usize] = '0';

    let valid_neighbours = [(-1, 0), (0, 1), (1, 0), (0, -1)]
        .iter()
        .map(|ele| (ele.0 + i, ele.1 + j))
        .filter(|ele| {
            is_valid(ele.0, ele.1, grid.len() as isize, grid[0].len() as isize)
                && temp == grid[ele.0 as usize][ele.1 as usize]
        })
        .collect::<Vec<(isize, isize)>>();

    if valid_neighbours.len() == 0 {
        return 4;
    }

    4 - valid_neighbours.len() as i64
        + valid_neighbours
            .iter()
            .map(|ele| find_area_and_perimeter(ele.0, ele.1, grid, visited_set))
            .reduce(|acc, ele| acc + ele)
            .unwrap()
}

pub fn process(input: &str) -> i64 {
    let mut res = 0;
    let mut grid: Vec<Vec<char>> = vec![];

    for line in input.split("\r\n") {
        grid.push(line.chars().collect());
    }

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] != '0' {
                let mut visited_set: HashSet<(usize, usize)> = HashSet::new();
                let value =
                    find_area_and_perimeter(i as isize, j as isize, &mut grid, &mut visited_set);

                // println!("{:?} {:?} {:?}", temp, visited_set.len(), value);

                res += visited_set.len() as i64 * value;
                for item in visited_set {
                    grid[item.0][item.1] = '0';
                }
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
        assert_eq!(result, 220722);
    }
}
