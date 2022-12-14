use itertools::Itertools;
use ndarray::prelude::*;
use ndarray::Array2;
use std::fs::File;
use std::io::{BufRead, BufReader};

static FILENAME: &str = "data/day14";
static SAND_X: usize = 500;
static SAND_Y: usize = 0;

struct Map {
    data: Array2<bool>,
    x_offset: usize,
}

pub fn task1() {
    let mut map = create_map(false);
    let mut num_units = 0;
    'units: for i in 0.. {
        let mut sx = SAND_X - map.x_offset;
        let mut sy = SAND_Y;
        loop {
            if sy == map.data.shape()[0] - 1 {
                num_units = i;
                break 'units;
            } else if !map.data[(sy + 1, sx)] {
                sy += 1;
            } else if !map.data[(sy + 1, sx - 1)] {
                sx -= 1;
                sy += 1;
            } else if !map.data[(sy + 1, sx + 1)] {
                sx += 1;
                sy += 1;
            } else {
                map.data[(sy, sx)] = true;
                break;
            }
        }
    }
    println!("14.1: {}", num_units);
}

pub fn task2() {
    let mut map = create_map(true);
    let mut num_units = 0;
    for i in 0.. {
        let mut sx = SAND_X - map.x_offset;
        let mut sy = SAND_Y;
        if map.data[(sy, sx)] {
            num_units = i;
            break;
        }
        loop {
            if !map.data[(sy + 1, sx)] {
                sy += 1;
            } else if !map.data[(sy + 1, sx - 1)] {
                sx -= 1;
                sy += 1;
            } else if !map.data[(sy + 1, sx + 1)] {
                sx += 1;
                sy += 1;
            } else {
                map.data[(sy, sx)] = true;
                break;
            }
        }
    }
    println!("14.2: {}", num_units);
}

fn create_map(add_floor_and_pad_width: bool) -> Map {
    let structures = load_structures();
    let (min, max) = structures.iter().flatten().fold(
        ((usize::MAX, usize::MAX), (usize::MIN, usize::MIN)),
        |((min_x, min_y), (max_x, max_y)), (x, y)| {
            (
                (min_x.min(*x), min_y.min(*y)),
                (max_x.max(*x), max_y.max(*y)),
            )
        },
    );
    let height = if add_floor_and_pad_width {
        max.1 + 3
    } else {
        max.1 + 1
    };
    let width = if add_floor_and_pad_width {
        max.0 - min.0 + 3 + 2 * height
    } else {
        max.0 - min.0 + 3
    };
    let x_offset = if add_floor_and_pad_width {
        min.0 - 1 - height
    } else {
        min.0 - 1
    };
    let map_size = (height, width);
    let mut map = Map {
        data: Array2::<bool>::from_elem(map_size, false),
        x_offset,
    };
    if add_floor_and_pad_width {
        map.data
            .slice_mut(s![height - 1..height, 0..width])
            .fill(true);
    }
    for structure in structures {
        for window in structure.windows(2) {
            let (mut x1, mut y1) = window[0];
            let (mut x2, mut y2) = window[1];
            if x1 > x2 {
                (x1, x2) = (x2, x1);
            }
            if y1 > y2 {
                (y1, y2) = (y2, y1);
            }
            x1 = x1 - map.x_offset;
            x2 = x2 - map.x_offset;
            map.data.slice_mut(s![y1..y2 + 1, x1..x2 + 1]).fill(true);
        }
    }
    return map;
}

fn load_structures() -> Vec<Vec<(usize, usize)>> {
    let reader = BufReader::new(File::open(FILENAME).unwrap());
    reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            line.split(" -> ")
                .map(|raw_coord| {
                    raw_coord
                        .split(',')
                        .map(|val| val.parse::<usize>().unwrap())
                        .collect_tuple::<(usize, usize)>()
                        .unwrap()
                })
                .collect::<Vec<(usize, usize)>>()
        })
        .collect::<Vec<Vec<(usize, usize)>>>()
}
