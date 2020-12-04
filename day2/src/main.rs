use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn read_password_file(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).expect("file not found");
    let reader = BufReader::new(file);

    reader.lines().filter_map(io::Result::ok).collect()
}

fn check_passwords_are_valid(list: &[String]) -> usize {
    let mut valid_passwords: usize = 0;
    for i in list {
        let mut tmp_pw = i.replace("-", ",");
        tmp_pw = tmp_pw.replace(":", ",");
        let slices: Vec<&str> = tmp_pw.split(',').collect();
        let min = slices[0].parse::<usize>().unwrap();
        let max_and_letter: Vec<&str> = slices[1].split(' ').collect();
        let max = max_and_letter[0].parse::<usize>().unwrap();
        let letter = max_and_letter[1].parse::<char>().unwrap();
        let password: &str = slices[2];
        if is_valid_password(min, max, letter, password) {
            valid_passwords += 1;
        }
    }

    valid_passwords
}

fn is_valid_password(min: usize, max: usize, letter: char, password: &str) -> bool {
    let occurences = password.matches(letter).count();
    if occurences >= min && occurences <= max {
        return true;
    }
    false
}

fn main() {
    let list = read_password_file("input.txt");
    println!("Read {} passwords", list.len());
    let valid_passwords = check_passwords_are_valid(&list);
    println!("Found {} valid passwords.", valid_passwords);
}
