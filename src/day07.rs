use std::fs;
use std::io::{BufRead, BufReader};

static FILENAME: &str = "data/day07";

const TOTAL_SIZE: usize = 70000000;
const REQUIRED_SIZE: usize = 30000000;

pub fn task1() {
    let total_size: usize = read_directories()
        .into_iter()
        .filter(|s| *s <= 100000)
        .sum();
    println!("07.1.: {}", total_size);
}

pub fn task2() {
    let mut directories = read_directories();
    directories.sort();
    let needed_size = REQUIRED_SIZE - (TOTAL_SIZE - directories.last().unwrap());
    let removed_size = directories
        .into_iter()
        .filter(|s| *s >= needed_size)
        .nth(0)
        .unwrap();
    println!("07.2.: {}", removed_size);
}

pub fn read_directories() -> Vec<usize> {
    let mut directory_sizes = Vec::<usize>::new();
    let mut iter = BufReader::new(fs::File::open(FILENAME).unwrap())
        .lines()
        .map(|line| line.unwrap());
    iter.next().unwrap();
    read_directory(&mut iter, &mut directory_sizes);
    return directory_sizes;
}

pub fn read_directory<I>(iter: &mut I, directory_sizes: &mut Vec<usize>) -> usize
where
    I: Iterator<Item = String>,
{
    let mut size = 0;
    loop {
        if let Some(line) = iter.next() {
            if line == "$ cd .." {
                directory_sizes.push(size);
                return size;
            } else if line.starts_with("$ cd") {
                size += read_directory(iter, directory_sizes);
            } else if line != "$ ls" && !line.starts_with("dir") {
                size += line.split(" ").nth(0).unwrap().parse::<usize>().unwrap();
            }
        } else {
            directory_sizes.push(size);
            return size;
        }
    }
}
