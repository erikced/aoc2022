use std::fs::File;
use std::io::{BufRead, BufReader};
use std::mem;

static FILENAME: &str = "data/day11";

enum Operation {
    AddConstant(u64),
    MultiplyConstant(u64),
    MultiplySelf,
}

struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    divisor: u64,
    dest_t: usize,
    dest_f: usize,
}

pub fn task1() {
    let monkeys = load_data();
    let monkey_business_level = get_monkey_business_level(monkeys, 25, |val| val / 3);
    println!("11.1: {}", monkey_business_level);
}

pub fn task2() {
    let monkeys = load_data();
    let common_denominator: u64 = monkeys.iter().map(|monkey| monkey.divisor).product();
    let monkey_business_level = get_monkey_business_level(monkeys, 10000, |val| val % common_denominator);
    println!("11.2: {}", monkey_business_level);
}

fn get_monkey_business_level<F>(
    mut monkeys: Vec<Monkey>,
    rounds: usize,
    worry_level_scaler: F,
) -> u64
where
    F: Fn(u64) -> u64,
{
    let mut num_inspections: Vec<u64> = [0].repeat(monkeys.len());
    for _round in 0..rounds {
        for i in 0..monkeys.len() {
            let items = mem::take(&mut monkeys[i].items);
            for item in items {
                num_inspections[i] += 1;
                let new_value = worry_level_scaler(match monkeys[i].operation {
                    Operation::MultiplySelf => item * item,
                    Operation::MultiplyConstant(val) => item * val,
                    Operation::AddConstant(val) => item + val,
                });
                let dest_idx = if new_value % monkeys[i].divisor == 0 {
                    monkeys[i].dest_t
                } else {
                    monkeys[i].dest_f
                };
                monkeys[dest_idx].items.push(new_value);
            }
        }
    }
    num_inspections.sort_by(|a, b| b.cmp(a));
    num_inspections.into_iter().take(2).product()
}

fn load_data() -> Vec<Monkey> {
    let mut reader = BufReader::new(File::open(FILENAME).unwrap());
    let mut line = String::new();

    let mut monkeys = Vec::<Monkey>::new();
    loop {
        reader.read_line(&mut line).unwrap();
        if !line.starts_with("Monkey") {
            break;
        }
        line.clear();

        reader.read_line(&mut line).unwrap();
        line.pop();
        let items: Vec<u64> = line
            .split(": ")
            .nth(1)
            .unwrap()
            .split(", ")
            .map(|item| item.parse::<u64>().unwrap())
            .collect();
        line.clear();

        reader.read_line(&mut line).unwrap();
        line.pop();
        let op_def: Vec<&str> = line.split("= ").nth(1).unwrap().split(" ").collect();
        let operation = match (op_def[0], op_def[1], op_def[2]) {
            ("old", "*", "old") => Operation::MultiplySelf,
            ("old", "*", val) => Operation::MultiplyConstant(val.parse::<u64>().unwrap()),
            ("old", "+", val) => Operation::AddConstant(val.parse::<u64>().unwrap()),
            _ => unreachable!(),
        };
        line.clear();

        reader.read_line(&mut line).unwrap();
        line.pop();
        let divisor: u64 = line.split(" ").last().unwrap().parse().unwrap();
        line.clear();

        reader.read_line(&mut line).unwrap();
        line.pop();
        let dest_t: usize = line.split(" ").last().unwrap().parse().unwrap();
        line.clear();

        reader.read_line(&mut line).unwrap();
        line.pop();
        let dest_f: usize = line.split(" ").last().unwrap().parse().unwrap();
        line.clear();

        reader.read_line(&mut line).unwrap();
        line.clear();

        monkeys.push(Monkey {
            items,
            operation,
            divisor,
            dest_t,
            dest_f,
        });
    }
    monkeys
}
