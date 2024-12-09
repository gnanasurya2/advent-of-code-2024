fn find_first_empty_space(disk: &Vec<i64>, curr: usize) -> usize {
    let mut res = curr + 1;

    while res < disk.len() && disk[res] != -1 {
        res += 1;
    }

    return res;
}

fn find_next_digit(disk: &Vec<i64>, curr: usize) -> usize {
    let mut res = curr - 1;

    while res >= 0 && disk[res] == -1 {
        res -= 1;
    }

    return res;
}

pub fn process(input: &str) -> i64 {
    let mut disk: Vec<i64> = vec![];
    let mut curr_id = 0;
    input.chars().enumerate().for_each(|(idx, ch)| {
        if idx % 2 == 0 {
            for _ in 0..ch.to_digit(10).unwrap() {
                disk.push(curr_id);
            }
            curr_id += 1;
        } else {
            for _ in 0..ch.to_digit(10).unwrap() {
                disk.push(-1);
            }
        }
    });

    let mut curr_space = find_first_empty_space(&disk, 0);
    let mut curr_digit = find_next_digit(&disk, disk.len());

    while curr_space < curr_digit {
        disk.swap(curr_space, curr_digit);
        curr_space = find_first_empty_space(&disk, curr_space);
        curr_digit = find_next_digit(&disk, curr_digit);
    }

    let mut res: i64 = 0;

    for i in 0..disk.len() {
        if disk[i] != -1 {
            res += i as i64 * disk[i];
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
        assert_eq!(result, 6241633730082);
    }
}
