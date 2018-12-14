use std::char;
use utils;

pub fn solve() {
    let input: String = utils::parse_file("input/14.txt");
    let content = parse(&input);

    println!("Problem 14");
    println!("\tFirst part:  {}", solve_first_part(content));
    println!("\tSecond part: {}", solve_second_part(&content.to_string()));
}

fn parse(content: &str) -> usize {
    return content.lines().nth(0).unwrap().trim().parse().unwrap();
}

fn solve_first_part(number: usize) -> String {
    let mut recipes: Vec<u8> = vec![0; 1_000_000];
    recipes[0] = 3;
    recipes[1] = 7;
    let mut a = 0;
    let mut b = 1;
    let mut len = 2;
    let mut new;
    while len < number + 10 {
        new = (recipes[a] + recipes[b]).to_string();
        for d in new.chars() {
            recipes[len] = d as u8 - '0' as u8;
            len += 1;
        }
        a += recipes[a] as usize + 1;
        a %= len;
        b += recipes[b] as usize + 1;
        b %= len;
    }
    return recipes
        .iter()
        .skip(number)
        .take(10)
        .map(|d| char::from_digit((*d) as u32, 10).unwrap())
        .collect();
}

fn solve_second_part(number: &str) -> usize {
    let digits: Vec<u8> = number.chars().map(|d| d as u8 - '0' as u8).collect();
    let mut recipes: Vec<u8> = vec![3, 7];
    let mut a = 0;
    let mut b = 1;
    let mut len = 2;
    let mut new;
    loop {
        new = (recipes[a] + recipes[b]).to_string();
        for d in new.chars() {
            recipes.push(d as u8 - '0' as u8);
            len += 1;
            if len > number.len() && recipes[(len - number.len())..len] == digits[0..digits.len()] {
                return len - number.len();
            }
        }
        a += recipes[a] as usize + 1;
        a %= len;
        b += recipes[b] as usize + 1;
        b %= len;
    }
}

#[cfg(test)]
mod tests {

    use super::{solve_first_part, solve_second_part};

    #[test]
    fn fourteenth_problem_first_part() {
        assert_eq!(solve_first_part(9), "5158916779");
        assert_eq!(solve_first_part(5), "0124515891");
        assert_eq!(solve_first_part(18), "9251071085");
        assert_eq!(solve_first_part(2018), "5941429882");
    }

    #[test]
    fn fourteenth_problem_second_part() {
        assert_eq!(solve_second_part("51589"), 9);
        assert_eq!(solve_second_part("01245"), 5);
        assert_eq!(solve_second_part("92510"), 18);
        assert_eq!(solve_second_part("59414"), 2018);
    }
}
