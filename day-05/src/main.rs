use itertools::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut input = Vec::new();
    let mut instructions = Vec::new();
    let mut lines = reader.lines();

    while let Some(line) = lines.next() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        let row = line
            .chars()
            .skip(1)
            .chunks(4)
            .into_iter()
            .flat_map(|c| c.take(1))
            .collect::<Vec<_>>();
        input.push(row);
    }

    while let Some(line) = lines.next() {
        let line = line.unwrap();
        if line.is_empty() || !line.starts_with("move ") {
            continue;
        }
        instructions.push(line);
    }

    let mut result = Vec::new();

    for (j, _) in input[0].iter().enumerate() {
        let mut vec = Vec::new();
        for (i, row) in input.iter().enumerate() {
            if !row[j].is_numeric() && !row[j].is_whitespace() {
                vec.push(input[i][j]);
            }
        }
        result.push(vec);
    }

    for instruction in instructions {
        let parts: Vec<&str> = instruction.split(' ').collect();
        let n = parts[1].parse::<usize>().unwrap();
        let x = parts[3].parse::<usize>().unwrap() - 1; // 1-based index
        let y = parts[5].parse::<usize>().unwrap() - 1; // 1-based index

        let mut items = result[x].drain(..n).rev().collect::<Vec<_>>();
        items.extend(result[y].clone());
        result[y] = items;

        // println!("{:?}", result);
    }

    for v in result {
        print!("{}", v[0]);
    }

    print!("\n");

    // println!("{:?}", result);
}
