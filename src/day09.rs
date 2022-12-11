use itertools::Itertools;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;

static FILENAME: &str = "data/day09";

pub fn task1() {
    println!("09.1: {:?}", get_num_tail_positions(2));
}

pub fn task2() {
    println!("09.2: {:?}", get_num_tail_positions(10));
}

pub fn load_motions() -> Vec<(char, i64)> {
    let reader = BufReader::new(File::open(FILENAME).unwrap());
    reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let (d, s) = line.split(" ").collect_tuple().unwrap();
            (d.chars().nth(0).unwrap(), s.parse::<i64>().unwrap())
        })
        .collect()
}

pub fn get_num_tail_positions(num_knots: usize) -> usize {
    let mut tail_positions = BTreeSet::<(i64, i64)>::new();
    let mut knots: Vec<(i64, i64)> = Vec::from_iter(iter::repeat((0i64, 0i64)).take(num_knots));
    for (direction, steps) in load_motions().into_iter() {
        let (dx, dy) = match direction {
            'U' => (0, 1),
            'D' => (0, -1),
            'R' => (1, 0),
            'L' => (-1, 0),
            _ => unreachable!(),
        };
        for _ in 0..steps {
            knots[0] = (knots[0].0 + dx, knots[0].1 + dy);
            for i in 1..knots.len() {
                let head = knots[i - 1];
                let tail = knots[i];
                if (head.0 - tail.0).abs() > 1 || (head.1 - tail.1).abs() > 1 {
                    knots[i] = (get_new_pos(head.0, tail.0), get_new_pos(head.1, tail.1));
                }
            }
            tail_positions.insert(knots[knots.len() - 1]);
        }
    }
    return tail_positions.len();
}

fn get_new_pos(head: i64, tail: i64) -> i64 {
    tail + if head == tail {
        0
    } else if head > tail {
        1
    } else {
        -1
    }
}
