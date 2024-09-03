#![forbid(unsafe_code)]

use std::collections::HashSet;
use std::{fs::File, io::BufRead, io::BufReader};

fn save_lines(file: File) -> HashSet<String> {
    let mut file_lines = HashSet::new();

    let reader = BufReader::new(file);
    for line in reader.lines() {
        file_lines.insert(line.unwrap());
    }

    file_lines
}

fn find_duplicate_lines(file: File, lines_set: &mut HashSet<String>) {
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let current_line = line.unwrap();

        if lines_set.contains(&current_line) {
            lines_set.take(&current_line).unwrap();
            println!("{current_line}");
        }
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let file = File::open(&args[1]).unwrap();
    let mut first_file_lines = save_lines(file);

    let file = File::open(&args[2]).unwrap();
    find_duplicate_lines(file, &mut first_file_lines);
}
