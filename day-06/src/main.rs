use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

fn look_for_chars(n: usize) -> Option<usize> {
    // Open the input file
    let mut file = File::open("input.txt").unwrap();

    // Read the contents of the file into a string
    let mut input = String::new();
    file.read_to_string(&mut input).unwrap();

    // Iterate over the characters in the input string
    for (i, _) in input.chars().enumerate() {
        // Stop the loop if there are not enough characters left to form a set of four
        if i + n > input.len() {
            break;
        }

        // Check the next three characters to see if they are unique
        let unique = input[i..i + n].chars().collect::<HashSet<_>>().len() == n;

        // If all four characters are unique, output the index of the final character
        // and exit the program
        if unique {
            return Some(i + n);
        }
    }

    return None;
}

fn main() {
    println!("{:?}", look_for_chars(4));
    println!("{:?}", look_for_chars(14));
}
