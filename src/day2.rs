use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    let path = std::path::Path::new("./input/day2.txt");
    let sum = run_pt_1(path);
    match sum {
        Ok(sum) => println!("Part 1: {}", sum),
        Err(e) => println!("Error: {}", e),
    }
    let sum = run_pt_2(path);
    match sum {
        Ok(sum) => println!("Part 2: {}", sum),
        Err(e) => println!("Error: {}", e),
    }
}

fn run_pt_1(path: &Path) -> Result<u32, std::io::Error> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let lines = reader.lines();
    let mut sum = 0;
    for line in lines {
        sum += check_line(&line?);
    }
    Ok(sum)
}

fn run_pt_2(path: &Path) -> Result<u32, std::io::Error> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let lines = reader.lines();
    let mut sum = 0;
    for line in lines {
        sum += line_power(&line?);
    }
    Ok(sum)
}

fn check_line(line: &str) -> u32 {
    let parts: Vec<&str> = line.split(": ").collect();
    let game_id: u32 = parts[0]
        .trim()
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();

    let sets = parts[1].split(";");

    for set in sets {
        let (mut red, mut green, mut blue) = (0, 0, 0);
        set.trim().split(", ").for_each(|color| {
            let parts: Vec<&str> = color.split_whitespace().collect();
            let val: u32 = parts[0].parse().unwrap();
            match parts[1] {
                "red" => red = val,
                "green" => green = val,
                "blue" => blue = val,
                _ => (),
            }
        });

        if red > 12 || green > 13 || blue > 14 {
            return 0;
        }
    }

    game_id
}

fn line_power(line: &str) -> u32 {
    let parts: Vec<&str> = line.split(": ").collect();
    let sets = parts[1].split(";");
    let (mut red, mut green, mut blue) = (0, 0, 0);
    for set in sets {
        set.trim().split(", ").for_each(|color| {
            let parts: Vec<&str> = color.split_whitespace().collect();
            let val: u32 = parts[0].parse().unwrap();
            match parts[1] {
                "red" => {
                    if val > red {
                        red = val
                    }
                }
                "green" => {
                    if val > green {
                        green = val
                    }
                }
                "blue" => {
                    if val > blue {
                        blue = val
                    }
                }
                _ => (),
            }
        });
    }
    red * green * blue
}
