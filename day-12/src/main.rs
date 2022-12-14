use pathfinding::prelude::dijkstra;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    // Read the contents of the file
    let contents = fs::read_to_string("input.txt")?;

    // Split the contents into lines
    let lines = contents.split("\n").collect::<Vec<&str>>();

    // Create the grid structure
    let mut grid = Vec::new();
    for line in lines {
        let row = line.chars().collect::<Vec<char>>();
        grid.push(row);
    }

    let mut start = (0, 0);
    let mut goal = (0, 0);

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 'S' {
                start = (x, y);
            }
            if grid[y][x] == 'E' {
                goal = (x, y);
            }
        }
    }

    let directions: Vec<(i32, i32)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

    let result = dijkstra(
        &start,
        |&(x, y)| {
            let mut successors = vec![];
            let c = (grid[y][x] as i32) + 1;

            for (xa, ya) in &directions {
                let xb = x as i32 + *xa;
                let yb = y as i32 + *ya;

                if yb >= 0 && yb < grid.len() as i32 && xb >= 0 && xb < grid[0].len() as i32 {
                    let f = grid[yb as usize][xb as usize];
                    let mut fc: i32 = 0;
                    if f == 'E' {
                        fc = 'z' as i32
                    } else {
                        fc = f as i32;
                    }

                    if grid[y][x] == 'S' || fc <= c {
                        successors.push(((xb as usize, yb as usize), 2))
                    }
                }
            }

            successors
        },
        |&p| p == goal,
    );

    println!("{:?}", result.map(|r| r.0.len() - 1));

    Ok(())
}
