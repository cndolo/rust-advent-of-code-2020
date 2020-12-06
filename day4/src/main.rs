use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input_from_file(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).expect("file not found");
    let reader = BufReader::new(file);
    let mut passports: Vec<String> = Vec::new();
    let mut passport: String = String::new();
    let mut lines_read = 0;
    let lines: Vec<String> = reader
        .lines()
        .map(|l| l.expect("error reading line"))
        .collect();
    for i in 0..lines.len() {
        passport.push_str(&lines[i]);
        if lines[i].is_empty() || i == (lines.len() - 1) {
            passports.push(passport.clone());
            passport.clear();
            lines_read = 0;
        }
    }
    passports
}

fn count_valid_passports(list: &[String]) -> usize {
    let mut valid = 0;
    for line in list {
        if line_contains_required_fields(line) {
            valid += 1;
        }
    }
    valid
}

fn line_contains_required_fields(line: &str) -> bool {
    let mut valid = true;
    let required = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    for r in required {
        if !line.contains(r) {
            valid = false;
            break;
        }
    }
    valid
}

fn main() {
    let passports = read_input_from_file("input.txt");
    println!("{:?}", passports);
    println!("Read {} passports", passports.len());
    let valid_passports = count_valid_passports(&passports);
    println!("valid passports: {}", valid_passports);
}
