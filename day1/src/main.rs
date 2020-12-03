use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_list_of_values(file_path: &str) -> Vec<u64> {
    let file = File::open(file_path).expect("file not found");
    let reader = BufReader::new(file);

    let values: Vec<u64> = reader
        .lines()
        .map(|line| line.unwrap().parse::<u64>().unwrap())
        .collect();
    values
}

pub fn find_pairs_equal_to_2020(list: &[u64]) -> (u64, u64) {
    let mut pair = (0, 0);
    for i in list {
        let complement = 2020 - i;
        let index = list.iter().position(|x| *x == complement);
        if let Some(idx) = index {
            pair = (*i, list[idx]);
            break;
        }
    }
    pair
}

pub fn find_triples_equal_to_2020(list: &[u64]) -> (u64, u64, u64) {
    for i in 0..list.len() {
        for k in 0..list.len() {
            for j in 0..list.len() {
                if list[i] + list[k] + list[j] == 2020 {
                    return (list[i], list[k], list[j]);
                }
            }
        }
    }
    (0, 0, 0)
}

fn main() {
    let mut list = read_list_of_values("src/input.txt");
    list.sort_unstable();
    let entries = find_pairs_equal_to_2020(&list);
    println!("entries: {} {}", entries.0, entries.1);
    let product = entries.0 * entries.1;
    println!("product: {}", product);
    let entries = find_triples_equal_to_2020(&list);
    println!("entries: {} {} {}", entries.0, entries.1, entries.2);
    let product = entries.0 * entries.1 * entries.2;
    println!("product: {}", product);
}
