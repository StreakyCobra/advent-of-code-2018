use std::collections::HashMap;
use std::process;
use utils;

pub fn solve() {
    let content: String = utils::parse_file("input/02.txt");

    let boxs: Vec<&str> = content.lines().collect();

    println!("Problem 02");
    println!("\tFirst part:  {}", solve_first_part(&boxs));
    println!("\tSecond part: {}", solve_second_part(&boxs));
}

fn solve_first_part(boxs: &Vec<&str>) -> u64 {
    let mut count: HashMap<char, usize> = HashMap::new();
    let mut two: u64 = 0;
    let mut three: u64 = 0;
    for id in boxs {
        // Frequencies
        for chr in id.chars() {
            if count.contains_key(&chr) {
                let current = count.get(&chr).unwrap().clone();
                count.insert(chr, current + 1);
            } else {
                count.insert(chr, 1);
            }
        }
        // Count twos
        if count.values().any(|&v| v == 2) {
            two += 1
        };
        // Count threes
        if count.values().any(|&v| v == 3) {
            three += 1
        };
        count.clear();
    }
    return two * three;
}

fn solve_second_part(boxs: &Vec<&str>) -> String {
    for i in 0..boxs.len() {
        let id = boxs[i];
        for j in i..boxs.len() {
            let cmp = boxs[j];
            let diffs: usize = id
                .chars()
                .zip(cmp.chars())
                .map(|(a, b)| if a == b { 0 } else { 1 })
                .sum();
            if diffs == 1 {
                let mut result = String::new();
                for (a, b) in id.chars().zip(cmp.chars()) {
                    if a == b {
                        result.push(a)
                    }
                }
                return result;
            }
        }
    }
    println!("There is no answer!");
    process::exit(1);
}

#[cfg(test)]
mod tests {

    use super::{solve_first_part, solve_second_part};

    #[test]
    fn second_problem_first_part() {
        let example_ids: Vec<&str> = vec![
            "abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab",
        ];
        assert_eq!(solve_first_part(&example_ids), 12);
    }

    #[test]
    fn second_problem_second_part() {
        let example_ids: Vec<&str> = vec![
            "abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz",
        ];
        assert_eq!(solve_second_part(&example_ids), "fgij");
    }
}
