use std::collections::{HashMap, HashSet};

use itertools::Itertools;
fn is_valid(x: i32, y: i32, max_row: i32, max_col: i32) -> bool {
    x >= 0 && x < max_row && y >= 0 && y < max_col
}

fn find_mirror_point(mirror: &(i32, i32), point: &(i32, i32)) -> Vec<(i32, i32)> {
    vec![
        (mirror.0 * 2 - point.0, mirror.1 * 2 - point.1),
        (point.0 * 2 - mirror.0, point.1 * 2 - mirror.1),
    ]
}

pub fn process(input: &str) -> i64 {
    let mut curr_row = 0;
    let mut max_col = 0;

    let mut antenna_map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for line in input.split("\r\n") {
        let mut curr_col = 0;
        for char in line.chars() {
            if char != '.' {
                antenna_map
                    .entry(char)
                    .and_modify(|locs| locs.push((curr_row, curr_col)))
                    .or_insert(vec![(curr_row, curr_col)]);
            }
            curr_col += 1;
        }
        max_col = curr_col;
        curr_row += 1;
    }

    let mut res_set: HashSet<(i32, i32)> = HashSet::new();
    antenna_map.values().for_each(|code| {
        code.iter().combinations(2).for_each(|points| {
            find_mirror_point(points[0], points[1])
                .iter()
                .for_each(|point| {
                    if is_valid(point.0, point.1, curr_row, max_col) {
                        res_set.insert(*point);
                    }
                });
        });
    });

    res_set.len() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input1.txt");
        let result = process(input);
        assert_eq!(result, 390);
    }
}
