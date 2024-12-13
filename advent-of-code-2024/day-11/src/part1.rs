pub fn process(input: &str) -> i64 {
    let mut stones = input
        .split(" ")
        .map(|st| st.to_string())
        .collect::<Vec<String>>();

    for _ in 0..25 {
        let mut temp: Vec<String> = vec![];

        for i in 0..stones.len() {
            if stones[i] == "0" {
                temp.push("1".to_string());
            } else if stones[i].len() % 2 == 0 {
                let mut found_non_zero = false;
                let first_part: String = stones[i]
                    .chars()
                    .skip(0)
                    .take(stones[i].len() / 2)
                    .filter(|ch| {
                        if found_non_zero {
                            return true;
                        }
                        if !found_non_zero && ch == &'0' {
                            return false;
                        }
                        found_non_zero = true;
                        return true;
                    })
                    .collect();
                if first_part.len() > 0 {
                    temp.push(first_part);
                } else {
                    temp.push("0".to_string());
                }

                found_non_zero = false;
                let second_part: String = stones[i]
                    .chars()
                    .skip(stones[i].len() / 2)
                    .take(stones[i].len() / 2)
                    .filter(|ch| {
                        if found_non_zero {
                            return true;
                        }
                        if !found_non_zero && ch == &'0' {
                            return false;
                        }
                        found_non_zero = true;
                        return true;
                    })
                    .collect();
                if second_part.len() > 0 {
                    temp.push(second_part);
                } else {
                    temp.push("0".to_string());
                }
            } else {
                temp.push((stones[i].parse::<u64>().unwrap() * 2024).to_string());
            }
        }
        stones = temp;
    }
    stones.len() as i64
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
