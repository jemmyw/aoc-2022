use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn part_1() -> usize {
    let file_name = Path::new("input.txt");

    // Read the file into a vector of strings, one string per line
    let lines: Vec<String> = fs::read_to_string(file_name)
        .unwrap()
        .split('\n')
        .map(|s| s.to_string())
        .collect();

    // Create a map to store the sizes of the containers
    let mut container_sizes: HashMap<String, usize> = HashMap::new();

    // Create a stack to keep track of the current container
    let mut container_stack: Vec<String> = vec![];

    let mut current_container = &"".to_owned();
    container_stack.push(current_container.clone());

    // Iterate through each line of the input file
    for line in lines {
        // Split the line into tokens
        let tokens: Vec<&str> = line.split_whitespace().collect();

        // If the line is a command line, process it
        if tokens[0].starts_with('$') {
            let command = tokens[1];
            match command {
                "ls" => {
                    // If the current container is not empty, update its size
                    if let Some(current_container) = container_stack.last() {
                        container_sizes.insert(current_container.to_string(), 0);
                    }
                }
                "cd" => {
                    match tokens[2] {
                        ".." => {
                            // If the new container is "..", pop the current container from the stack
                            container_stack.pop();
                        }
                        _ => {
                            // Otherwise, push the new container onto the stack
                            let new_container = format!("{}/{}", current_container, tokens[2]);
                            container_stack.push(new_container.to_string());
                        }
                    }
                    current_container = container_stack.last().unwrap();
                }
                _ => {}
            }
        } else {
            // If the line is a data line, process it
            let first_token = tokens[0];
            match first_token {
                "dir" => {
                    // If the first token is "dir", it is a container. Add it to the map with size 0.
                    let container_name = format!("{}/{}", current_container, tokens[1]);
                    container_sizes.insert(container_name.to_string(), 0);
                }
                _ => {
                    // Otherwise, it is a sized object. Update the size of the current container.
                    let size: usize = first_token.parse().unwrap();

                    for stack_container in container_stack.iter() {
                        if let Some(container) = container_sizes.get_mut(stack_container) {
                            *container += size;
                        }
                    }
                }
            }
        }
    }

    // Return a vector of the names of the containers that have a size of at most 100,000
    container_sizes
        .into_iter()
        .filter(|(_, size)| *size <= 100_000)
        .map(|(_, size)| size)
        .sum()
}

fn part_2() -> () {
    let file_name = Path::new("input.txt");

    // Read the file into a vector of strings, one string per line
    let lines: Vec<String> = fs::read_to_string(file_name)
        .unwrap()
        .split('\n')
        .map(|s| s.to_string())
        .collect();

    // Create a map to store the sizes of the containers
    let mut container_sizes: HashMap<String, usize> = HashMap::new();

    // Create a stack to keep track of the current container
    let mut container_stack: Vec<String> = vec![];

    let unnamed = &"".to_owned();
    let mut current_container = unnamed;
    container_stack.push(unnamed.clone());
    container_sizes.insert(unnamed.clone(), 0);

    // Iterate through each line of the input file
    for line in lines {
        // Split the line into tokens
        let tokens: Vec<&str> = line.split_whitespace().collect();

        // If the line is a command line, process it
        if tokens[0].starts_with('$') {
            let command = tokens[1];
            match command {
                "ls" => {
                    // If the current container is not empty, update its size
                    if let Some(current_container) = container_stack.last() {
                        container_sizes.insert(current_container.to_string(), 0);
                    }
                }
                "cd" => {
                    match tokens[2] {
                        ".." => {
                            // If the new container is "..", pop the current container from the stack
                            container_stack.pop();
                        }
                        _ => {
                            // Otherwise, push the new container onto the stack
                            let new_container = format!("{}/{}", current_container, tokens[2]);
                            container_stack.push(new_container.to_string());
                        }
                    }
                    current_container = container_stack.last().unwrap();
                }
                _ => {}
            }
        } else {
            // If the line is a data line, process it
            let first_token = tokens[0];
            match first_token {
                "dir" => {
                    // If the first token is "dir", it is a container. Add it to the map with size 0.
                    let container_name = format!("{}/{}", current_container, tokens[1]);
                    container_sizes.insert(container_name.to_string(), 0);
                }
                _ => {
                    // Otherwise, it is a sized object. Update the size of the current container.
                    let size: usize = first_token.parse().unwrap();

                    for stack_container in container_stack.iter() {
                        if let Some(container) = container_sizes.get_mut(stack_container) {
                            *container += size;
                        }
                    }
                }
            }
        }
    }

    let total_used = *container_sizes.get(&unnamed.clone()).unwrap();
    let free_space = 70000000 - total_used;
    let to_delete = 30000000 - free_space;

    println!("Total used: {:?}", total_used);
    println!("Free space: {:?}", free_space);
    println!("Min to delete: {:?}", to_delete);

    let mut sizes: Vec<&usize> = container_sizes.values().collect();
    sizes.sort_unstable();
    let size = sizes.into_iter().find(|&&s| s >= to_delete);
    println!("{:?}", size);
}

fn main() {
    println!("{:?}", part_1());
    part_2();
}
