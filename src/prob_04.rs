use chrono::{NaiveDateTime, Timelike};
use std::collections::HashMap;
use std::process;
use utils;

#[derive(Debug, Clone, PartialEq)]
enum Event {
    Start,
    Sleep,
    Wakeup,
}

#[derive(Debug, Clone)]
struct Record {
    datetime: NaiveDateTime,
    guard: u64,
    event: Event,
}

impl Record {
    pub fn parse(entry: &str, previous: Option<Record>) -> Record {
        let datetime: String = entry.chars().skip(1).take(16).collect();
        let event = match entry.chars().nth(19).unwrap() {
            'G' => Event::Start,
            'f' => Event::Sleep,
            'w' => Event::Wakeup,
            _ => {
                println!("Error when parsing records");
                process::exit(1);
            }
        };
        let guard: u64 = if event == Event::Start {
            entry
                .split(" ")
                .nth(3)
                .unwrap()
                .chars()
                .skip(1)
                .collect::<String>()
                .parse()
                .unwrap()
        } else {
            previous.unwrap().guard
        };
        Record {
            datetime: NaiveDateTime::parse_from_str(&datetime, "%Y-%m-%d %H:%M").unwrap(),
            guard: guard,
            event: event,
        }
    }
}

pub fn solve() {
    let input: String = utils::parse_file("input/04.txt");
    let content = parse(&input);

    println!("Problem 01");
    println!("\tFirst part:  {}", solve_first_part(&content));
    println!("\tSecond part: {}", solve_second_part(&content));
}

fn parse(content: &str) -> Vec<Record> {
    let mut entries: Vec<&str> = content.lines().collect();
    entries.sort();
    let mut records: Vec<Record> = Vec::new();
    let mut previous: Option<Record> = None;
    for entry in entries {
        let record = Record::parse(entry, previous);
        previous = Some(record.clone());
        records.push(record);
    }
    return records;
}

fn solve_first_part(records: &Vec<Record>) -> u64 {
    let mut totals: HashMap<u64, u64> = HashMap::new();
    let mut from = 0;
    for record in records {
        match record.event {
            Event::Start => (),
            Event::Sleep => from = record.datetime.minute() as u64,
            Event::Wakeup => {
                let length: u64 = record.datetime.minute() as u64 - from;
                if totals.contains_key(&record.guard) {
                    let total = totals.get(&record.guard).unwrap().clone();
                    totals.insert(record.guard, total + length);
                } else {
                    totals.insert(record.guard, length);
                }
            }
        }
    }
    let (maxguard, _) = totals.iter().max_by_key(|&(_k, v)| v).unwrap();
    let mut minutes: [u64; 60] = [0; 60];
    for record in records {
        if &record.guard != maxguard {
            continue;
        };
        match record.event {
            Event::Start => (),
            Event::Sleep => from = record.datetime.minute() as u64,
            Event::Wakeup => {
                let to: u64 = record.datetime.minute() as u64;
                for i in from as usize..to as usize {
                    minutes[i] += 1
                }
            }
        }
    }
    let (maxtime, _) = minutes.iter().enumerate().max_by_key(|&(_p, v)| v).unwrap();
    return maxguard * maxtime as u64;
}

fn solve_second_part(records: &Vec<Record>) -> u64 {
    let mut stats: HashMap<u64, [usize; 60]> = HashMap::new();
    let mut from: usize = 0;
    let mut max: (u64, usize, usize) = (0, 0, 0);
    for record in records {
        match record.event {
            Event::Start => (),
            Event::Sleep => from = record.datetime.minute() as usize,
            Event::Wakeup => {
                let to: usize = record.datetime.minute() as usize;
                if !stats.contains_key(&record.guard) {
                    let mut minutes = [0; 60];
                    stats.insert(record.guard, minutes);
                }
                let mut minutes = stats.get_mut(&record.guard).unwrap();
                for i in from..to {
                    minutes[i] += 1;
                    if minutes[i] > max.2 {
                        max.0 = record.guard;
                        max.1 = i;
                        max.2 = minutes[i];
                    }
                }
            }
        }
    }
    return max.0 * max.1 as u64;
}

#[cfg(test)]
mod tests {

    use super::{parse, solve_first_part, solve_second_part};

    #[test]
    fn fourth_problem_first_part() {
        let records = parse("[1518-11-01 00:00] Guard #10 begins shift\n[1518-11-01 00:05] falls asleep\n[1518-11-01 00:25] wakes up\n[1518-11-01 00:30] falls asleep\n[1518-11-01 00:55] wakes up\n[1518-11-01 23:58] Guard #99 begins shift\n[1518-11-02 00:40] falls asleep\n[1518-11-02 00:50] wakes up\n[1518-11-03 00:05] Guard #10 begins shift\n[1518-11-03 00:24] falls asleep\n[1518-11-03 00:29] wakes up\n[1518-11-04 00:02] Guard #99 begins shift\n[1518-11-04 00:36] falls asleep\n[1518-11-04 00:46] wakes up\n[1518-11-05 00:03] Guard #99 begins shift\n[1518-11-05 00:45] falls asleep\n[1518-11-05 00:55] wakes up");
        assert_eq!(solve_first_part(&records), 240);
    }

    #[test]
    fn fourth_problem_second_part() {
        let records = parse("[1518-11-01 00:00] Guard #10 begins shift\n[1518-11-01 00:05] falls asleep\n[1518-11-01 00:25] wakes up\n[1518-11-01 00:30] falls asleep\n[1518-11-01 00:55] wakes up\n[1518-11-01 23:58] Guard #99 begins shift\n[1518-11-02 00:40] falls asleep\n[1518-11-02 00:50] wakes up\n[1518-11-03 00:05] Guard #10 begins shift\n[1518-11-03 00:24] falls asleep\n[1518-11-03 00:29] wakes up\n[1518-11-04 00:02] Guard #99 begins shift\n[1518-11-04 00:36] falls asleep\n[1518-11-04 00:46] wakes up\n[1518-11-05 00:03] Guard #99 begins shift\n[1518-11-05 00:45] falls asleep\n[1518-11-05 00:55] wakes up");
        assert_eq!(solve_second_part(&records), 4455);
    }
}
