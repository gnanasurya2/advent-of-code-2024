use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use itertools::Itertools;

pub fn process(input: &str) -> i64 {
    let mut res = 0;

    let mut before_set: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut is_rows = false;

    let mut rows: Vec<Vec<&str>> = vec![];

    fn is_good(row: &Vec<&str>, before_set: &HashMap<&str, HashSet<&str>>) -> bool {
        for i in 0..row.len() {
            for j in i + 1..row.len() {
                if let Some(set) = before_set.get(row[i]) {
                    if set.contains(row[j]) {
                        return false;
                    }
                }
            }
        }
        return true;
    }

    for line in input.split("\r\n") {
        if line.len() == 0 {
            is_rows = true;
        } else if !is_rows {
            let nums = line.split("|").collect::<Vec<&str>>();
            before_set
                .entry(nums[1])
                .or_insert(HashSet::new())
                .insert(nums[0]);
        } else {
            rows.push(line.split(",").collect_vec());
        }
    }

    for row in rows {
        if !is_good(&row, &before_set) {
            let mut sorted_row = row.clone();
            sorted_row.sort_by(|a, b| {
                if let Some(set) = before_set.get(b) {
                    if set.contains(a) {
                        return Ordering::Less;
                    }
                }

                if let Some(set) = before_set.get(a) {
                    if set.contains(b) {
                        return Ordering::Greater;
                    }
                }

                return Ordering::Equal;
            });

            res += sorted_row[sorted_row.len() / 2].parse::<i64>().unwrap()
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
        assert_eq!(result, 71668682);
    }
}
