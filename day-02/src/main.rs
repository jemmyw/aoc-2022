use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use itertools::Itertools;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main1() {
    if let Ok(lines) = read_lines("input.txt") {
        let score: u32 = lines
            .into_iter()
            .filter_map(|line| {
                line.ok()
                    .unwrap()
                    .split_ascii_whitespace()
                    .map(|s| s.chars().next().unwrap())
                    .collect_tuple::<(char, char)>()
            })
            .map(|t| {
                let won_score = match t {
                    ('A', 'X') => 3,
                    ('A', 'Y') => 6,
                    ('A', 'Z') => 0,
                    ('B', 'X') => 0,
                    ('B', 'Y') => 3,
                    ('B', 'Z') => 6,
                    ('C', 'X') => 6,
                    ('C', 'Y') => 0,
                    ('C', 'Z') => 3,
                    _ => 0,
                };
                let item_score = match t.1 {
                    'X' => 1,
                    'Y' => 2,
                    'Z' => 3,
                    _ => 0,
                };

                won_score + item_score
            })
            .sum();

        println!("{:?}", score);
    }
}

fn main2() {
    if let Ok(lines) = read_lines("input.txt") {
        let score: u32 = lines
            .into_iter()
            .filter_map(|line| {
                line.ok()
                    .unwrap()
                    .split_ascii_whitespace()
                    .map(|s| s.chars().next().unwrap())
                    .collect_tuple::<(char, char)>()
            })
            .map(|t| {
                let won_score = match t.1 {
                    'X' => 0,
                    'Y' => 3,
                    'Z' => 6,
                    _ => 0,
                };

                let item_score = match (t.0, won_score) {
                    ('A', 0) => 3,
                    ('A', 3) => 1,
                    ('A', 6) => 2,
                    ('B', 0) => 1,
                    ('B', 3) => 2,
                    ('B', 6) => 3,
                    ('C', 0) => 2,
                    ('C', 3) => 3,
                    ('C', 6) => 1,
                    _ => 0,
                };

                won_score + item_score
            })
            .sum();

        println!("{:?}", score);
    }
}

fn main() {
    main1();
    main2();
}
