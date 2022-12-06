use std::collections::VecDeque;
use std::collections::BTreeSet;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

static FILENAME: &str = "data/day06";

pub fn task1() {
    let pos = load_data().chars().tuple_windows().enumerate().filter_map(|(pos, (a, b, c, d))| {
        if a != b && a != c && a != d && b != c && b != d && c != d {
            Some(pos + 4) 
        } else {
            None
        }
    }).nth(0).unwrap();
    println!("Day 6, task 1");
    println!("Position: {}", pos);
}

const N_MSG_CHARS: usize = 14;

pub fn task2() {
    let mut buffer = VecDeque::<char>::new();
    let pos = load_data().chars().enumerate().filter_map(|(idx, ch)| {
        buffer.push_back(ch);
        if buffer.len() > N_MSG_CHARS {
            buffer.pop_front();
        }
        if buffer.iter().copied().collect::<BTreeSet<_>>().len() == 14 {
            Some(idx + 1) 
        } else {
            None
        }
    }).nth(0).unwrap();
    println!("Day 6, task 2");
    println!("Position: {}", pos);
}

fn load_data() -> String {
    let mut reader = BufReader::new(File::open(FILENAME).unwrap());
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    return line
}
