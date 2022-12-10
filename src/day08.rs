use ndarray::prelude::*;
use ndarray::{Array, Array2};
use std::cmp::max;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader, Seek};

static FILENAME: &str = "data/day08";

pub fn task1() {
    let map = load_map();
    let mut visibility_map = Array2::<bool>::from_elem(map.raw_dim(), false);
    for row in 0..map.shape()[0] {
        update_visibility(visibility_map.row_mut(row), map.row(row));
    }
    for column in 1..map.shape()[1] {
        update_visibility(visibility_map.column_mut(column), map.column(column));
    }
    visibility_map.slice_mut(s![0, ..]).fill(true);
    visibility_map
        .slice_mut(s![visibility_map.shape()[0] - 1, ..])
        .fill(true);
    visibility_map.slice_mut(s![.., 0]).fill(true);
    visibility_map
        .slice_mut(s![.., visibility_map.shape()[1] - 1])
        .fill(true);
    let num_visible = visibility_map.iter().copied().filter(|x| *x).count();
    println!("08.1: {}", num_visible);
}

pub fn task2() {
    let map = load_map();
    let mut scores = Array2::<u32>::ones(map.raw_dim());
    for row in 0..scores.shape()[0] {
        update_score(scores.row_mut(row), map.row(row));
    }
    for column in 1..scores.shape()[1] {
        update_score(scores.column_mut(column), map.column(column));
    }
    let max_score = scores.iter().fold(0, |tot, x| max(tot, *x));
    println!("08.2: {}", max_score);
}

fn update_visibility(mut visibility_map: ArrayViewMut1<bool>, map: ArrayView1<u32>) {
    update_line_visibility(visibility_map.iter_mut().zip(map.iter()));
    update_line_visibility(visibility_map.iter_mut().zip(map.iter()).rev());
}

fn update_line_visibility<'a, I>(iter: I)
where
    I: Iterator<Item = (&'a mut bool, &'a u32)>,
{
    let mut max_height: u32 = 0;
    for (visible, map_value) in iter {
        *visible |= *map_value > max_height;
        max_height = max(max_height, *map_value);
    }
}

fn update_score(mut scores: ArrayViewMut1<u32>, map: ArrayView1<u32>) {
    update_line_score(scores.iter_mut().zip(map.iter()));
    update_line_score(scores.iter_mut().zip(map.iter()).rev());
}

fn update_line_score<'a, I>(iter: I)
where
    I: Iterator<Item = (&'a mut u32, &'a u32)>,
{
    let mut distance_to_size = [0; 10];
    for (score, map_value) in iter {
        *score *= distance_to_size[*map_value as usize];
        for i in 0..*map_value as usize + 1 {
            distance_to_size[i] = 1;
        }
        for i in *map_value as usize + 1..distance_to_size.len() {
            distance_to_size[i] += 1;
        }
    }
}

fn load_map() -> Array2<u32> {
    let mut reader = BufReader::new(File::open(FILENAME).unwrap());
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    reader.rewind().expect("Failed to rewind reader");
    let array_size = line.len() - 1;
    Array::from_iter(
        reader
            .bytes()
            .map(|b| b.unwrap() as char)
            .filter(|ch| *ch != '\n' && *ch != '\r' && *ch != '\0')
            .map(|ch| ch.to_digit(10).unwrap()),
    )
    .into_shape((array_size, array_size))
    .unwrap()
}
