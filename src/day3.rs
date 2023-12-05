use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    let path = std::path::Path::new("./input/day3.txt");
    let pt_1 = part_1(path);
    match pt_1 {
        Ok(pt_1) => println!("Part 1: {}", pt_1),
        Err(e) => println!("Error: {}", e),
    }
    let pt_2 = part_2(path);
    match pt_2 {
        Ok(pt_2) => println!("Part 2: {}", pt_2),
        Err(e) => println!("Error: {}", e),
    }
}

fn part_1(path: &Path) -> Result<u32, std::io::Error> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let lines: Vec<Vec<char>> = reader
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();
    let nums = nums_next_to_symbols(&lines);
    Ok(sum_num_strings(nums))
}

fn part_2(path: &Path) -> Result<u32, std::io::Error> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let lines: Vec<Vec<char>> = reader
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();
    Ok(sum_gear_ratios(&lines))
}

fn sum_num_strings(nums: Vec<String>) -> u32 {
    let mut sum = 0;
    for num in nums {
        sum += num.parse::<u32>().unwrap();
    }
    sum
}

fn is_symbol(c: char) -> bool {
    !c.is_digit(10) && c != '.'
}

fn is_gear_symbol(c: char) -> bool {
    c == '*'
}

fn nums_next_to_symbols(lines: &[Vec<char>]) -> Vec<String> {
    let mut nums = Vec::new();
    //(line #, char #)
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1), //up left, up, up right
        (0, -1),
        (0, 1), //left, right
        (1, -1),
        (1, 0),
        (1, 1),
    ]; //down left, down, down right

    for (i, line) in lines.iter().enumerate() {
        let mut j = 0;
        while j < line.len() {
            let mut num = String::new();
            let mut next_to_sym = false;

            if line[j].is_digit(10) {
                let mut k = j;
                while k < line.len() && line[k].is_digit(10) {
                    num.push(line[k]);
                    k += 1;
                }

                for idx in j..k {
                    for &(di, dj) in &directions {
                        if is_valid_char(
                            i as i32,
                            idx as i32,
                            di,
                            dj,
                            lines.len() as i32,
                            line.len() as i32,
                        ) {
                            let adjacent_char =
                                lines[(i as i32 + di) as usize][(idx as i32 + dj) as usize];
                            if is_symbol(adjacent_char) {
                                next_to_sym = true;
                                break;
                            }
                        }
                    }
                    if next_to_sym {
                        break;
                    }
                }

                if next_to_sym {
                    nums.push(num);
                }
                j = k;
            } else {
                j += 1;
            }
        }
    }

    nums
}

fn is_valid_char(i: i32, j: i32, di: i32, dj: i32, num_lines: i32, line_len: i32) -> bool {
    i + di >= 0 && i + di < num_lines && j + dj >= 0 && j + dj < line_len
}

fn sum_gear_ratios(lines: &[Vec<char>]) -> u32 {
    let mut gear_ratio = 0;

    //(line #, char #)
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1), //up left, up, up right
        (0, -1),
        (0, 1), //left, right
        (1, -1),
        (1, 0),
        (1, 1),
    ]; //down left, down, down right

    for (i, line) in lines.iter().enumerate() {
        let mut j = 0;
        while j < line.len() {
            if is_gear_symbol(line[j]) {
                let mut checked_coords: HashMap<(i32, i32), bool> = HashMap::new();
                let mut nums: Vec<String> = Vec::new();

                for &(di, dj) in &directions {
                    if is_valid_char(
                        i as i32,
                        j as i32,
                        di,
                        dj,
                        lines.len() as i32,
                        line.len() as i32,
                    ) {
                        let adjacent_char =
                            lines[(i as i32 + di) as usize][(j as i32 + dj) as usize];

                        if adjacent_char.is_digit(10)
                            && !is_checked((i as i32 + di, j as i32 + dj), &checked_coords)
                        {
                            let mut num = String::new();
                            let i_num = i as i32 + di;
                            let mut j_num = j as i32 + dj;

                            //go left until not a digit to find the start
                            while j_num >= 1 {
                                if lines[i_num as usize][(j_num - 1) as usize].is_digit(10) {
                                    j_num -= 1;
                                } else {
                                    break;
                                }
                            }

                            //then iterate back right to find the end and add the coords as checked
                            while lines[i_num as usize][j_num as usize].is_digit(10) {
                                num.push(lines[i_num as usize][j_num as usize]);
                                checked_coords.insert((i_num, j_num), true);
                                if j_num + 1 < line.len() as i32 {
                                    j_num += 1;
                                } else {
                                    break;
                                }
                            }

                            if num.len() > 0 {
                                nums.push(num)
                            };
                        }
                    }
                }
                if nums.len() == 2 {
                    gear_ratio += nums[0].parse::<u32>().unwrap() * nums[1].parse::<u32>().unwrap();
                }
            }
            j += 1;
        }
    }
    gear_ratio
}

fn is_checked(coord: (i32, i32), checked_coords: &HashMap<(i32, i32), bool>) -> bool {
    *checked_coords.get(&coord).unwrap_or(&false)
}
