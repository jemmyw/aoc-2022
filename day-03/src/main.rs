use std::collections::HashSet;
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

fn char_to_score(c: &char) -> u64 {
    if c.is_uppercase() {
        *c as u64 - 64 + 26
    } else {
        *c as u64 - 96
    }
}

fn main1() {
    if let Ok(lines) = read_lines("input.txt") {
        let items: u64 = lines
            .into_iter()
            .flat_map(|line| {
                let s = line.unwrap();
                let mut h1 = HashSet::new();
                let mut h2 = HashSet::new();

                for c in s.chars().take(s.len() / 2) {
                    // h1.insert(c);
                    h1.insert(char_to_score(&c));
                }
                for c in s.chars().skip(s.len() / 2) {
                    // h2.insert(c);
                    h2.insert(char_to_score(&c));
                }

                return h1.intersection(&h2).cloned().collect::<Vec<_>>();
            })
            .sum();

        println!("{:?}", items);
    }
}

fn main2() {
    if let Ok(lines) = read_lines("input.txt") {
        let items: u64 = lines
            .into_iter()
            .map(|line| line.unwrap())
            .chunks(3)
            .into_iter()
            .map(|group| {
                char_to_score(
                    &group
                        .map(|s| s.chars().collect::<HashSet<char>>())
                        .reduce(|acc, hs| acc.intersection(&hs).cloned().collect())
                        .unwrap()
                        .into_iter()
                        .next()
                        .unwrap(),
                )
            })
            .sum();

        println!("{:?}", items);
    }
}

fn main() {
    main1();
    main2();
}
