use std::fs::File;
use std::io::{self, BufRead};
use std::iter;
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
        let calories = lines
            .into_iter()
            .map(|line| line.ok().and_then(|x| x.parse::<u32>().ok()))
            .chain(iter::once(None))
            .group_by(|x| x.is_some())
            .into_iter()
            .map(|(_, group)| group.filter_map(|x| x).sum::<u32>())
            .max()
            .unwrap();

        println!("{:?}", calories);
    }
}

fn main2() {
    if let Ok(lines) = read_lines("input.txt") {
        let calories: u32 = lines
            .into_iter()
            .map(|line| line.ok().and_then(|x| x.parse::<u32>().ok()))
            .chain(iter::once(None))
            .group_by(|x| x.is_some())
            .into_iter()
            .map(|(_, group)| group.filter_map(|x| x).sum::<u32>())
            .sorted_unstable()
            .rev()
            .take(3)
            .sum();

        println!("{:?}", calories);
    }
}

fn main() {
    main1();
    main2();
}
