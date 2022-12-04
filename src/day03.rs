use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

static FILENAME: &str = "data/day03";

pub fn task1() {
    let reader = BufReader::new(File::open(FILENAME).unwrap());
    let priority = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| calculate_priority(&line))
        .fold(0, |tot, x| x + tot);
    println!("Day 3, task 1");
    println!("Total priority: {}", priority);
}

fn calculate_priority(s: &str) -> i64 {
    let left: HashSet<char> = s[..s.len() / 2].chars().collect();
    let right: HashSet<char> = s[s.len() / 2..].chars().collect();
    let v = left
        .intersection(&right)
        .map(|x| get_item_priority(*x))
        .fold(0, |tot, x| x + tot);
    return v;
}

fn get_item_priority(c: char) -> i64 {
    if 'a' <= c && c <= 'z' {
        c as i64 - 'a' as i64 + 1
    } else {
        c as i64 - 'A' as i64 + 27
    }
}

pub fn task2() {
    let reader = BufReader::new(File::open(FILENAME).unwrap());
    let priority = reader
        .lines()
        .map(|l| l.unwrap())
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            get_item_priority(
                chunk
                    .into_iter()
                    .map(|line| line.chars().collect::<HashSet<char>>())
                    .reduce(|a, b| a.intersection(&b).copied().collect::<HashSet<char>>())
                    .unwrap()
                    .into_iter()
                    .nth(0)
                    .unwrap(),
            )
        })
        .fold(0, |tot, x| tot + x);
    println!("Day 3, task 2");
    println!("Total priority: {}", priority);
}
