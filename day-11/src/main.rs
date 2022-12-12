use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;
use regex::Regex;

type Round = Vec<(i32, usize)>;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
enum Operation {
    WithValue { operand: String, value: i32 },
    OnSelf { operand: String },
}

impl Operation {
    fn from_string(input: &str) -> Option<Self> {
        let r = Regex::new(r"old (.) (old|\d+)").unwrap();
        let c = r.captures(input).unwrap();

        c.get(1).map(|op| op.as_str()).and_then(|op| {
            c.get(2).map(|v| v.as_str()).and_then(|v| match v {
                "old" => Some(Operation::OnSelf {
                    operand: op.to_string(),
                }),
                v => v.parse().ok().map(|v| Operation::WithValue {
                    operand: op.to_string(),
                    value: v,
                }),
            })
        })
    }

    fn worry(&self, item: &i32) -> Result<i32, &'static str> {
        match self {
            Operation::OnSelf { operand } => Operation::WithValue {
                operand: operand.to_string(),
                value: *item,
            }
            .worry(item),
            Operation::WithValue { operand, value } => match operand.as_str() {
                "*" => Ok(value * item),
                "+" => Ok(value + item),
                _ => Err("Invalid operand"),
            },
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Monkey {
    id: usize,
    items: Vec<i32>,
    operation: Operation,
    test: i32,
    if_true: usize,
    if_false: usize,
}

// Implement a method to parse the rules of a monkey from a string
impl Monkey {
    fn from_lines(lines: Vec<String>) -> Option<Monkey> {
        if lines.len() < 5 {
            return None;
        }

        let id = lines[0]
            .split(" ")
            .skip(1)
            .next()
            .and_then(|str| str.trim_end_matches(":").parse().ok())?;

        // Extract the starting items
        let starting_items_str = lines[1].split(":").skip(1).next()?;

        let starting_items: Vec<i32> = starting_items_str
            .split(",")
            .map(|s| s.trim().parse())
            .collect::<Result<Vec<i32>, _>>()
            .ok()?;

        // Extract the operation
        let operation = lines[2]
            .split(":")
            .skip(1)
            .next()
            .and_then(|op| Operation::from_string(op))?;

        // Extract the test
        let test = lines[3].split(" ").last().and_then(|l| l.parse().ok())?;

        // Extract the "if true" and "if false" actions
        let if_true = lines[4].split(" ").last().and_then(|l| l.parse().ok())?;
        let if_false = lines[5].split(" ").last().and_then(|l| l.parse().ok())?;

        Some(Monkey {
            id,
            items: starting_items,
            operation,
            test,
            if_true,
            if_false,
        })
    }

    fn round(&self) -> Round {
        self.items
            .iter()
            .map(|item| {
                let worry_level = self.operation.worry(item).unwrap() / 3;

                if worry_level % self.test == 0 {
                    (worry_level, self.if_true)
                } else {
                    (worry_level, self.if_false)
                }
            })
            .collect()
    }
}

fn main() {
    // Open the input file
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    // Create a HashSet to store the monkeys
    let mut monkeys: Vec<Monkey> = vec![];
    let mut inspections: HashMap<usize, u32> = HashMap::new();

    reader
        .lines()
        .chunks(7)
        .into_iter()
        .filter_map(|chunk| {
            let lines = chunk.filter_map(|line| line.ok()).collect::<Vec<_>>();
            Monkey::from_lines(lines)
        })
        .for_each(|monkey| {
            inspections.insert(monkey.id, 0);
            monkeys.push(monkey);
        });

    for _ in 1..=20 {
        for id in 0..monkeys.len() {
            if let Some(m) = monkeys.get_mut(id) {
                if let Some(x) = inspections.get_mut(&m.id) {
                    *x += m.items.len() as u32;
                }

                let round = m.round();
                m.items = vec![];

                for (item, id) in round {
                    monkeys[id].items.push(item);
                }
            }
        }
    }

    let result = inspections
        .values()
        .sorted_unstable()
        .rev()
        .take(2)
        .product::<u32>();
    println!("{:?}", result);
}
