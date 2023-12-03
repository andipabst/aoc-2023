use std::{fs, io};
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use regex::Regex;

pub(crate) fn part_a() {
    let re = Regex::new(r"[a-z]").unwrap();
    let mut sum: u32 = 0;

    if let Ok(lines) = read_lines("input/day1.txt") {
        for _line in lines {
            if let Ok(line) = _line {
                let digits = re.replace_all(&line, "");
                let mut d = digits.chars();
                sum += d.clone().nth(0).unwrap().to_digit(10).unwrap() * 10 + d.clone().nth_back(0).unwrap().to_digit(10).unwrap();
            }
        }
    }
    println!("{sum}")
}

pub(crate) fn part_b() {
    let re = Regex::new(r"[a-z]").unwrap();
    let mut sum: u32 = 0;

    if let Ok(lines) = read_lines("input/day1.txt") {
        for _line in lines {
            if let Ok(mut line) = _line {
                line = line.replace("twone", "21");
                line = line.replace("oneight", "18");
                line = line.replace("one", "1");
                line = line.replace("eightwo", "82");
                line = line.replace("two", "2");
                line = line.replace("eighthree", "83");
                line = line.replace("three", "3");
                line = line.replace("four", "4");
                line = line.replace("five", "5");
                line = line.replace("six", "6");
                line = line.replace("seven", "7");
                line = line.replace("nineight", "98");
                line = line.replace("eight", "8");
                line = line.replace("nine", "9");
                let digits = re.replace_all(&line, "");
                let mut d = digits.chars();
                sum += d.clone().nth(0).unwrap().to_digit(10).unwrap() * 10 + d.clone().nth_back(0).unwrap().to_digit(10).unwrap();
            }
        }
    }
    println!("{sum}")
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}