use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

static FILENAME: &str = "data/day06";

pub fn task1() {
    println!("06.1: {}", get_start_position(4));
}

pub fn task2() {
    println!("06.2: {}", get_start_position(14));
}

fn get_start_position(num_unique: usize) -> usize {
    let mut reader = BufReader::new(File::open(FILENAME).unwrap());
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let buffer = line.chars().collect::<Vec<char>>();
    (0..buffer.len() - num_unique)
        .filter_map(|tag_start| {
            if buffer[tag_start..tag_start + num_unique]
                .iter()
                .copied()
                .collect::<BTreeSet<_>>()
                .len()
                == num_unique
            {
                Some(tag_start + num_unique)
            } else {
                None
            }
        })
        .nth(0)
        .unwrap()
}
