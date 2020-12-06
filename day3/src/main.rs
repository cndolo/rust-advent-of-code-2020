use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input_from_file(file_path: &str) -> Vec<Vec<String>> {
    let file = File::open(file_path).expect("file not found");
    let reader = BufReader::new(file);
    let mut map: Vec<Vec<String>> = Vec::new();
    for line in reader.lines() {
        map.push(vec![line.unwrap()])
    }
    map
}

fn traverse_map_in_different_orders(list: &[Vec<String>], right: usize, down: usize) -> usize {
    let mut trees_hit = 0;
    let mut column = 0;
    let mut pos = 1;
    while column < list.len() - 1 {
        pos += right;
        column += down;
        let line = &list[column][0];
        let potential_tree = line.chars().nth((pos - 1) % line.len()).unwrap();
        if potential_tree == '#' {
            trees_hit += 1;
        }
    }
    trees_hit
}

fn main() {
    let map = read_input_from_file("input.txt");
    let down = vec![1, 1, 1, 1, 2];
    let right = vec![1, 3, 5, 7, 1];
    let mut hits = vec![];
    for i in 0..down.len() {
        hits.push(traverse_map_in_different_orders(&map, right[i], down[i]));
    }
    println!("Slopes per combination: {:?}", hits);
    let total_crashes: usize = hits.iter().product();
    println!("Hit {} trees", total_crashes);
}
