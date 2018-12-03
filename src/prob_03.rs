use std::collections::{HashMap, HashSet};
use std::process;
use utils;

const SIZE: usize = 1000;

#[derive(Debug)]
struct Rectangle {
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}

impl Rectangle {
    fn new(square: &str) -> Rectangle {
        let mut iter = square.split(':');
        let pos: &str = iter.next().unwrap().trim();
        let size: &str = iter.next().unwrap().trim();
        let (x, y) = pos.split_at(pos.find(',').unwrap());
        let (w, h) = size.split_at(size.find('x').unwrap());
        Rectangle {
            left: x.parse::<usize>().unwrap(),
            top: y
                .chars()
                .skip(1)
                .collect::<String>()
                .parse::<usize>()
                .unwrap(),
            width: w.parse::<usize>().unwrap(),
            height: h
                .chars()
                .skip(1)
                .collect::<String>()
                .parse::<usize>()
                .unwrap(),
        }
    }

    fn mark(&self, fabric: &mut [[u8; SIZE]; SIZE]) {
        for i in self.left..self.left + self.width {
            for j in self.top..self.top + self.height {
                fabric[j][i] += 1;
            }
        }
    }

    fn overlap(&self, other: &Rectangle) -> bool {
        if other.left + other.width - 1 < self.left {
            return false;
        }
        if other.top + other.height - 1 < self.top {
            return false;
        }
        if other.left > self.left + self.width - 1 {
            return false;
        }
        if other.top > self.top + self.height - 1 {
            return false;
        }
        return true;
    }
}

pub fn solve() {
    let content: String = utils::parse_file("input/03.txt");
    let squares: HashMap<String, Rectangle> = parse(&content);

    println!("Problem 03");
    println!("\tFirst part:  {}", solve_first_part(&squares));
    println!("\tSecond part: {}", solve_second_part(&squares));
}

fn parse(content: &str) -> HashMap<String, Rectangle> {
    let mut result: HashMap<String, Rectangle> = HashMap::new();
    for line in content.lines() {
        let mut iter = line.split('@');
        let id: String = iter.next().unwrap().trim().chars().skip(1).collect();
        let square = iter.next().unwrap().trim();
        let rectangle = Rectangle::new(square);
        result.insert(id, rectangle);
    }
    return result;
}

fn solve_first_part(squares: &HashMap<String, Rectangle>) -> u64 {
    let mut fabric: [[u8; SIZE]; SIZE] = [[0; SIZE]; SIZE];
    for (_id, square) in squares {
        square.mark(&mut fabric);
    }
    return fabric
        .iter()
        .map(|row| {
            row.iter()
                .map(|&val| if val > 1 { 1 } else { 0 } as u64)
                .sum::<u64>()
        }).sum();
}

fn solve_second_part(squares: &HashMap<String, Rectangle>) -> &String {
    let mut overlapping: HashSet<String> = HashSet::new();
    for a in 0..squares.len() {
        let (first_id, first) = squares.iter().nth(a).unwrap();
        for b in a + 1..squares.len() {
            let (other_id, other) = squares.iter().nth(b).unwrap();
            if first.overlap(other) {
                overlapping.insert(first_id.to_string());
                overlapping.insert(other_id.to_string());
            }
        }
        if !overlapping.contains(first_id) {
            return first_id;
        }
    }
    println!("There is no answer!");
    process::exit(1);
}

#[cfg(test)]
mod tests {

    use super::{parse, solve_first_part, solve_second_part};

    #[test]
    fn third_problem_first_part() {
        let content = "#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2";
        let squares = parse(content);
        assert_eq!(solve_first_part(&squares), 4);
    }

    #[test]
    fn third_problem_second_part() {
        let content = "#1 @ 1,3: 4x4\n#2 @ 3,1: 4x4\n#3 @ 5,5: 2x2";
        let squares = parse(content);
        assert_eq!(solve_second_part(&squares), "3");
    }
}
