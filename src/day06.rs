use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp::min;

static FILENAME: &str = "data/day06";

pub fn task1() {
    println!("Day 6, task 1");
    println!("Position: {}", get_start_position(4));
}

pub fn task2() {
    println!("Day 6, task 2");
    println!("Position: {}", get_start_position(14));
}

fn get_start_position(num_unique: usize) -> usize {
    let mut reader = BufReader::new(File::open(FILENAME).unwrap());
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let mut buffer = VecDeque::<char>::new();
    let mut duplicate_buffer = Vec::<char>::new();
    line.chars()
        .enumerate()
        .filter_map(|(idx, ch)| {
            buffer.push_front(ch);
            buffer.resize_with(min(buffer.len(), num_unique), || unreachable!());
            duplicate_buffer.clear();
            let num_unique_found = buffer.iter().map(|ch| {
                if duplicate_buffer.contains(ch) {
                    false
                } else {
                    duplicate_buffer.push(*ch);
                    true
                }
            }).take_while(|x| *x).count();
            if num_unique_found == num_unique {
                Some(idx + 1)
            } else {
                None
            }
        })
        .nth(0)
        .unwrap()
}
