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
        let items = lines
            .into_iter()
            .map(|line| {
                line.ok().and_then(|line| {
                    line.split(',')
                        .map(|side| {
                            side.split('-')
                                .map(|s| s.parse::<u32>().unwrap())
                                .collect_tuple::<(u32, u32)>()
                        })
                        .collect_tuple::<(_, _)>()
                        .and_then(|x| Option::zip(x.0, x.1))
                        .map(|(r1, r2)| {
                            (r1.0 <= r2.0 && r1.1 >= r2.1) || (r2.0 <= r1.0 && r2.1 >= r1.1)
                        })
                        .and_then(|x| x.then(|| true))
                })
            })
            .count();

        println!("{:?}", items);
    }
}

fn main2() {
    if let Ok(lines) = read_lines("input.txt") {
        let items = lines
            .into_iter()
            .filter_map(|line| {
                line.ok().and_then(|line| {
                    line.split(',')
                        .map(|side| {
                            side.split('-')
                                .map(|s| s.parse::<u32>().unwrap())
                                .collect_tuple::<(u32, u32)>()
                        })
                        .collect_tuple::<(_, _)>()
                        .and_then(|x| Option::zip(x.0, x.1))
                        .map(|(r1, r2)| r1.0 <= r2.1 && r2.0 <= r1.1)
                        .and_then(|x| x.then(|| true))
                })
            })
            .count();

        println!("{:?}", items);
    }
}

fn main() {
    main1();
    main2();
}
