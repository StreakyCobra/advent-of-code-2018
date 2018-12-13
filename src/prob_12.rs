use utils;

// Number of generations
const NUM_GEN: usize = 20;
// Offset to the pot zero (0)
const OFFSET: usize = 2 * NUM_GEN;
// Max size = initial state + 2 times the number of generation
const SIZE: usize = 99 + 4 * NUM_GEN;

type Statement = ([bool; SIZE], Vec<([bool; 5], bool)>);

pub fn solve() {
    let input: String = utils::parse_file("input/12.txt");
    let content = parse(&input);

    println!("Problem 12");
    println!("\tFirst part:  {}", solve_first_part(&content));
    println!("\tSecond part: {}", solve_second_part(&content));
}

#[allow(dead_code)]
fn print_pots(pots: &[bool; SIZE]) {
    for v in pots.iter() {
        let chr = if *v { '#' } else { '.' };
        print!("{}", chr);
    }
    println!("");
}

fn parse(content: &str) -> Statement {
    let mut initial: [bool; SIZE] = [false; SIZE];
    let mut rules: Vec<([bool; 5], bool)> = Vec::new();
    let mut lines = content.lines();
    // Initial state
    for (i, chr) in lines.next().unwrap().trim().chars().skip(15).enumerate() {
        if chr == '#' {
            initial[i + OFFSET] = true;
        }
    }
    lines.next();
    // Rules
    for line in lines {
        let mut rule: [bool; 5] = [false; 5];
        let chars: Vec<char> = line.trim().chars().collect();
        for i in 0..5 {
            if chars[i] == '#' {
                rule[i] = true;
            }
        }
        let outcome = if chars[9] == '#' { true } else { false };
        rules.push((rule, outcome));
    }
    return (initial, rules);
}

fn generation(curr: &[bool; SIZE], next: &mut [bool; SIZE], rules: &Vec<([bool; 5], bool)>) {
    for i in 2..curr.len() - 2 {
        next[i] = false;
        'rules: for (rule, outcome) in rules {
            for j in 0..5 {
                if rule[j] != curr[i - 2 + j] {
                    continue 'rules;
                }
            }
            next[i] = *outcome;
            break 'rules;
        }
    }
}

fn solve_first_part(statement: &Statement) -> usize {
    let (initial, rules) = statement;
    let mut curr: [bool; SIZE] = initial.clone();
    let mut next: [bool; SIZE] = initial.clone();
    for _ in 0..NUM_GEN / 2 {
        generation(&curr, &mut next, &rules);
        generation(&next, &mut curr, &rules);
    }
    return curr
        .iter()
        .enumerate()
        .map(|(i, v)| if *v { i - OFFSET } else { 0 })
        .sum();
}

fn solve_second_part(_statement: &Statement) -> &str {
    return "Solved with a calculator \\o/ I noticed that after a while the
    \tpattern keep getting the same, only shifting to the right. So by looking
    \tat the increase at each step and doing some maths I got to the right answer.";
}

#[cfg(test)]
mod tests {

    use super::{parse, solve_first_part};

    #[test]
    fn twelfth_problem_first_part() {
        let content = parse("initial state: #..#.#..##......###...###\n\n...## => #\n..#.. => #\n.#... => #\n.#.#. => #\n.#.## => #\n.##.. => #\n.#### => #\n#.#.# => #\n#.### => #\n##.#. => #\n##.## => #\n###.. => #\n###.# => #\n####. => #\n");
        assert_eq!(solve_first_part(&content), 325);
    }
}
