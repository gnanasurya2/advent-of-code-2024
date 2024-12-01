pub fn process(input: &str) -> i32 {
    let mut final_sum = 0;
    let mut locations_1: Vec<i32> = vec![];
    let mut locations_2: Vec<i32> = vec![];
    for locations in input.split("\r\n") {
        let mut loc = 1;
        for location_id in locations.split("   ") {
            if loc == 1 {
                locations_1.push(location_id.parse().unwrap());
                loc = 2;
            } else {
                locations_2.push(location_id.parse().unwrap());
            }
        }
    }
    locations_1.sort();
    locations_2.sort();

    for i in 0..locations_1.len() {
        final_sum += (locations_1[i] - locations_2[i]).abs()
    }
    final_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input1.txt");
        let result = process(input);
        assert_eq!(result, 1506483);
    }
}
