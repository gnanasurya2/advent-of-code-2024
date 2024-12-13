use std::collections::HashMap;

pub fn iterate(stone: &str, blinks: usize, map: &mut HashMap<String, i64>) -> i64 {
    let cache_key = format!("{stone}-{blinks}");

    if map.contains_key(&cache_key) {
        return map.get(&cache_key).unwrap().to_owned();
    } else {
        let count = match blinks {
            75 => 1i64,
            _ => {
                if stone == "0" || stone == "" {
                    iterate("1", blinks + 1, map)
                } else {
                    match stone.len() & 1 {
                        0 => {
                            iterate(&stone[0..stone.len() / 2], blinks + 1, map)
                                + iterate(
                                    &stone[stone.len() / 2..].trim_start_matches("0"),
                                    blinks + 1,
                                    map,
                                )
                        }
                        _ => iterate(
                            &format!("{}", stone.parse::<u64>().unwrap() * 2024).as_str(),
                            blinks + 1,
                            map,
                        ),
                    }
                }
            }
        };
        map.insert(cache_key, count);
        count
    }
}

pub fn process(input: &str) -> i64 {
    let mut cache: HashMap<String, i64> = HashMap::new();

    input
        .split_whitespace()
        .map(|stone| iterate(stone, 0, &mut cache))
        .sum()
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
