use std::fs::File;
use std::io::{BufRead, BufReader};

static FILENAME: &str = "data/day10";

pub fn task1() {
    let total_signal_strength = run_program()
        .into_iter()
        .enumerate()
        .filter_map(|(cycle, x)| {
            if [20, 60, 100, 140, 180, 220].contains(&(cycle + 1)) {
                Some(x as usize * (cycle + 1))
            } else {
                None
            }
        })
        .sum::<usize>();
    println!("10.1: {}", total_signal_strength);
}

pub fn task2() {
    let mut display = [['.'; 40]; 6];
    for (cycle, x) in run_program().into_iter().take(240).enumerate() {
        let row = cycle / 40;
        let col = cycle % 40;
        if x - 1 <= col as i64 && x + 1 >= col as i64 {
            display[row][col] = '#'
        };
    }
    println!("10.2:");
    for row in display {
        println!("{:}", row.into_iter().collect::<String>());
    }
}

pub fn run_program() -> Vec<i64> {
    let reader = BufReader::new(File::open(FILENAME).unwrap());
    let mut op_iter = reader.lines().map(|x| x.unwrap()).map(|line| {
        let data = line.split(" ").collect::<Vec<&str>>();
        if data[0] == "noop" {
            (1i64, 0i64)
        } else {
            (2, data[1].parse::<i64>().unwrap())
        }
    });
    let mut x = 1i64;
    let mut delay = 1;
    let mut op_val = 0i64;
    let mut output = Vec::<i64>::new();
    loop {
        delay -= 1;
        if delay == 0 {
            x = x + op_val;
            (delay, op_val) = match op_iter.next() {
                Some((a, b)) => (a, b),
                None => break,
            }
        }
        output.push(x);
    }
    return output;
}
