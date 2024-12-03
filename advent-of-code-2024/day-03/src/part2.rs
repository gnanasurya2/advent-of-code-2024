use regex::Regex;

pub fn process(input: &str) -> i64 {
    let mut res: i64 = 0;
    let re = Regex::new(r"mul\((?P<num1>\d{1,3}),(?P<num2>\d{1,3})\)|don\'t\(\)|do\(\)").unwrap();

    let mut is_valid = true;
    for captures in re.captures_iter(input) {
        let val = captures.get(0).unwrap().as_str();
        if val == "do()" {
            is_valid = true
        } else if val == "don't()" {
            is_valid = false
        } else if is_valid {
            res += captures.get(1).unwrap().as_str().parse::<i64>().unwrap()
                * captures.get(2).unwrap().as_str().parse::<i64>().unwrap();
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
