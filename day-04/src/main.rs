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

fn main1() {
    if let Ok(lines) = read_lines("input.txt") {
        let items: u64 = lines
            .into_iter()
            .map(|line| {
                match line
                    .unwrap()
                    .split(',')
                    .map(|side| {
                        side.split('-')
                            .map(|s| s.parse::<u32>().unwrap())
                            .collect_tuple::<(u32, u32)>()
                            .unwrap()
                    })
                    .collect_tuple::<(_, _)>()
                    .map(|(r1, r2)| {
                        (r1.0 <= r2.0 && r1.1 >= r2.1) || (r2.0 <= r1.0 && r2.1 >= r1.1)
                    }) {
                    Some(true) => 1,
                    _ => 0,
                }
            })
            .sum();

        println!("{:?}", items);
    }
}

fn main2() {
    if let Ok(lines) = read_lines("input.txt") {
        let items: u64 = lines
            .into_iter()
            .map(|line| {
                match line
                    .unwrap()
                    .split(',')
                    .map(|side| {
                        side.split('-')
                            .map(|s| s.parse::<u32>().unwrap())
                            .collect_tuple::<(u32, u32)>()
                            .unwrap()
                    })
                    .collect_tuple::<(_, _)>()
                    .map(|(r1, r2)| r1.0 <= r2.1 && r2.0 <= r1.1)
                {
                    Some(true) => 1,
                    _ => 0,
                }
            })
            .sum();

        println!("{:?}", items);
    }
}

fn main() {
    main1();
    main2();
}
