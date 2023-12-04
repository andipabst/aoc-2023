use std::{fs, io};
use std::collections::hash_map::RandomState;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufRead;
use std::path::Path;

use regex::Regex;

pub(crate) fn part_a() {
    let re_space = Regex::new(r"\s+").unwrap();

    let mut sum = 0;
    let mut card_counts = HashMap::from([(1, 1)]);
    if let Ok(lines) = read_lines("input/day4.txt") {
        for _line in lines {
            if let Ok(line) = _line {
                let (card, values) = line.split_once(":").unwrap();
                let card_id = card.replace("Card", "").trim().parse::<i32>().unwrap();
                let (winning_numbers, numbers_i_have) = values.split_once("|").unwrap();
                let winning: HashSet<&str, RandomState> = HashSet::from_iter(re_space.split(winning_numbers.trim()));
                let having: HashSet<&str, RandomState> = HashSet::from_iter(re_space.split(numbers_i_have.trim()));

                let numbers: Vec<&&str> = winning.intersection(&having).collect();
                println!("{} has matching numbers {:?}", card, numbers);
                sum += match numbers.len() {
                    0 => { 0 }
                    1 => { 1 }
                    x => { 2_usize.pow((x as u32) - 1) }
                };
                let current_card_count = if let Some(c) = card_counts.get(&card_id) {
                    *c
                } else {
                    card_counts.insert(card_id, 1);
                    1
                };
                for i in (card_id + 1)..(card_id + 1 + (numbers.len() as i32)) {
                    if let Some(c) = card_counts.get_mut(&i) {
                        *c = *c + current_card_count;
                    } else {
                        card_counts.insert(i, current_card_count + 1);
                    }
                }
                println!("{:?}", card_counts);
            }
        }
    }
    println!("---\n{}", sum);
    println!("---\n{}", card_counts.values().sum::<i32>());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}