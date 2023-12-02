use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    let path = std::path::Path::new("./input/day1.txt");
    let calibration = sum_calibration_pt1(path);
    match calibration {
        Ok(calibration) => println!("Part 1 Calibration: {}", calibration),
        Err(e) => println!("Error: {}", e),
    }
    let calibration_pt2 = sum_calibration_pt2(path);
    match calibration_pt2 {
        Ok(calibration) => println!("Part 2 Calibration: {}", calibration),
        Err(e) => println!("Error: {}", e),
    }
}

fn sum_calibration_pt1(path: &Path) -> Result<u32, std::io::Error> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let lines = reader.lines();
    let mut sum = 0;
    for line in lines {
        sum += compute_line_pt1(&line?);
    }
    Ok(sum)
}

fn sum_calibration_pt2(path: &Path) -> Result<u32, std::io::Error> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let lines = reader.lines();
    let mut sum = 0;
    for line in lines {
        sum += compute_line_pt2(&line?);
    }
    Ok(sum)
}

fn compute_line_pt1(line: &str) -> u32 {
    let digits: Vec<u32> = line
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as u32)
        .collect();

    match digits.first() {
        Some(&first_digit) => {
            let last_digit = *digits.last().unwrap_or(&first_digit);
            first_digit * 10 + last_digit
        }
        None => 0,
    }
}

fn compute_line_pt2(line: &str) -> u32 {
    let pattern = r"zero|one|two|three|four|five|six|seven|eight|nine|\d";
    let re = Regex::new(pattern).unwrap();

    let num_words = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let num_map: HashMap<_, _> = num_words
        .iter()
        .enumerate()
        .map(|(i, &s)| (s, i as u32))
        .collect();

    let digits: Vec<u32> = re
        .find_iter(line)
        .filter_map(|m| {
            let m_str = m.as_str();
            m_str
                .parse::<u32>()
                .ok()
                .or_else(|| num_map.get(m_str).cloned())
        })
        .collect();

    match digits.len() {
        0 => 0,
        1 => digits[0] * 11,
        _ => digits[0] * 10 + digits.last().unwrap(),
    }
}
