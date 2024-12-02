fn is_valid(num1: i32, num2: i32, is_decreasing: bool) -> bool {
    let diff = num1 - num2;

    if diff < 0 && !is_decreasing || diff > 0 && is_decreasing {
        return false;
    }
    let abs_val = diff.abs();

    if abs_val == 0 || abs_val > 3 {
        return false;
    }
    true
}

pub fn process(input: &str) -> i32 {
    let mut safe_reports = 0;

    for report in input.split("\r\n") {
        let data: Vec<i32> = report.split(" ").map(|ele| ele.parse().unwrap()).collect();

        let mut is_decreasing = true;
        if data[1] > data[0] {
            is_decreasing = false;
        }
        let mut is_safe = true;
        for idx in 1..data.len() {
            if !is_valid(data[idx], data[idx - 1], is_decreasing) {
                is_safe = false;
                break;
            }
        }
        if is_safe {
            safe_reports += 1;
        }
    }
    safe_reports
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input1.txt");
        let result = process(input);
        assert_eq!(result, 269);
    }
}
