use std::{fs, io};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;

#[derive(Debug)]
struct Mapping {
    from: String,
    to: String,
    from_start: u64,
    to_start: u64,
    length: u64,
}

impl Mapping {
    pub fn maps_to(&self, from: String, id: u64) -> Option<u64> {
        if from == self.from && self.from_start <= id && id < self.from_start + self.length {
            let new_id = id.saturating_add_signed(self.to_start as i64 - self.from_start as i64);
            println!("{} to {} in {:?}", id, new_id, self);
            Some(new_id)
        } else {
            None
        }
    }
}

pub(crate) fn part_a() {
    if let Ok(lines) = read_lines("input/day5.txt") {
        let mut seeds = Vec::new();
        let mut mappings: HashMap<String, Vec<Mapping>> = HashMap::new();

        let mut current_mapping = String::new();
        for _line in lines {
            if let Ok(line) = _line {
                if line.starts_with("seeds:") {
                    seeds = Vec::from_iter(line.replace("seeds: ", "").split_whitespace().map(|x| x.parse::<u64>().unwrap()));
                } else if line.ends_with("map:") {
                    current_mapping = line.replace(" map:", "");
                } else if !line.trim().is_empty() {
                    let ranges = Vec::from_iter(line.split_whitespace().map(|x| x.parse::<u64>().unwrap()));
                    let from = current_mapping.split("-to-").nth(0).unwrap().to_string();
                    let mapping = Mapping {
                        from: current_mapping.split("-to-").nth(0).unwrap().to_string(),
                        to: current_mapping.split("-to-").nth(1).unwrap().to_string(),
                        from_start: ranges[1],
                        to_start: ranges[0],
                        length: ranges[2],
                    };
                    if let Some(m) = mappings.get_mut(&from) {
                        m.push(mapping);
                    } else {
                        mappings.insert(from, vec![mapping]);
                    }
                }
            }
        }

        let mut smallest = u64::MAX;
        for seed in &seeds {
            let mut result = ("seed".to_string(), seed.clone());
            while result.0 != "location" {
                let at_start = result.0.clone();
                for mapping in mappings.get(&result.0).unwrap() {
                    if let Some(to) = mapping.maps_to(result.0.clone(), result.1) {
                        result = (mapping.to.clone(), to);
                        println!("{:?}", result);
                    }
                }
                // no mapping for this in list
                if result.0 == at_start {
                    for mapping in mappings.get(&result.0).unwrap() {
                        if mapping.from == at_start {
                            result = (mapping.to.clone(), result.1);
                            println!("{:?}", result);
                            break;
                        }
                    }
                }
            }

            if result.1 < smallest {
                smallest = result.1
            }

            println!("seed {} -> {:?}", seed, result);
            println!("----------");
        }

        let mut smallest_b = u64::MAX;
        for chunk in seeds.chunks_exact(2) {
            for seed in chunk[0]..(chunk[0] + chunk[1]) {
                let mut result = ("seed".to_string(), seed.clone());
                while result.0 != "location" {
                    let at_start = result.0.clone();
                    for mapping in mappings.get(&result.0).unwrap() {
                        if let Some(to) = mapping.maps_to(result.0.clone(), result.1) {
                            result = (mapping.to.clone(), to);
                            //println!("{:?}", result);
                        }
                    }
                    // no mapping for this in list
                    if result.0 == at_start {
                        for mapping in mappings.get(&result.0).unwrap() {
                            if mapping.from == at_start {
                                result = (mapping.to.clone(), result.1);
                                //println!("{:?}", result);
                                break;
                            }
                        }
                    }
                }

                if result.1 < smallest_b {
                    smallest_b = result.1
                }

                //println!("seed {} -> {:?}", seed, result);
                //println!("----------");
            }
        }

        println!("---\n{smallest}");
        println!("---\n{smallest_b}");
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}