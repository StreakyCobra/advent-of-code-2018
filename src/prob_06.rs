use std::collections::{HashMap, HashSet};
use utils;

const SIZE: usize = 1000;

pub fn solve() {
    let input: String = utils::parse_file("input/06.txt");
    let centroids = parse(&input);

    println!("Problem 06");
    println!("\tFirst part:  {}", solve_first_part(&centroids));
    println!("\tSecond part: {}", solve_second_part(&centroids, 10000));
}

fn parse(content: &str) -> Vec<(usize, usize)> {
    let mut centroids: Vec<(usize, usize)> = Vec::new();
    for line in content.trim().lines() {
        let (a, b) = line.split_at(line.find(',').unwrap());
        let x: usize = a.parse().unwrap();
        let y: usize = b.chars().skip(2).collect::<String>().parse().unwrap();
        centroids.push((y, x));
    }
    return centroids;
}

fn distance(a: &(usize, usize), b: &(usize, usize)) -> usize {
    let x1 = a.0 as isize;
    let y1 = a.1 as isize;
    let x2 = b.0 as isize;
    let y2 = b.1 as isize;
    return ((x2 - x1).abs() + (y2 - y1).abs()) as usize;
}

fn counter(grid: &[[u8; SIZE]; SIZE]) -> HashMap<u8, usize> {
    let mut counts: HashMap<u8, usize> = HashMap::new();
    for value in grid.iter().flat_map(|array| array.iter()) {
        let count = counts.get(&value).unwrap_or(&0).clone();
        counts.insert(*value, count + 1);
    }
    return counts;
}

fn solve_first_part(centroids: &Vec<(usize, usize)>) -> usize {
    let mut grid: [[u8; SIZE]; SIZE] = [[0; SIZE]; SIZE];
    for j in 0..grid.len() {
        for i in 0..grid[j].len() {
            let mut distances: Vec<(usize, usize)> = centroids
                .iter()
                .enumerate()
                .map(|(n, p)| (n + 1, distance(&p, &(i, j))))
                .collect();
            distances.sort_by_key(|&(_, p)| p);
            grid[j][i] = if distances[0].1 < distances[1].1 {
                distances[0].0 as u8
            } else {
                0u8
            };
        }
    }
    let counts: HashMap<u8, usize> = counter(&grid);
    let mut excluded: HashSet<u8> = HashSet::new();
    excluded.insert(0);
    for i in 0..grid.len() {
        excluded.insert(grid[i][0]);
        excluded.insert(grid[0][i]);
        excluded.insert(grid[i][grid.len()-1]);
        excluded.insert(grid[grid.len()-1][i]);
    }
    let result: &usize = counts.iter().filter(|&(k, _)| !excluded.contains(k)).max_by_key(|&(_,v)| v).unwrap().1;
    return *result;
}

fn solve_second_part(centroids: &Vec<(usize, usize)>, limit: usize) -> usize {
    let mut grid: [[usize; SIZE]; SIZE] = [[0; SIZE]; SIZE];
    for j in 0..grid.len() {
        for i in 0..grid[j].len() {
            let mut total: usize = centroids
                .iter()
                .map(|p| distance(&p, &(i, j)))
                .sum();
            grid[j][i] = total;
        }
    }
    return grid.iter().flat_map(|row| row.iter()).filter(|&v| v < &limit).count()
}

#[cfg(test)]
mod tests {

    use super::{parse, solve_first_part, solve_second_part};

    #[test]
    fn sixth_problem_first_part() {
        let content = parse("1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9\n");
        assert_eq!(solve_first_part(&content), 17);
    }

    #[test]
    fn sixth_problem_second_part() {
        let content = parse("1, 1\n1, 6\n8, 3\n3, 4\n5, 5\n8, 9\n");
        assert_eq!(solve_second_part(&content, 32), 16);
    }
}
