use itertools::EitherOrBoth::{Both, Left, Right};
use itertools::Itertools;
use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::fs::File;
use std::io::{BufRead, BufReader};

static FILENAME: &str = "data/day13";

#[derive(Clone)]
enum ItemType {
    List(Vec<ItemType>),
    Value(u64),
}

pub fn task1() {
    let index_sum: usize = load_packets()
        .iter()
        .tuples()
        .enumerate()
        .filter_map(|(idx, (left, right))| {
            if compare_items(&left, &right) == Less {
                Some(idx + 1)
            } else {
                None
            }
        })
        .sum();
    println!("13.1: {}", index_sum);
}

pub fn task2() {
    let mut packets = load_packets();
    let divider_1 = ItemType::List([ItemType::List([ItemType::Value(2)].to_vec())].to_vec());
    let divider_2 = ItemType::List([ItemType::List([ItemType::Value(6)].to_vec())].to_vec());
    packets.push(divider_1.clone());
    packets.push(divider_2.clone());
    packets.sort_by(compare_items);

    let decoder_key: usize = packets
        .iter()
        .enumerate()
        .filter_map(|(idx, packet)| {
            if compare_items(packet, &divider_1) == Equal
                || compare_items(packet, &divider_2) == Equal
            {
                Some(idx + 1)
            } else {
                None
            }
        })
        .product();
    println!("13.2: {}", decoder_key);
}

fn load_packets() -> Vec<ItemType> {
    let reader = BufReader::new(File::open(FILENAME).unwrap());
    reader
        .lines()
        .filter_map(|line| {
            let line = line.unwrap();
            if line.len() > 0 {
                return Some(parse_packet(&line));
            } else {
                return None;
            }
        })
        .collect()
}

fn compare_items(left: &ItemType, right: &ItemType) -> Ordering {
    match (left, right) {
        (ItemType::Value(lval), ItemType::Value(rval)) => lval.cmp(&rval),
        (ItemType::List(litems), ItemType::List(ritems)) => compare_item_slices(&litems, &ritems),
        (ItemType::Value(lval), ItemType::List(ritems)) => {
            compare_item_slices(&[ItemType::Value(*lval)], &ritems)
        }
        (ItemType::List(litems), ItemType::Value(rval)) => {
            compare_item_slices(&litems, &[ItemType::Value(*rval)])
        }
    }
}

fn compare_item_slices(left: &[ItemType], right: &[ItemType]) -> Ordering {
    for val in left.iter().zip_longest(right) {
        match val {
            Both(litem, ritem) => {
                let result = compare_items(litem, ritem);
                if result != Equal {
                    return result;
                }
            }
            Left(_) => return Greater,
            Right(_) => return Less,
        }
    }
    Equal
}

fn parse_packet(line: &str) -> ItemType {
    let mut stack = Vec::<Vec<ItemType>>::new();
    let mut tmp = String::new();
    for ch in line.chars() {
        if ch == '[' {
            stack.push(Vec::new());
        } else if ch == ']' || ch == ',' {
            if tmp.len() > 0 {
                stack
                    .last_mut()
                    .unwrap()
                    .push(ItemType::Value(tmp.parse::<u64>().unwrap()));
                tmp.clear();
            }
            if ch == ']' {
                let item = ItemType::List(stack.pop().unwrap());
                if let Some(_last) = stack.last() {
                    stack.last_mut().unwrap().push(item);
                } else {
                    return item;
                }
            }
        } else {
            tmp.push(ch);
        }
    }
    unreachable!();
}
