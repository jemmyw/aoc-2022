use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let mut instructions = BufReader::new(file).lines().map(|line| line.unwrap());

    let mut signal_strength = vec![];
    let mut x: i32 = 1;

    let mut increment = |x| {
        let cycle = signal_strength.len() as i32 + 1;
        println!("{}, {} = {}", cycle, x, cycle * x);
        signal_strength.push(x * cycle);
    };

    loop {
        let instruction = instructions.next().map(|str| {
            let mut s = str.split(" ");
            let i = s.next().unwrap();
            let a: i32 = s.next().and_then(|a| a.parse().ok()).unwrap_or(0);
            (i.to_owned(), a)
        });

        if let Some((i, a)) = instruction {
            match i.as_str() {
                "noop" => increment(x),
                "addx" => {
                    increment(x);
                    increment(x);
                    x += a;
                }
                _ => {
                    break;
                }
            }
        } else {
            break;
        }
    }

    increment(x);

    let cycles = [20, 60, 100, 140, 180, 220];

    // Output the signal strength for cycles 20, 60, 100, 140, 180, and 220
    for i in cycles.iter() {
        println!(
            "Signal strength at cycle {}: {}",
            i,
            signal_strength[(*i) - 1]
        );
    }

    let sum: i32 = cycles.iter().map(|i| signal_strength[(*i) - 1]).sum();
    println!("Sum: {}", sum);

    Ok(())
}
