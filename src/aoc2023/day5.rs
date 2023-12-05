use std::collections::{HashMap, VecDeque};

const INPUT1: &'static str = include_str!("./day5_input1.txt");
const INPUTU: &'static str = include_str!("./day5_input_unique.txt");

#[derive(Default, Debug)]
struct Mapping {
    from: String,
    to: String,
    ranges: Vec<MappingRange>
}

#[derive(Debug)]
struct MappingRange {
    dest_start: u64,
    src_start: u64,
    range: u64
}

impl MappingRange {
    pub fn map(&self, value: u64) -> u64 {
        if value >= self.src_start && value <= self.src_start + self.range {
            return self.dest_start + (value - self.src_start);
        } else {
            return value;
        }
    }
}

// 9:12pm have a good idea
// 9:25pm done!
pub fn _part1() {
    let input = INPUTU;

    let mut mappings: Vec<Mapping> = Vec::new();
    let mut seeds: Vec<u64> = Vec::new();
    for line in input.lines() {
        if line.starts_with("seeds: ") {
            seeds = line.split("seeds: ")
                .collect::<Vec<_>>()[1]
                .split(" ")
                .map(|x|x.parse::<u64>().expect("not a num"))
                .collect();
        } else if line.ends_with(" map:") {
            let categories = line.split(" map:").collect::<Vec<_>>()[0]
                .split("-to-").collect::<Vec<_>>();
            let mapping = Mapping {
                from: categories[0].to_string(),
                to: categories[1].to_string(),
                ranges: Vec::new(),
            };

            mappings.push(mapping);
        } else if !line.is_empty() {
            let nums: Vec<u64> = line.split(" ").map(|x|x.parse::<u64>().expect("not a num"))
            .collect();
            let range = MappingRange {
                dest_start: nums[0],
                src_start: nums[1],
                range: nums[2],
            };
            mappings.last_mut().expect("no mapping?").ranges.push(range);
        }
    }

    dbg!(&mappings);
    let mut current_category = "seed".to_string();

    while current_category != "location" {
        for mapping in mappings.iter() {
            if mapping.from == current_category {
                current_category = mapping.to.clone();

                // translate nums
                for seed in seeds.iter_mut() {
                    for range in mapping.ranges.iter() {
                        let old_seed = *seed;
                        *seed = range.map(*seed);
                        if old_seed != *seed {
                            break;
                        }
                    }
                }

                break;
            }
        }
    }

    let min = seeds.iter().min();
    dbg!(min);
}

// 9:26 part 2
// 9:37 done! got 300th!!
pub fn run() {
    let input = INPUTU;

    let mut mappings: Vec<Mapping> = Vec::new();
    let mut seeds: Vec<u64> = Vec::new();
    for line in input.lines() {
        if line.starts_with("seeds: ") {
            let seed_parts: Vec<u64> = line.split("seeds: ")
                .collect::<Vec<_>>()[1]
                .split(" ")
                .map(|x|x.parse::<u64>().expect("not a num"))
                .collect();
            for i in (0u64..seed_parts.len() as u64).step_by(2) {
                let seed_start = seed_parts[i as usize];
                let seed_len = seed_parts[i as usize + 1];

                for j in 0..seed_len {
                    seeds.push(seed_start+j);
                }
            }
        } else if line.ends_with(" map:") {
            let categories = line.split(" map:").collect::<Vec<_>>()[0]
                .split("-to-").collect::<Vec<_>>();
            let mapping = Mapping {
                from: categories[0].to_string(),
                to: categories[1].to_string(),
                ranges: Vec::new(),
            };

            mappings.push(mapping);
        } else if !line.is_empty() {
            let nums: Vec<u64> = line.split(" ").map(|x|x.parse::<u64>().expect("not a num"))
            .collect();
            let range = MappingRange {
                dest_start: nums[0],
                src_start: nums[1],
                range: nums[2],
            };
            mappings.last_mut().expect("no mapping?").ranges.push(range);
        }
    }

    //dbg!(&seeds);
    let mut current_category = "seed".to_string();

    while current_category != "location" {
        for mapping in mappings.iter() {
            if mapping.from == current_category {
                current_category = mapping.to.clone();

                // translate nums
                for seed in seeds.iter_mut() {
                    for range in mapping.ranges.iter() {
                        let old_seed = *seed;
                        *seed = range.map(*seed);
                        if old_seed != *seed {
                            break;
                        }
                    }
                }

                break;
            }
        }
    }

    let min = seeds.iter().min();
    dbg!(min);
}
