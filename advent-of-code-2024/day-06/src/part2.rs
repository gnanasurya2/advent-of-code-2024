use std::collections::HashSet;

pub fn process(input: &str) -> i64 {
    let mut result: i32 = 0;
    let mut curr_x = 0;
    let mut curr_y = 0;
    let mut curr_dir = 0; // [0,1,2,3] = [up,right,down,left]
    let mut map: Vec<Vec<char>> = Vec::new();
    for (line_index, line) in input.lines().enumerate() {
        let chars: Vec<char> = line.chars().collect();
        if let Some(index) = chars.iter().position(|&ch| ch == '^') {
            curr_x = index;
            curr_y = line_index;
        }
        map.push(chars);
    }

    let start_x = curr_x;
    let stary_y = curr_y;
    let height = map.len();
    let width = if height > 0 { map[0].len() } else { 0 };

    let mut history_position: HashSet<(usize, usize)> = HashSet::new();
    while !move_normal(
        &mut curr_x,
        &mut curr_y,
        curr_dir,
        width,
        height,
        &mut map,
        &mut history_position,
    ) {
        curr_dir = (curr_dir + 1) % 4;
    }

    let mut history_full: HashSet<(usize, usize, i32)> = HashSet::new();
    for (x, y) in &history_position {
        if start_x == *x && stary_y == *y {
            continue;
        }

        map[*y][*x] = '#';
        curr_dir = 0;
        curr_x = start_x;
        curr_y = stary_y;
        history_full.clear();

        loop {
            let (a, b) = move_loop(
                &mut curr_x,
                &mut curr_y,
                curr_dir,
                width,
                height,
                &mut map,
                &mut history_full,
            );

            if b {
                result += 1;
                break; // loop detected
            }

            if a {
                break; // exited
            }
            curr_dir = (curr_dir + 1) % 4;
        }

        map[*y][*x] = '.';
    }

    result as i64
}

fn move_normal(
    curr_x: &mut usize,
    curr_y: &mut usize,
    curr_dir: i32,
    width: usize,
    height: usize,
    map: &mut Vec<Vec<char>>,
    history_position: &mut HashSet<(usize, usize)>,
) -> bool {
    if curr_dir == 0 {
        for y in (0..=*curr_y).rev() {
            if map[y][*curr_x] == '#' {
                break;
            }
            *curr_y = y;

            history_position.insert((*curr_x, *curr_y));
        }

        return *curr_y == 0;
    } else if curr_dir == 1 {
        for x in *curr_x..width {
            if map[*curr_y][x] == '#' {
                break;
            }
            *curr_x = x;

            history_position.insert((*curr_x, *curr_y));
        }

        return *curr_x == width - 1;
    } else if curr_dir == 2 {
        for y in *curr_y..height {
            if map[y][*curr_x] == '#' {
                break;
            }
            *curr_y = y;

            history_position.insert((*curr_x, *curr_y));
        }

        return *curr_y == height - 1;
    } else if curr_dir == 3 {
        for x in (0..=*curr_x).rev() {
            if map[*curr_y][x] == '#' {
                break;
            }
            *curr_x = x;

            history_position.insert((*curr_x, *curr_y));
        }

        return *curr_x == 0;
    }

    return true;
}

fn move_loop(
    curr_x: &mut usize,
    curr_y: &mut usize,
    curr_dir: i32,
    width: usize,
    height: usize,
    map: &mut Vec<Vec<char>>,
    history_full: &mut HashSet<(usize, usize, i32)>,
) -> (bool, bool) {
    if curr_dir == 0 {
        for y in (0..=*curr_y).rev() {
            if map[y][*curr_x] == '#' {
                break;
            }
            *curr_y = y;

            if !history_full.insert((*curr_x, *curr_y, curr_dir)) {
                return (*curr_y == 0, true);
            }
        }

        return (*curr_y == 0, false);
    } else if curr_dir == 1 {
        for x in *curr_x..width {
            if map[*curr_y][x] == '#' {
                break;
            }
            *curr_x = x;

            if !history_full.insert((*curr_x, *curr_y, curr_dir)) {
                return (*curr_x == width - 1, true);
            }
        }

        return (*curr_x == width - 1, false);
    } else if curr_dir == 2 {
        for y in *curr_y..height {
            if map[y][*curr_x] == '#' {
                break;
            }
            *curr_y = y;

            if !history_full.insert((*curr_x, *curr_y, curr_dir)) {
                return (*curr_y == height - 1, true);
            }
        }

        return (*curr_y == height - 1, false);
    } else if curr_dir == 3 {
        for x in (0..=*curr_x).rev() {
            if map[*curr_y][x] == '#' {
                break;
            }
            *curr_x = x;

            if !history_full.insert((*curr_x, *curr_y, curr_dir)) {
                return (*curr_x == 0, true);
            }
        }

        return (*curr_x == 0, false);
    }

    return (true, true);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./input1.txt");
        let result = process(input);
        assert_eq!(result, 1604);
    }
}
