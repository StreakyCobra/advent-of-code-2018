use utils;

pub fn solve() {
    let input: String = utils::parse_file("input/08.txt");
    let content = parse(&input);

    println!("Problem 08");
    println!("\tFirst part:  {}", solve_first_part(&content));
    println!("\tSecond part: {}", solve_second_part(&content));
}

fn parse(input: &str) -> Vec<u8> {
    let values: Vec<u8> = input
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();
    return values;
}

fn solve_first_part(values: &Vec<u8>) -> usize {
    let values_copy = values.clone();
    return sum_first_part(&mut values_copy.iter());
}

fn sum_first_part<'a, I>(values: &mut I) -> usize
where
    I: Iterator<Item = &'a u8>,
{
    let nchd: usize = values.next().unwrap().clone() as usize;
    let nval: usize = values.next().unwrap().clone() as usize;
    let mut total: usize = 0;
    for _ in 0..nchd {
        total += sum_first_part(values);
    }
    for _ in 0..nval {
        let val = values.next().unwrap().clone() as usize;
        total += val;
    }
    return total;
}

fn solve_second_part(values: &Vec<u8>) -> usize {
    let values_copy = values.clone();
    return sum_second_part(&mut values_copy.iter());
}

fn sum_second_part<'a, I>(values: &mut I) -> usize
where
    I: Iterator<Item = &'a u8>,
{
    let nchd: usize = values.next().unwrap().clone() as usize;
    let nval: usize = values.next().unwrap().clone() as usize;
    let mut total = 0;
    let mut subtree: Vec<usize> = vec![0; nchd];
    for n in 0..nchd {
        subtree[n] += sum_second_part(values);
    }
    for _ in 0..nval {
        let val = values.next().unwrap().clone() as usize;
        if nchd == 0 {
            total += val;
        } else if val <= nchd {
            total += subtree[val - 1]
        }
    }
    return total;
}

#[cfg(test)]
mod tests {

    use super::{parse, solve_first_part, solve_second_part};

    #[test]
    fn eighth_problem_first_part() {
        let values = parse("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2");
        assert_eq!(solve_first_part(&values), 138);
    }

    #[test]
    fn eighth_problem_second_part() {
        let values = parse("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2");
        assert_eq!(solve_second_part(&values), 66);
    }
}
