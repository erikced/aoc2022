use ndarray::{Array, Array2};
use std::collections::BTreeSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader, Seek};
use std::mem;

static FILENAME: &str = "data/day12";

enum EndCondition {
    ReachesStart,
    ReachesHeight(char),
}

pub fn task1() {
    let steps = find_shortest_path_length(EndCondition::ReachesStart);
    println!("12.1: {:?}", steps);
}

pub fn task2() {
    let steps = find_shortest_path_length(EndCondition::ReachesHeight('a'));
    println!("12.2: {}", steps);
}

fn find_shortest_path_length(end_condition: EndCondition) -> u32 {
    let mut map = load_map();
    let mut score = Array2::<u32>::from_elem(map.raw_dim(), u32::MAX);
    let mut changed = BTreeSet::<(usize, usize)>::new();
    let start = find_value(&map, 'S');
    map[start] = 'a';
    let end = find_value(&map, 'E');
    map[end] = 'z';
    changed.insert(end);
    score[end] = 0;
    while changed.len() > 0 {
        for pos in mem::take(&mut changed) {
            let min_new_height = map[pos] as u32 - 1;
            let new_score = score[pos] + 1;
            for npos in get_neighbors(pos, map.shape())
                .into_iter()
                .filter_map(|x| x)
            {
                if score[npos] == u32::MAX && map[npos] as u32 >= min_new_height {
                    match end_condition {
                        EndCondition::ReachesStart => {
                            if npos == start {
                                return new_score;
                            }
                        }
                        EndCondition::ReachesHeight(h) => {
                            if map[npos] == h {
                                return new_score;
                            }
                        }
                    }
                    score[npos] = new_score;
                    changed.insert(npos);
                }
            }
        }
    }
    return u32::MAX;
}

fn find_value(map: &Array2<char>, ch: char) -> (usize, usize) {
    map.indexed_iter()
        .filter_map(|(pos, val)| if *val == ch { Some(pos) } else { None })
        .nth(0)
        .unwrap()
}

fn get_neighbors(pos: (usize, usize), array_size: &[usize]) -> [Option<(usize, usize)>; 4] {
    let mut neighbors = [None; 4];
    if pos.1 > 0 {
        neighbors[0] = Some((pos.0, pos.1 - 1));
    }
    if pos.1 < array_size[1] - 1 {
        neighbors[1] = Some((pos.0, pos.1 + 1));
    }
    if pos.0 > 0 {
        neighbors[2] = Some((pos.0 - 1, pos.1));
    }
    if pos.0 < array_size[0] - 1 {
        neighbors[3] = Some((pos.0 + 1, pos.1));
    }
    return neighbors;
}

fn load_map() -> Array2<char> {
    let mut reader = BufReader::new(File::open(FILENAME).unwrap());
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    reader.rewind().expect("Failed to rewind reader");
    let array_width = line.len() - 1;
    let map = Array::from_iter(
        reader
            .bytes()
            .map(|b| b.unwrap() as char)
            .filter(|ch| *ch != '\n' && *ch != '\r' && *ch != '\0'),
    );
    let new_shape = (map.len() / array_width, array_width);
    map.into_shape(new_shape).unwrap()
}
