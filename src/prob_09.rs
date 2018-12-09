use utils;

pub fn solve() {
    let input: String = utils::parse_file("input/09.txt");
    let content = parse(&input);
    let second_content = (content.0, content.1 * 100);

    println!("Problem 09");
    println!("\tFirst part:  {}", solve_first_part(content));
    println!("\tSecond part: {}", solve_first_part(second_content));
}

fn parse(content: &str) -> (usize, usize) {
    let line = content.lines().nth(0).unwrap();
    let words: Vec<&str> = line.split_whitespace().collect();
    let num_players: usize = words[0].parse().unwrap();
    let last_marble: usize = words[6].parse().unwrap();
    return (num_players, last_marble + 1);
}

fn solve_first_part(rules: (usize, usize)) -> u32 {
    let mut circle: Vec<usize> = vec![0];
    let mut scores: Vec<usize> = vec![0; rules.0];
    let mut pos: usize = 0;
    for val in 1..rules.1 {
        if val % 23 != 0 {
            pos += 2;
            pos %= circle.len();
            if pos == 0 {
                circle.push(val);
                pos = circle.len() - 1;
            } else {
                circle.insert(pos, val);
            }
        } else {
            if pos < 7 {
                pos += circle.len() - 7
            } else {
                pos -= 7
            };
            let removed = circle.remove(pos);
            scores[val % rules.0] += val;
            scores[val % rules.0] += removed;
        }
    }
    return scores.iter().max().unwrap().clone() as u32;
}

#[cfg(test)]
mod tests {

    use super::{parse, solve_first_part};

    #[test]
    fn ninth_problem_first_part() {
        assert_eq!(
            solve_first_part(parse(&"9 players; last marble is worth 25 points")),
            32
        );
        assert_eq!(
            solve_first_part(parse(&"10 players; last marble is worth 1618 points")),
            8317
        );
        assert_eq!(
            solve_first_part(parse(&"13 players; last marble is worth 7999 points")),
            146373
        );
        assert_eq!(
            solve_first_part(parse(&"17 players; last marble is worth 1104 points")),
            2764
        );
        assert_eq!(
            solve_first_part(parse(&"21 players; last marble is worth 6111 points")),
            54718
        );
        assert_eq!(
            solve_first_part(parse(&"30 players; last marble is worth 5807 points")),
            37305
        );
    }
}
