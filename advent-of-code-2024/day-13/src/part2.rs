fn parse_coordinates(line: &str) -> (i64, i64) {
    let parts: Vec<_> = line.split(',').collect();
    let x = parts[0][2..].parse::<i64>().unwrap();
    let y = parts[1].trim()[2..].parse::<i64>().unwrap();
    (x, y)
}

#[derive(Debug)]
struct Puzzle {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

// a = puzzle.button_a.0
// b = puzzle.button_b.0
// c = puzzle.prize.0
// d = puzzle.button_a.1
// e = puzzle.button_b.1
// f = puzzle.prize.1

fn solve_puzzle(puzzle: &Puzzle) -> i64 {
    let determinant = puzzle.button_a.0 * puzzle.button_b.1 - puzzle.button_a.1 * puzzle.button_b.0;

    if determinant == 0 {
        return 0;
    }

    let x = (puzzle.prize.0 as f64 * puzzle.button_b.1 as f64
        - puzzle.button_b.0 as f64 * puzzle.prize.1 as f64)
        / determinant as f64;
    let y = (puzzle.button_a.0 as f64 * puzzle.prize.1 as f64
        - puzzle.prize.0 as f64 * puzzle.button_a.1 as f64)
        / determinant as f64;

    if x.floor() != x || y.floor() != y {
        return 0;
    }
    x as i64 * 3 + y as i64
}

pub fn process(input: &str) -> i64 {
    let mut res = 0;

    let mut lines = input.lines().filter(|line| !line.trim().is_empty());

    while let Some(line) = lines.next() {
        let button_a = parse_coordinates(&line[10..]);
        let button_b = parse_coordinates(&(lines.next().unwrap())[10..]);
        let prize = parse_coordinates(&(lines.next().unwrap())[7..]);

        res += solve_puzzle(&Puzzle {
            button_a,
            button_b,
            prize: (10000000000000 + prize.0, 10000000000000 + prize.1),
        })
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
        assert_eq!(result, 88584689879723);
    }
}
