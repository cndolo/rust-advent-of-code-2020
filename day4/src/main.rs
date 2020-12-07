use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input_from_file(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).expect("file not found");
    let reader = BufReader::new(file);
    let mut passports: Vec<String> = Vec::new();
    let mut passport: String = String::new();
    let lines: Vec<String> = reader
        .lines()
        .map(|l| l.expect("error reading line"))
        .collect();
    for i in 0..lines.len() {
        passport.push_str(&lines[i]);
        passport.push_str(" ");
        if lines[i].is_empty() || i == (lines.len() - 1) {
            passports.push(passport.clone());
            passport.clear();
        }
    }
    passports
}

fn count_valid_passports(list: &[String]) -> usize {
    let mut valid = 0;
    for line in list {
        if line_contains_required_fields(line) {
            let split_at_space = line.split(' ');
            let mut map: HashMap<&str, &str> = HashMap::new();
            for s in split_at_space {
                let key_values: Vec<&str> = s.split(':').collect();
                for i in 0..key_values.len() {
                    if i % 2 != 0 {
                        continue;
                    }
                    if !key_values[0].is_empty() {
                        map.insert(key_values[0], key_values[1]);
                    }
                }
            }
            if is_valid_passport(&map) {
                valid += 1
            }
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

fn is_valid_passport(passport: &HashMap<&str, &str>) -> bool {
    passport.iter().all(|(&k, v)| match k {
        "byr" => (1920..=2002).contains(&v.parse().unwrap_or(0)),
        "iyr" => (2010..=2020).contains(&v.parse().unwrap_or(0)),
        "eyr" => (2020..=2030).contains(&v.parse().unwrap_or(0)),
        "hcl" => {
            v.starts_with('#') && v.len() == 7 && v.chars().skip(1).all(|c| c.is_ascii_hexdigit())
        }
        "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(v),
        "pid" => v.len() == 9 && v.chars().all(|c| c.is_ascii_digit()),
        "cid" => true,
        "hgt" => {
            let height = v[0..(v.len() - 2)].parse().unwrap_or(0);
            match &v[(v.len() - 2)..] {
                "cm" => (150..=193).contains(&height),
                "in" => (59..=76).contains(&height),
                _ => false,
            }
        }
        _ => unreachable!(),
    })
}

fn main() {
    let passports = read_input_from_file("input.txt");
    println!("Read {} passports", passports.len());
    let valid_passports = count_valid_passports(&passports);
    println!("valid passports: {}", valid_passports);
}
