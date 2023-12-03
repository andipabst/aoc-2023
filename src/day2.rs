use std::{fs, io};
use std::cmp::max;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use regex::Regex;

pub(crate) fn part_a() {
    let re = Regex::new(r"Game (\d+): (.*)").unwrap();
    let re_split = Regex::new(r"[,;]+").unwrap();
    let mut sum_a: u32 = 0;
    let mut sum_b: i32 = 0;

    if let Ok(lines) = read_lines("input/day2.txt") {
        for _line in lines {
            if let Ok(line) = _line {
                println!("{line}");
                for (_, [game_id, draws]) in re.captures_iter(&line).map(|c| c.extract()) {
                    let mut red_max = 0;
                    let mut green_max = 0;
                    let mut blue_max = 0;
                    for colors in re_split.split(draws) {
                        if let [amount, color] = colors.trim().split(" ").collect::<Vec<&str>>()[..] {
                            print!("{}+{}", amount, color);
                            let amount_number = amount.parse::<i32>().unwrap();
                            if color == "blue" {
                                blue_max = max(amount_number, blue_max);
                            } else if color == "red" {
                                red_max = max(amount_number, red_max)
                            } else if color == "green" {
                                green_max = max(amount_number, green_max)
                            }
                        }
                    }

                    if red_max <= 12 && green_max <= 13 && blue_max <= 14 {
                        sum_a += game_id.parse::<u32>().unwrap()
                    }

                    sum_b += red_max * green_max * blue_max;

                    println!("- {}", game_id);
                }
            }
            println!("---");
        }
    }
    println!("{sum_a}");
    println!("{sum_b}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}