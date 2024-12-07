use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, u64},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

fn num_of_digits(num: u64) -> u32 {
    let mut num = num;
    let mut res = 0;
    while num > 0 {
        num /= 10;
        res += 1;
    }
    res
}

fn can_reach_target(curr: u64, target: u64, nums: &Vec<u64>, idx: usize) -> bool {
    if idx == nums.len() {
        return target == curr;
    }

    return can_reach_target(curr + nums[idx], target, nums, idx + 1)
        || can_reach_target(curr * nums[idx], target, nums, idx + 1)
        || can_reach_target(
            curr * 10_u64.pow(num_of_digits(nums[idx])) + nums[idx],
            target,
            nums,
            idx + 1,
        );
}

fn parse_input(input: &str) -> IResult<&str, Vec<(u64, Vec<u64>)>> {
    separated_list1(
        line_ending,
        separated_pair(u64, tag(": "), separated_list1(tag(" "), u64)),
    )(input)
}

pub fn process(input: &str) -> i64 {
    let (_, lines) = parse_input(input).unwrap();

    lines
        .iter()
        .filter(|ele| can_reach_target(ele.1[0], ele.0, &ele.1, 1))
        .fold(0, |acc, ele| acc + ele.0) as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input1.txt");
        let result = process(input);
        assert_eq!(result, 159490400628354);
    }
}
