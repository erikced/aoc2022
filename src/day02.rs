use std::fs::File;
use std::io::{BufRead, BufReader};

static FILENAME: &str = "data/day02";

pub fn task1() {
    let reader = BufReader::new(File::open(FILENAME).unwrap());
    let score = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let shape_score = (line.as_bytes()[2] as i64) - ('X' as i64) + 1;
            let outcome_score = match (line.as_bytes()[0] as char, line.as_bytes()[2] as char) {
                ('A', 'X') | ('B', 'Y') | ('C', 'Z') => 3,
                ('A', 'Y') | ('B', 'Z') | ('C', 'X') => 6,
                _ => 0,
            };
            shape_score + outcome_score
        })
        .fold(0, |tot, x| tot + x);
    println!("02.1: {}", score);
}

pub fn task2() {
    let reader = BufReader::new(File::open(FILENAME).unwrap());
    let score = reader
        .lines()
        .map(|line| line.unwrap())
        .map(
            |line| match (line.as_bytes()[0] as char, line.as_bytes()[2] as char) {
                ('A', 'X') => 3,
                ('B', 'X') => 1,
                ('C', 'X') => 2,
                ('A', 'Y') => 1 + 3,
                ('B', 'Y') => 2 + 3,
                ('C', 'Y') => 3 + 3,
                ('A', 'Z') => 2 + 6,
                ('B', 'Z') => 3 + 6,
                ('C', 'Z') => 1 + 6,
                _ => panic!(),
            },
        )
        .fold(0, |tot, x| tot + x);
    println!("02.2: {}", score);
}
