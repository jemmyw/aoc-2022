use std::collections::HashSet;
use std::fs;
use std::io;
use std::io::BufRead;

type Grid = Vec<Vec<u32>>;

fn count_edge_lines(grid: &Grid) -> usize {
    let mut positions_seen = HashSet::new();

    let c_range = 1..grid[0].len() - 1;
    let r_range = 1..grid.len() - 1;

    for c in c_range.clone() {
        let mut h = grid[0][c];
        for r in r_range.clone() {
            if grid[r][c] > h {
                positions_seen.insert((c, r));
                h = grid[r][c];
            }
        }
        let mut h = grid[grid.len() - 1][c];
        for r in r_range.clone().rev() {
            if grid[r][c] > h {
                positions_seen.insert((c, r));
                h = grid[r][c];
            }
        }
    }
    for r in r_range {
        let mut h = grid[r][0];
        for c in c_range.clone() {
            if grid[r][c] > h {
                positions_seen.insert((c, r));
                h = grid[r][c];
            }
        }
        h = grid[r][grid[0].len() - 1];
        for c in c_range.clone().rev() {
            if grid[r][c] > h {
                positions_seen.insert((c, r));
                h = grid[r][c];
            }
        }
    }

    return positions_seen.len() + (grid.len() * 4 - 4);
}

// Load the grid from the file "input.txt"
fn load_grid() -> Result<Grid, io::Error> {
    let mut grid = Vec::new();

    let file = fs::File::open("input.txt")?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let row = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
        grid.push(row);
    }

    Ok(grid)
}

// Test the implementation with the example grid
fn part_1(grid: &Grid) {
    let count = count_edge_lines(&grid);
    println!(
        "Number of positions with a straight line to an edge: {}",
        count
    );
}

fn scenic_score(grid: &Grid, c: usize, r: usize) -> u32 {
    let directions = [(0, -1), (0, 1), (1, 0), (-1, 0)];
    let mut scenic_score = 1;

    for (dx, dy) in directions.iter() {
        let mut count = 0;
        for d in 1.. {
            let x = c as i32 + d * dx;
            let y = r as i32 + d * dy;

            if x < 0 || x >= grid[0].len() as i32 || y < 0 || y >= grid.len() as i32 {
                break;
            }
            count += 1;

            if grid[y as usize][x as usize] >= grid[r][c] {
                break;
            }
        }
        scenic_score *= count;
    }

    scenic_score
}

fn part_2(grid: &Grid) {
    let scores = (0..grid[0].len()).into_iter().flat_map(|c| {
        (0..grid.len())
            .into_iter()
            .map(move |r| scenic_score(grid, c, r))
    });

    println!("{:?}", scores.max());
}

fn main() {
    let grid = load_grid().expect("Failed to load grid from file");
    part_1(&grid);
    part_2(&grid);
}
