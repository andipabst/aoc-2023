use std::{fs, io};
use std::cmp::max;
use std::fs::File;
use std::io::BufRead;
use std::ops::Range;
use std::path::Path;
use regex::{Match, Regex};

struct Number {
    row: usize,
    cols: Range<usize>,
    content: u32,
}

struct Symbol {
    row: usize,
    col: usize,
    content: char,
}

pub(crate) fn part_a() {
    let re_numbers = Regex::new(r"\d+").unwrap();
    let re_symbols = Regex::new(r"[^.\d]").unwrap();
    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

    if let Ok(lines) = read_lines("input/day3.txt") {
        let mut row: usize = 0;
        for _line in lines {
            if let Ok(line) = _line {
                //println!("{line}");

                for m in re_numbers.find_iter(&line) {
                    numbers.push(Number {
                        row: row,
                        cols: m.range(),
                        content: m.as_str().parse().unwrap(),
                    });
                }

                for m in re_symbols.find_iter(&line) {
                    symbols.push(Symbol {
                        row: row,
                        col: m.start(),
                        content: m.as_str().chars().nth(0).unwrap(),
                    });
                }
            }
            row += 1;
        }
    }

    let mut sum: u32 = 0;
    for number in &numbers {
        let number_cols = Range { start: number.cols.start.saturating_sub(1), end: number.cols.end.saturating_add(1) };
        for symbol in &symbols {
            let symbol_rows = Range { start: symbol.row.saturating_sub(1), end: symbol.row.saturating_add(2) };
            if symbol_rows.contains(&number.row) && number_cols.contains(&symbol.col) {
                sum += number.content;
                //println!("{} has contact with symbol {}", number.content, symbol.content);
                break;
            }
        }
    }
    println!("-----\n{sum}");

    let mut sum: u32 = 0;
    for symbol in symbols {
        let symbol_rows = Range { start: symbol.row.saturating_sub(1), end: symbol.row.saturating_add(2) };
        if symbol.content != '*' {
            continue
        }
        let mut parts: Vec<&Number> = Vec::new();
        for number in &numbers {
            let number_cols = Range { start: number.cols.start.saturating_sub(1), end: number.cols.end.saturating_add(1) };
            if symbol_rows.contains(&number.row) && number_cols.contains(&symbol.col) {
                parts.push(number);
            }
        }

        if parts.len() == 2 {
            sum += parts.get(0).unwrap().content * parts.get(1).unwrap().content
        }
    }
    println!("-----\n{sum}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}