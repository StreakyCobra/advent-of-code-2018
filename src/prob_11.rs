use utils;

const SIZE: usize = 300;

pub fn solve() {
    let input: String = utils::parse_file("input/11.txt");
    let content = parse(&input);

    println!("Problem 11");
    println!("\tFirst part:  {:?}", solve_first_part(content));
    println!("\tSecond part: {:?}", solve_second_part(content));
}

fn parse(content: &str) -> i64 {
    return content.lines().next().unwrap().parse().unwrap();
}

fn power_level(serial: i64, x: usize, y: usize) -> i64 {
    let rack_id = x as i64 + 10;
    let mut power_level: i64 = rack_id * y as i64;
    power_level += serial;
    power_level *= rack_id;
    power_level /= 100;
    power_level %= 10;
    power_level -= 5;
    return power_level;
}

fn solve_first_part(serial: i64) -> (usize, usize) {
    let mut grid: [[i64; SIZE]; SIZE] = [[0; SIZE]; SIZE];
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            grid[i][j] = power_level(serial, j + 1, i + 1);
        }
    }
    let mut max: i64 = 0;
    let mut max_pos: (usize, usize) = (0, 0);
    let mut sum: i64;
    for i in 0..grid.len() - 3 {
        for j in 0..grid.len() - 3 {
            sum = 0;
            for wi in 0..3 {
                for wj in 0..3 {
                    sum += grid[i + wi][j + wj];
                }
            }
            if sum > max {
                max = sum;
                max_pos = (i, j)
            }
        }
    }
    return (max_pos.1 + 1, max_pos.0 + 1);
}

fn solve_second_part(serial: i64) -> (usize, usize, usize) {
    let mut grid: [[i64; SIZE]; SIZE] = [[0; SIZE]; SIZE];
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            grid[i][j] = power_level(serial, j + 1, i + 1);
        }
    }
    let mut max: i64 = 0;
    let mut max_pos: (usize, usize) = (0, 0);
    let mut max_size: usize = 0;
    let mut sum: i64;
    for size in 1..301 {
        for i in 0..grid.len() - size {
            for j in 0..grid.len() - size {
                sum = 0;
                for wi in 0..size {
                    for wj in 0..size {
                        sum += grid[i + wi][j + wj];
                    }
                }
                if sum > max {
                    max = sum;
                    max_pos = (i, j);
                    max_size = size;
                }
            }
        }
    }
    return (max_pos.1 + 1, max_pos.0 + 1, max_size);
}

#[cfg(test)]
mod tests {

    use super::{parse, power_level, solve_first_part, solve_second_part};

    #[test]
    fn eleventh_problem_first_part() {
        assert_eq!(power_level(8, 3, 5), 4);
        assert_eq!(power_level(57, 122, 79), -5);
        assert_eq!(power_level(39, 217, 196), 0);
        assert_eq!(power_level(71, 101, 153), 4);
        assert_eq!(solve_first_part(parse("18")), (33, 45));
        assert_eq!(solve_first_part(parse("42")), (21, 61));
    }

    #[test]
    fn eleventh_problem_second_part() {
        assert_eq!(solve_second_part(parse("18")), (90, 269, 16));
        assert_eq!(solve_second_part(parse("42")), (232, 251, 12));
    }
}
