use std::collections::HashMap;

pub fn process(input: &str) -> i32 {
    let mut similarity_score = 0;
    let mut locations_1: Vec<i32> = vec![];
    let mut frequency_map: HashMap<i32, i32> = HashMap::new();
    for locations in input.split("\r\n") {
        let mut loc = 1;
        for location_id in locations.split("   ") {
            if loc == 1 {
                locations_1.push(location_id.parse().unwrap());
                loc = 2;
            } else {
                let value: i32 = location_id.parse().unwrap();
                match frequency_map.get(&value) {
                    Some(frequency) => frequency_map.insert(value, frequency + 1),
                    None => frequency_map.insert(value, 1),
                };
            }
        }
    }

    for num in locations_1 {
        match frequency_map.get(&num) {
            Some(frequency) => similarity_score += num * frequency,
            None => (),
        };
    }

    similarity_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input1.txt");
        let result = process(input);
        assert_eq!(result, 23126924);
    }
}
