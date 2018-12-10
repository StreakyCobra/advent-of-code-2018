use nom;
use std::str;
use utils;

#[derive(Clone, Debug)]
struct Point {
    x: i64,
    y: i64,
    dx: i64,
    dy: i64,
}

pub fn solve() {
    let input: String = utils::parse_file("input/10.txt");
    let points = parse(&input);
    let (points, time) = solve_points(&points);

    println!("Problem 10");
    println!("\tFirst part:",);
    print(&points);
    println!("\tSecond part: {}", time);
}

fn is_num(chr: u8) -> bool {
    return nom::is_digit(chr) || nom::is_space(chr) || chr == b'-';
}

named!(number(&[u8]) -> &[u8], take_while!(is_num));
named!(tuple(&[u8]) -> (&[u8], &[u8]), separated_pair!(number, tag!(", "), number));
named!(coord(&[u8]) -> (&[u8], &[u8]), delimited!(char!('<'), tuple, char!('>')));
named!(position(&[u8]) -> (&[u8], &[u8]), preceded!(tag!("position="), coord));
named!(velocity(&[u8]) -> (&[u8], &[u8]), preceded!(tag!("velocity="), coord));
named!(light(&[u8]) -> ((&[u8], &[u8]), (&[u8], &[u8])), separated_pair!(position, char!(' '), velocity));

fn parse(content: &str) -> Vec<Point> {
    let mut points: Vec<Point> = Vec::new();
    for line in content.lines() {
        if let Some((_, a)) = light(line.trim().as_bytes()).ok() {
            let ((x, y), (dx, dy)) = a;
            let x: i64 = str::from_utf8(x).unwrap().trim().parse().unwrap();
            let y: i64 = str::from_utf8(y).unwrap().trim().parse().unwrap();
            let dx: i64 = str::from_utf8(dx).unwrap().trim().parse().unwrap();
            let dy: i64 = str::from_utf8(dy).unwrap().trim().parse().unwrap();
            points.push(Point {
                x: x,
                y: y,
                dx: dx,
                dy: dy,
            })
        }
    }
    return points;
}

fn step(points: &mut Vec<Point>) {
    for point in points {
        point.x += point.dx;
        point.y += point.dy;
    }
}

fn back_step(points: &mut Vec<Point>) {
    for point in points {
        point.x -= point.dx;
        point.y -= point.dy;
    }
}

fn dimension(points: &Vec<Point>) -> (i64, i64, i64, i64) {
    let min_x: i64 = points.iter().map(|p| p.x).min().unwrap();
    let max_x: i64 = points.iter().map(|p| p.x).max().unwrap();
    let min_y: i64 = points.iter().map(|p| p.y).min().unwrap();
    let max_y: i64 = points.iter().map(|p| p.y).max().unwrap();
    let width = max_y - min_y + 1;
    let height = max_x - min_x + 1;
    return (width, height, min_x, min_y);
}

fn entropy(points: &Vec<Point>) -> i64 {
    let (w, h, _, _) = dimension(points);
    return w * h;
}

fn print(points: &Vec<Point>) {
    let (width, height, shift_x, shift_y) = dimension(&points);
    for j in 0..width {
        print!("            ");
        for i in 0..height {
            if points
                .iter()
                .any(|p| p.x == shift_x + i && p.y == shift_y + j)
            {
                print!("#")
            } else {
                print!(" ")
            };
        }
        println!("");
    }
    println!("");
}

fn solve_points(points: &Vec<Point>) -> (Vec<Point>, i64) {
    let mut points = points.clone();
    let mut last_entropy: i64 = entropy(&points) + 1;
    let mut i: i64 = 0;
    while entropy(&points) < last_entropy {
        last_entropy = entropy(&points);
        step(&mut points);
        i += 1;
    }
    back_step(&mut points);
    return (points, i - 1);
}

#[cfg(test)]
mod tests {

    use super::{parse, solve_points};

    #[test]
    fn tenth_problem() {
        let points = parse("position=< 9,  1> velocity=< 0,  2>\nposition=< 7,  0> velocity=<-1,  0>\nposition=< 3, -2> velocity=<-1,  1>\nposition=< 6, 10> velocity=<-2, -1>\nposition=< 2, -4> velocity=< 2,  2>\nposition=<-6, 10> velocity=< 2, -2>\nposition=< 1,  8> velocity=< 1, -1>\nposition=< 1,  7> velocity=< 1,  0>\nposition=<-3, 11> velocity=< 1, -2>\nposition=< 7,  6> velocity=<-1, -1>\nposition=<-2,  3> velocity=< 1,  0>\nposition=<-4,  3> velocity=< 2,  0>\nposition=<10, -3> velocity=<-1,  1>\nposition=< 5, 11> velocity=< 1, -2>\nposition=< 4,  7> velocity=< 0, -1>\nposition=< 8, -2> velocity=< 0,  1>\nposition=<15,  0> velocity=<-2,  0>\nposition=< 1,  6> velocity=< 1,  0>\nposition=< 8,  9> velocity=< 0, -1>\nposition=< 3,  3> velocity=<-1,  1>\nposition=< 0,  5> velocity=< 0, -1>\nposition=<-2,  2> velocity=< 2,  0>\nposition=< 5, -2> velocity=< 1,  2>\nposition=< 1,  4> velocity=< 2,  1>\nposition=<-2,  7> velocity=< 2, -2>\nposition=< 3,  6> velocity=<-1, -1>\nposition=< 5,  0> velocity=< 1,  0>\nposition=<-6,  0> velocity=< 2,  0>\nposition=< 5,  9> velocity=< 1, -2>\nposition=<14,  7> velocity=<-2,  0>\nposition=<-3,  6> velocity=< 2, -1>\n");
        assert_eq!(solve_points(&points).1, 3);
    }
}
