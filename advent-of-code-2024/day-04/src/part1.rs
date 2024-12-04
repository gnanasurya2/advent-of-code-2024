fn is_valid(grid: &Vec<Vec<char>>, y: i32, x: i32) -> bool {
    x >= 0 && x < grid[0].len() as i32 && y >= 0 && y < grid.len() as i32
}

fn check_xmas(grid: &Vec<Vec<char>>, pos: (i32, i32), d: (i32, i32)) -> bool {
    let (px, py) = pos;
    let (dx, dy) = d;

    "MAS".chars().enumerate().all(|(i, c)| {
        let i = i as i32;
        is_valid(grid, py + dy * (1 + i), px + dx * (1 + i))
            && grid[(py + dy * (1 + i)) as usize][(px + dx * (1 + i)) as usize] == c
    })
}

pub fn process(input: &str) -> i64 {
    let mut res = 0;
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.trim().chars().collect()).collect();

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 'X' {
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        res += check_xmas(&grid, (x as i32, y as i32), (dx, dy)) as i32;
                    }
                }
            }
        }
    }
    res as i64
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
