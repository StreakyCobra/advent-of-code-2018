use utils;

pub fn solve() {
    let content: String = utils::parse_file("input/01.txt");

    let freqs: Vec<i64> = content
        .lines()
        .map(|line| line.parse().ok().unwrap())
        .collect();

    println!("Problem 01");
    println!("\tFirst part:  {}", solve_first_part(&freqs));
    println!("\tSecond part: {}", solve_second_part(&freqs));
}

fn solve_first_part(freqs: &Vec<i64>) -> i64 {
    return freqs.into_iter().sum();
}

fn solve_second_part(freqs: &Vec<i64>) -> i64 {
    unimplemented!()
}

#[cfg(test)]
mod tests {

    use super::{solve_first_part, solve_second_part};

    #[test]
    fn first_problem_first_part() {
        assert_eq!(solve_first_part(&vec![1, -2, 3, 1]), 3);
        assert_eq!(solve_first_part(&vec![1, 1, 1]), 3);
        assert_eq!(solve_first_part(&vec![1, 1, -2]), 0);
        assert_eq!(solve_first_part(&vec![-1, -2, -3]), -6);
    }

    #[test]
    fn first_problem_second_part() {
        assert_eq!(solve_second_part(&vec![1, -2, 3, 1]), 3);
        assert_eq!(solve_second_part(&vec![1, -1]), 0);
        assert_eq!(solve_second_part(&vec![3, 3, 4, -2, -4]), 10);
        assert_eq!(solve_second_part(&vec![-6, 3, 8, 5, -6]), 5);
        assert_eq!(solve_second_part(&vec![7, 7, -2, -7, -4]), 14);
    }
}
