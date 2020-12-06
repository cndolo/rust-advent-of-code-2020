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

fn traverse_map(list: &[Vec<String>]) -> usize {
    let mut trees_hit = 0;
    let mut column = 0;
    let mut pos = 1;
    while column < list.len() - 1 {
        pos += 3;
        column += 1;
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
    let crashes = traverse_map(&map);
    println!("Hit {} trees", crashes);
}
