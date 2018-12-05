use utils;

// Problem solving template.
//
// To do:
// 1. Change input number (00.txt) and problem number
// 2. Search and replace "zeroth" by problem number
// 3. Delete this comment
// 4. Solve the problems \o/

pub fn solve() {
    let input: String = utils::parse_file("input/00.txt");
    let content = parse(&input);

    println!("Problem 00");
    println!("\tFirst part:  {}", solve_first_part(content));
    println!("\tSecond part: {}", solve_second_part(content));
}

fn parse(content: &str) -> &str {
    return content;
}

fn solve_first_part(_input: &str) -> i64 {
    return 0;
}

fn solve_second_part(_input: &str) -> i64 {
    return 0;
}

#[cfg(test)]
mod tests {

    use super::{parse, solve_first_part, solve_second_part};

    #[test]
    fn zeroth_problem_first_part() {
        let content = parse("");
        assert_eq!(solve_first_part(content), 0);
    }

    #[test]
    fn zeroth_problem_second_part() {
        let content = parse("");
        assert_eq!(solve_second_part(content), 0);
    }
}
