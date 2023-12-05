use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    let path = std::path::Path::new("./input/day4.txt");
    let sum = run_pt_1(path);
    match sum {
        Ok(sum) => println!("Part 1: {}", sum),
        Err(e) => println!("Error: {}", e),
    }
    run_pt_2(path);
}

fn run_pt_1(path: &Path) -> Result<u32, std::io::Error> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let lines = reader.lines();
    let mut sum = 0;
    for line in lines {
        sum += calc_winning_points(&line?);
    }
    Ok(sum)
}

fn run_pt_2(path: &Path) {
    let file = File::open(path).unwrap();
    let reader = io::BufReader::new(file);
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    let card_to_winnings: HashMap<usize, usize> = (0..lines.len())
        .map(|i| (i, calc_scorecards_won(lines[i].as_str())))
        .collect();
    let mut scores = vec![1u32; card_to_winnings.len()];
    (0..scores.len()).for_each(|i| {
        (i + 1..=i + card_to_winnings.get(&i).unwrap()).for_each(|j| scores[j] += scores[i]);
    });
    let score = scores.iter().sum::<u32>();
    println!("Part 2: {}", score);
}

fn calc_winning_points(line: &str) -> u32 {
    let parts = line.split(": ").collect::<Vec<&str>>();
    let num_parts = parts[1].split("|").collect::<Vec<&str>>();
    let winning_nums: Vec<&str> = num_parts[0].split_whitespace().collect();
    let my_nums: Vec<&str> = num_parts[1].split_whitespace().collect();
    let mut winning_points = 0;
    for num in my_nums {
        if winning_nums.contains(&num) {
            if winning_points == 0 {
                winning_points += 1;
            } else {
                winning_points *= 2;
            }
        }
    }
    winning_points
}

fn calc_scorecards_won(line: &str) -> usize {
    let parts = line.split(": ").collect::<Vec<&str>>();
    let num_parts = parts[1].split("|").collect::<Vec<&str>>();
    let winning_nums: Vec<&str> = num_parts[0].split_whitespace().collect();
    let my_nums: Vec<&str> = num_parts[1].split_whitespace().collect();
    let mut cards_won = 0;
    for num in my_nums {
        if winning_nums.contains(&num) {
            cards_won += 1;
        }
    }
    cards_won
}
