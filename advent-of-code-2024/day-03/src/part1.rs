use regex::Regex;

pub fn process(input: &str) -> i64 {
    let mut res: i64 = 0;
    let re = Regex::new(r"mul\((?P<num1>\d{1,3}),(?P<num2>\d{1,3})\)").unwrap();

    for (_, [num1, num2]) in re.captures_iter(input).map(|c| c.extract()) {
        res += num1.parse::<i64>().unwrap() * num2.parse::<i64>().unwrap()
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
        assert_eq!(result, 175700056);
    }
}
