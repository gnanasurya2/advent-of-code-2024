fn is_valid(grid: &Vec<Vec<char>>, y: i32, x: i32) -> bool {
    x >= 0 && x < grid[0].len() as i32 && y >= 0 && y < grid.len() as i32
}

fn check_xmas(grid: &Vec<Vec<char>>, x: i32, y: i32) -> bool {
    if !((-1..=1)
        .flat_map(|dx| (-1..=1).map(move |dy| (dx, dy)))
        .filter(|(dx, dy)| *dx != 0 && *dy != 0)
        .all(|(dx, dy)| is_valid(grid, y + dy, x + dx)))
    {
        return false;
    }

    let chars = [
        grid[(y + 1) as usize][(x + 1) as usize],
        grid[(y - 1) as usize][(x - 1) as usize],
        grid[(y - 1) as usize][(x + 1) as usize],
        grid[(y + 1) as usize][(x - 1) as usize],
    ];

    chars.iter().filter(|&&c| c == 'S').count() == 2
        && chars.iter().filter(|&&c| c == 'M').count() == 2
        && chars[0] != chars[1]
}

pub fn process(input: &str) -> i64 {
    let mut res = 0;
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.trim().chars().collect()).collect();

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 'A' {
                res += check_xmas(&grid, x as i32, y as i32) as i32;
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
