use std::collections::{HashMap, HashSet};
use utils;

pub fn solve() {
    let input: String = utils::parse_file("input/07.txt");
    let dependencies = parse(&input);

    println!("Problem 07");
    println!("\tFirst part:  {}", solve_first_part(&dependencies));
    println!("\tSecond part: {}", solve_second_part(&dependencies, 60, 5));
}

fn parse(content: &str) -> HashMap<char, HashSet<char>> {
    let mut dependencies: HashMap<char, HashSet<char>> = HashMap::new();
    for line in content.lines() {
        let a: char = line.chars().nth(5).unwrap();
        let b: char = line.chars().nth(36).unwrap();
        dependencies.entry(a).or_insert(HashSet::new());
        dependencies.entry(b).or_insert(HashSet::new());
        dependencies.get_mut(&b).unwrap().insert(a);
    }
    return dependencies;
}

fn solve_first_part(dependencies: &HashMap<char, HashSet<char>>) -> String {
    let mut result = String::new();
    let mut depends = dependencies.clone();
    while !depends.is_empty() {
        let mut upcoming: Vec<char> = depends
            .iter()
            .filter(|&(c, d)| d.len() == 0)
            .map(|(c, d)| c.clone())
            .collect();
        upcoming.sort();
        let next = upcoming[0];
        result.push(next);
        depends.remove(&next);
        for (_, v) in depends.iter_mut() {
            if v.contains(&next) {
                v.remove(&next);
            }
        }
    }
    return result;
}

fn solve_second_part(
    dependencies: &HashMap<char, HashSet<char>>,
    basetime: usize,
    num_workers: usize,
) -> usize {
    let mut workers: Vec<(Option<char>, usize)> = vec![(None, 0); num_workers];
    let mut clock: usize = 0;
    let mut depends = dependencies.clone();
    while !depends.is_empty() || workers.iter().any(|&(c, _)| c != None) {
        // Deal with workers tasks
        for (task, time) in workers.iter_mut() {
            let mut changed: bool = false;
            if let Some(chr) = task {
                // If task is finished, remove it from others deps
                if time == &0 {
                    changed = true;
                    for (_, v) in depends.iter_mut() {
                        if v.contains(chr) {
                            v.remove(chr);
                        }
                    }
                }
                // Otherwise decrease time
                else {
                    *time -= 1
                }
            }
            if changed { *task = None};
        }
        // Upcoming tasks
        let mut upcoming: Vec<char> = depends
            .iter()
            .filter(|&(c, d)| d.len() == 0)
            .map(|(c, d)| c.clone())
            .collect();
        upcoming.sort();
        // Split tasks to workers
        'task: for chr in upcoming {
            'worker: for (task, time) in workers.iter_mut() {
                if *task != None {
                    continue 'worker;
                }
                *task = Some(chr);
                *time = basetime + chr as usize - 65;
                depends.remove(&chr);
                continue 'task;
            }
            break;
        }
        // Clock step
        clock += 1;
    }
    return clock - 1;
}

#[cfg(test)]
mod tests {

    use super::{parse, solve_first_part, solve_second_part};

    #[test]
    fn seventh_problem_first_part() {
        let dependencies = parse("Step C must be finished before step A can begin.\nStep C must be finished before step F can begin.\nStep A must be finished before step B can begin.\nStep A must be finished before step D can begin.\nStep B must be finished before step E can begin.\nStep D must be finished before step E can begin.\nStep F must be finished before step E can begin.\n");
        assert_eq!(solve_first_part(&dependencies), "CABDFE");
    }

    #[test]
    fn seventh_problem_second_part() {
        let dependencies = parse("Step C must be finished before step A can begin.\nStep C must be finished before step F can begin.\nStep A must be finished before step B can begin.\nStep A must be finished before step D can begin.\nStep B must be finished before step E can begin.\nStep D must be finished before step E can begin.\nStep F must be finished before step E can begin.\n");
        assert_eq!(solve_second_part(&dependencies, 0, 2), 15);
    }
}
