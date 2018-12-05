use std::cmp;
use utils;

pub fn solve() {
    let input: String = utils::parse_file("input/05.txt");
    let polymer: &str = input.trim();

    println!("Problem 05");
    println!("\tFirst part:  {}", solve_first_part(polymer));
    println!("\tSecond part: {}", solve_second_part(polymer));
}

fn solve_first_part(polymer: &str) -> usize {
    let mut chars: Vec<char> = polymer.chars().collect();
    let mut i = 0;
    while i + 1 < chars.len() {
        if chars[i].to_ascii_lowercase() == chars[i + 1].to_ascii_lowercase()
            && chars[i] != chars[i + 1]
        {
            chars.remove(i);
            chars.remove(i);
            if i > 0 {
                i -= 1;
            }
        } else {
            i += 1;
        }
    }
    return chars.len();
}

fn solve_second_part(polymer: &str) -> usize {
    let mut min = polymer.len();
    for c in 0..26u8 {
        let chr = (97 + c) as char;
        let changed: String = polymer
            .chars()
            .filter(|v| v.to_ascii_lowercase() != chr)
            .collect();
        let score = solve_first_part(&changed);
        min = cmp::min(score, min);
    }
    return min;
}

#[cfg(test)]
mod tests {

    use super::{solve_first_part, solve_second_part};

    #[test]
    fn fifth_problem_first_part() {
        assert_eq!(solve_first_part("aA"), 0);
        assert_eq!(solve_first_part("abBA"), 0);
        assert_eq!(solve_first_part("abAB"), 4);
        assert_eq!(solve_first_part("aabAAB"), 6);
        assert_eq!(solve_first_part("dabAcCaCBAcCcaDA"), 10);
    }

    #[test]
    fn fifth_problem_second_part() {
        assert_eq!(solve_second_part("dabAcCaCBAcCcaDA"), 4);
    }
}
