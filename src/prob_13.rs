use std::cmp::Ordering;
use std::collections::HashSet;
use std::process;
use utils;

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

#[derive(Debug, Clone)]
struct Cart {
    x: usize,
    y: usize,
    direction: Direction,
    next_turn: Direction,
}

impl Ord for Cart {
    fn cmp(&self, other: &Cart) -> Ordering {
        if self.y == other.y {
            return self.x.cmp(&other.x);
        } else {
            return self.y.cmp(&other.y);
        }
    }
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Cart) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Cart {
    fn eq(&self, other: &Cart) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Cart {}

#[derive(Debug, Clone)]
enum Cell {
    Empty,
    Horizontal,
    Vertical,
    Intersection,
    Curve45,
    Curve135,
}

type Grid = Vec<Vec<Cell>>;

pub fn solve() {
    let input: String = utils::parse_file("input/13.txt");
    let (grid, carts) = parse(&input);

    println!("Problem 13");
    println!("\tFirst part:  {:?}", solve_first_part(&grid, &carts));
    println!("\tSecond part: {:?}", solve_second_part(&grid, &carts));
}

fn parse(content: &str) -> (Grid, Vec<Cart>) {
    let mut grid = Vec::new();
    let mut carts = Vec::new();
    for (i, line) in content.lines().enumerate() {
        grid.push(Vec::new());
        for (j, chr) in line.chars().enumerate() {
            let cell = match chr {
                ' ' => Cell::Empty,
                '-' => Cell::Horizontal,
                '|' => Cell::Vertical,
                '+' => Cell::Intersection,
                '/' => Cell::Curve45,
                '\\' => Cell::Curve135,
                '^' | 'v' | '<' | '>' => {
                    carts.push(Cart {
                        x: j,
                        y: i,
                        direction: match chr {
                            '^' => Direction::Up,
                            'v' => Direction::Down,
                            '<' => Direction::Left,
                            '>' => Direction::Right,
                            _ => unreachable!(),
                        },
                        next_turn: Direction::Left,
                    });
                    match chr {
                        '^' => Cell::Vertical,
                        'v' => Cell::Vertical,
                        '<' => Cell::Horizontal,
                        '>' => Cell::Horizontal,
                        _ => unreachable!(),
                    }
                }
                c => {
                    println!("Malformed input, char {}", c);
                    process::exit(1);
                }
            };
            grid[i].push(cell)
        }
    }
    return (grid, carts);
}

fn step(grid: &Grid, cart: &mut Cart) -> bool {
    // Move the cart by one
    match cart.direction {
        Direction::Up => {
            cart.y -= 1;
        }
        Direction::Down => {
            cart.y += 1;
        }
        Direction::Left => {
            cart.x -= 1;
        }
        Direction::Right => {
            cart.x += 1;
        }
    }
    // React according to the new cell
    match grid[cart.y][cart.x] {
        Cell::Empty => (),
        Cell::Horizontal => (),
        Cell::Vertical => (),
        Cell::Curve135 => match cart.direction {
            Direction::Up => cart.direction = Direction::Left,
            Direction::Right => cart.direction = Direction::Down,
            Direction::Down => cart.direction = Direction::Right,
            Direction::Left => cart.direction = Direction::Up,
        },
        Cell::Curve45 => match cart.direction {
            Direction::Up => cart.direction = Direction::Right,
            Direction::Right => cart.direction = Direction::Up,
            Direction::Down => cart.direction = Direction::Left,
            Direction::Left => cart.direction = Direction::Down,
        },
        Cell::Intersection => match cart.next_turn {
            Direction::Left => {
                cart.direction = cart.direction.turn_left();
                cart.next_turn = Direction::Up;
            }
            Direction::Right => {
                cart.direction = cart.direction.turn_right();
                cart.next_turn = Direction::Left;
            }
            _ => cart.next_turn = Direction::Right,
        },
    }
    return false;
}

fn solve_first_part(grid: &Grid, carts: &Vec<Cart>) -> (usize, usize) {
    let mut carts: Vec<Cart> = (*carts).clone();
    loop {
        carts.sort();
        for i in 0..carts.len() {
            step(grid, &mut carts[i]);
            if carts
                .iter()
                .enumerate()
                .filter(|&(p, _)| p != i)
                .any(|(_, c)| *c == carts[i])
            {
                return (carts[i].x, carts[i].y);
            }
        }
    }
}

fn solve_second_part(grid: &Grid, carts: &Vec<Cart>) -> (usize, usize) {
    let mut carts: Vec<Cart> = (*carts).clone();
    loop {
        carts.sort();
        let mut to_remove: HashSet<usize> = HashSet::new();
        for i in 0..carts.len() {
            step(grid, &mut carts[i]);
            for j in carts
                .iter()
                .enumerate()
                .filter(|&(k, _)| k != i)
                .filter(|&(_, c)| *c == carts[i])
                .map(|(i, _)| i.clone())
                .collect::<Vec<usize>>()
            {
                to_remove.insert(i);
                to_remove.insert(j);
            }
        }
        let mut to_remove: Vec<&usize> = to_remove.iter().collect();
        to_remove.sort();
        to_remove.reverse();
        for i in to_remove {
            carts.remove(*i);
        }
        if carts.len() <= 1 {
            return (carts[0].x, carts[0].y);
        }
    }
}

#[cfg(test)]
mod tests {

    use super::{parse, solve_first_part, solve_second_part};

    #[test]
    fn thirteenth_problem_first_part() {
        let (grid, carts) = parse("/->-\\        \n|   |  /----\\\n| /-+--+-\\  |\n| | |  | v  |\n\\-+-/  \\-+--/\n  \\------/   \n");
        assert_eq!(solve_first_part(&grid, &carts), (7, 3));
    }

    #[test]
    fn thirteenth_problem_second_part() {
        let (grid, carts) =
            parse("/>-<\\  \n|   |  \n| /<+-\\\n| | | v\n\\>+</ |\n  |   ^\n  \\<->/\n");
        assert_eq!(solve_second_part(&grid, &carts), (6, 4));
    }
}
