use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

static FILENAME: &str = "data/day04";

pub fn task1() {
    let overlaps = check_overlaps(|((a, b), (c, d))| (a <= c && b >= d) || (c <= a && d >= b));
    println!("Day 4, task 1");
    println!("Total number of overlaps: {}", overlaps);
}

pub fn task2() {
    let overlaps = check_overlaps(|((a, b), (c, d))| {
        (a >= c && a <= d) || (b >= c && b <= d) || (c >= a && c <= b) || (d >= a && d <= b)
    });
    println!("Day 4, task 2");
    println!("Total number of overlaps: {}", overlaps);
}

fn check_overlaps<F>(have_overlap: F) -> usize
where
    F: Fn(((usize, usize), (usize, usize))) -> bool,
{
    let reader = BufReader::new(File::open(FILENAME).unwrap());
    reader
        .lines()
        .map(|line| {
            let ranges = line
                .unwrap()
                .split(',')
                .filter_map(|part| {
                    part.split('-')
                        .map(|num_str| num_str.parse().unwrap())
                        .collect_tuple::<(usize, usize)>()
                })
                .collect_tuple::<((usize, usize), (usize, usize))>()
                .unwrap();
            have_overlap(ranges)
        })
        .filter(|x| *x)
        .count()
}
