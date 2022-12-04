use std::fs::File;
use std::io::{BufRead, BufReader};

static FILENAME: &str = "data/day01";

pub fn task1() {
    let reader = BufReader::new(File::open(FILENAME).unwrap());

    let mut current_sum: usize = 0;
    let mut max_sum = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            if current_sum > max_sum {
                max_sum = current_sum;
            }
            current_sum = 0;
        } else {
            current_sum += line.parse::<usize>().unwrap();
        }
    }
    if current_sum > max_sum {
        max_sum = current_sum;
    }
    println!("Day 1, task 1");
    println!("Total calories: {}", max_sum);
}

pub fn task2() {
    let reader = BufReader::new(File::open(FILENAME).unwrap());
    let mut current_sum: usize = 0;
    let mut data: Vec<usize> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            data.push(current_sum);
            current_sum = 0;
        } else {
            current_sum += line.parse::<usize>().unwrap();
        }
    }
    data.sort_by(|a, b| b.cmp(a));
    let sum = data.iter().take(3).fold(0, |tot, x| tot + x);
    println!("Day 1, task 2");
    println!("Total calories: {}", sum);
}
