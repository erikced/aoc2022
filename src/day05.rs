use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

static FILENAME: &str = "data/day05";

pub fn task1() {
    let (stacks, operations) = load_data();
    let stacks = operations
        .into_iter()
        .fold(stacks, |mut cur_stacks, (count, src, dst)| {
            for _ in 0..count {
                let val = cur_stacks[src - 1].pop().unwrap();
                cur_stacks[dst - 1].push(val);
            }
            cur_stacks
        });
    let top_items: String = stacks.into_iter().map(|stack| *stack.last().unwrap()).collect();
    println!("05.1: {}", top_items);
}

pub fn task2() {
    let (stacks, operations) = load_data();
    let mut tmp: Vec<char> = Vec::new();
    let stacks = operations
        .into_iter()
        .fold(stacks, |mut cur_stacks, (count, src, dst)| {
            let src_stack = &mut cur_stacks[src - 1];
            tmp.extend_from_slice(&src_stack[src_stack.len() - count..]);
            src_stack.resize_with(src_stack.len() - count, || unreachable!());
            let dst_stack = &mut cur_stacks[dst - 1];
            dst_stack.extend(tmp.iter());
            tmp.clear();
            cur_stacks
        });
    let top_items: String = stacks.into_iter().map(|stack| *stack.last().unwrap()).collect();
    println!("05.2: {}", top_items);
}

fn load_data() -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    let reader = BufReader::new(File::open(FILENAME).unwrap());
    let mut line_iter = reader.lines().map(|line| line.unwrap());
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for line in line_iter
        .by_ref()
        .take_while(|line| line.as_bytes()[1] as char != '1')
    {
        if stacks.len() == 0 {
            for _ in 0..((line.len() + 1) / 4) {
                stacks.push(Vec::new())
            }
        }
        let crate_iter = line.chars().skip(1).step_by(4);
        for (stack_idx, crate_id) in crate_iter
            .enumerate()
            .filter(|(_, crate_id)| *crate_id != ' ')
        {
            stacks[stack_idx].push(crate_id);
        }
    }
    for vec in &mut stacks {
        vec.reverse();
    }
    let operations = line_iter
        .skip(1)
        .map(|line| {
            line.split(" ")
                .skip(1)
                .step_by(2)
                .map(|num_str| num_str.parse().unwrap())
                .collect_tuple::<(usize, usize, usize)>()
                .unwrap()
        })
        .collect::<Vec<_>>();
    return (stacks, operations);
}
