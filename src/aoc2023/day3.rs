use std::cell::Cell;
use regex::Regex;

const INPUT1: &'static str = include_str!("./day3_input1.txt");
const INPUTU: &'static str = include_str!("./day3_input_unique.txt");

#[derive(Debug, Clone)]
struct PartNumber {
    value: u64,
    line_num: u64,
    pos: (u64, u64),
    counted: Cell<bool>,
}

// woo day 3
// covid continues
// 10:17pm: it works on the example but not my unique input
// 10:24pm: I was missing '/'. one star down!
pub fn _part1() {
    let input = INPUTU;
    let line_len = input.find("\n").expect("empty?") as u64 + 1;

    let mut part_numbers = Vec::<PartNumber>::new();
    let num_regex = Regex::new(r"\d+").unwrap();
    for mat in num_regex.find_iter(input) {
        let line_num = mat.start() as u64 / line_len;
        let pos = (mat.start() as u64 % line_len, mat.end() as u64 % line_len - 1);
        let value = mat.as_str().parse::<u64>().expect("wut");
        let part_number = PartNumber {
            value,
            pos,
            line_num,
            counted: Cell::new(false),
        };
        println!("{:?}, line number {}", part_number, line_num);
        part_numbers.push(part_number);
    }

    let sym_regex = Regex::new(r"[\!\@\#\$\%\^\&\*\_\+\-\=\/]").unwrap();
    for mat in sym_regex.find_iter(input) {
        let sym_idx = mat.start() as u64 % line_len;
        let line_num = mat.start() as u64 / line_len;

        for part in part_numbers.iter() {
            if part.counted.clone().into_inner() { continue; }

            let in_bounds =
                line_num.abs_diff(part.line_num) <= 1
                    && sym_idx >= part.pos.0.saturating_sub(1)
                    && sym_idx <= part.pos.1.saturating_add(1)
            ;
            if in_bounds {
                part.counted.set(true);
            }
        }
    }

    for part_number in part_numbers.iter() {
        println!("{:?}", part_number);
    }

    let result: u64 = part_numbers.iter().filter(|x|x.counted.get()).map(|x|x.value).sum();
    dbg!(result);
}

// 10:32pm: just realized misread the instructions lol
// 10:35pm: done!
pub fn run() {
    let input = INPUTU;
    let line_len = input.find("\n").expect("empty?") as u64 + 1;

    let mut total = 0u64;

    let mut part_numbers = Vec::<PartNumber>::new();
    let num_regex = Regex::new(r"\d+").unwrap();
    for mat in num_regex.find_iter(input) {
        let line_num = mat.start() as u64 / line_len;
        let pos = (mat.start() as u64 % line_len, mat.end() as u64 % line_len - 1);
        let value = mat.as_str().parse::<u64>().expect("wut");
        let part_number = PartNumber {
            value,
            pos,
            line_num,
            counted: Cell::new(false),
        };
        part_numbers.push(part_number);
    }

    let sym_regex = Regex::new(r"\*").unwrap();
    for mat in sym_regex.find_iter(input) {
        let sym_idx = mat.start() as u64 % line_len;
        let line_num = mat.start() as u64 / line_len;

        let mut adj_parts = Vec::<&PartNumber>::new();
        for part in part_numbers.iter() {
            if part.counted.clone().into_inner() { continue; }

            let in_bounds =
                line_num.abs_diff(part.line_num) <= 1
                    && sym_idx >= part.pos.0.saturating_sub(1)
                    && sym_idx <= part.pos.1.saturating_add(1)
            ;
            if in_bounds {
                adj_parts.push(part);
            }
        }
        if adj_parts.len() == 2 {
            total += adj_parts[0].value * adj_parts[1].value;
        }
    }

    dbg!(total);
}