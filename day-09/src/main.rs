use itertools::*;
use std::collections::HashSet;
use std::io::BufRead;
use std::ops;
use std::{fs, io};

type HT = (Position, Position);

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self { x: x, y: y }
    }
}

impl ops::AddAssign<Position> for Position {
    fn add_assign(&mut self, rhs: Position) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::Sub<Position> for Position {
    type Output = Position;

    fn sub(self, rhs: Position) -> Self::Output {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl ops::SubAssign<Position> for Position {
    fn sub_assign(&mut self, rhs: Position) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

fn valid_ht(h: &Position, t: &Position) -> bool {
    let n = h.clone() - t.clone();
    return (-1..=1).contains(&n.x) && (-1..=1).contains(&n.y);
}

fn make_ht_valid(h: &Position, t: &mut Position) {
    if valid_ht(h, t) {
        return;
    }

    let n = h.clone() - t.clone();
    if n.x == 0 {
        t.y += n.y / n.y.abs();
    } else if n.y == 0 {
        t.x += n.x / n.x.abs();
    } else {
        t.y += n.y / n.y.abs();
        t.x += n.x / n.x.abs();
    }
}

fn draw_grid(ht: &HT) {
    for r in -20..20 {
        for c in -20..20 {
            let p = Position::new(c, r);
            if ht.0 == p {
                print!("H");
            } else if ht.1 == p {
                print!("T");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}

fn load_instructions() -> Vec<Position> {
    let file = fs::File::open("input.txt").unwrap();
    let reader = io::BufReader::new(file);

    let instructions: Vec<Position> = reader
        .lines()
        .filter_map(|line| {
            line.ok().and_then(|line| {
                let mut s = line.split(" ");

                let d = match s.next() {
                    Some("U") => Position::new(0, -1),
                    Some("R") => Position::new(1, 0),
                    Some("D") => Position::new(0, 1),
                    Some("L") => Position::new(-1, 0),
                    _ => return None,
                };

                s.next().and_then(|n| n.parse::<u32>().ok()).map(|n| (d, n))
            })
        })
        .flat_map(|(d, n)| (0..n).map(move |_| d.clone()))
        .collect();

    return instructions;
}

fn part_1() {
    let instructions = load_instructions();

    let mut visited = HashSet::new();
    let mut ht = (Position::new(0, 0), Position::new(0, 0));

    for instruction in instructions {
        ht.0 += instruction;
        make_ht_valid(&ht.0, &mut ht.1);
        visited.insert(ht.1.clone());
    }

    println!("Visited positions {:?}", visited.len());
}

fn part_2() {
    let instructions = load_instructions();

    let mut visited = HashSet::new();
    let mut h = Position::new(0, 0);
    let mut knots: Vec<Position> = (1..=9).map(|_| Position::new(0, 0)).collect();

    for instruction in instructions {
        h += instruction;

        make_ht_valid(&h, knots.get_mut(0).unwrap());

        for (a, b) in (0..=8).tuple_windows::<(usize, usize)>() {
            let an = knots.get(a).unwrap().clone();
            let bn = knots.get_mut(b).unwrap();
            make_ht_valid(&an, bn);
        }

        let last = knots.last().unwrap();
        visited.insert(last.clone());
    }

    println!("Visited positions {:?}", visited.len());
}

fn main() {
    part_1();
    part_2();
}
