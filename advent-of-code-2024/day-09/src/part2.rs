use std::vec;

#[derive(Debug)]
struct DiskType {
    start_idx: usize,
    size: i64,
}

fn find_first_empty_space(disk: &Vec<DiskType>, target: &DiskType) -> Option<usize> {
    for i in 0..disk.len() {
        if disk[i].size >= target.size && disk[i].start_idx < target.start_idx {
            return Some(i);
        }
    }
    None
}

pub fn process(input: &str) -> i64 {
    let mut disk: Vec<i64> = vec![];
    let mut curr_id = 0;
    let mut disk_types: Vec<DiskType> = vec![];
    let mut empty_types: Vec<DiskType> = vec![];

    input.chars().enumerate().for_each(|(idx, ch)| {
        let val = ch.to_digit(10).unwrap();
        if idx % 2 == 0 {
            disk_types.push(DiskType {
                start_idx: disk.len(),
                size: val as i64,
            });
            for _ in 0..val {
                disk.push(curr_id as i64);
            }
            curr_id += 1;
        } else {
            empty_types.push(DiskType {
                start_idx: disk.len(),
                size: val as i64,
            });
            for _ in 0..val {
                disk.push(-1);
            }
        }
    });

    for idx in (0..disk_types.len()).rev() {
        if let Some(curr_idx) = find_first_empty_space(&empty_types, &disk_types[idx]) {
            for i in 0..disk_types[idx as usize].size {
                disk.swap(
                    empty_types[curr_idx].start_idx + i as usize,
                    disk_types[idx as usize].start_idx + i as usize,
                );
            }

            empty_types[curr_idx].start_idx += disk_types[idx].size as usize;
            empty_types[curr_idx].size -= disk_types[idx].size;
        }
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
        assert_eq!(result, 6265268809555);
    }
}
