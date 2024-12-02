use itertools::Itertools;

fn is_safe_report(report: Vec<i32>) -> bool {
    let report = report.into_iter();
    (report.clone().tuple_windows().all(|(l, r)| l < r)
        || report.clone().tuple_windows().all(|(l, r)| l > r))
        && report
            .clone()
            .tuple_windows()
            .all(|(l, r)| (l - r).abs() <= 3)
}

pub fn process(input: &str) -> i32 {
    input
        .split("\r\n")
        .map(|report| {
            report
                .split(" ")
                .map(|ele| ele.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|l| {
            let rep = l.to_vec();
            let rep_len = rep.len();
            if !is_safe_report(rep.clone()) {
                for i in 0..rep_len {
                    let mut new_report = rep.clone();
                    new_report.remove(i);
                    if is_safe_report(new_report) {
                        return true;
                    }
                }
                return false;
            }
            true
        })
        .count() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input1.txt");
        let result = process(input);
        assert_eq!(result, 337);
    }
}
