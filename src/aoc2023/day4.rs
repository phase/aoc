use std::collections::{HashMap, VecDeque};

const INPUT1: &'static str = include_str!("./day4_input1.txt");
const INPUTU: &'static str = include_str!("./day4_input_unique.txt");

// 9pm start
// 9:10pm nums parsed into vecs per line
// 9:12pm first star!
// big twist
pub fn _part1() {
    let input = INPUTU;
    let result: u64 = input.lines().map(|l| {
        let mut value = 0;
        let mut inc = || {
            if value == 0 { value = 1 } else { value *= 2};
        };

        let start = l.find(": ").expect("needs prefix");
        let middle = l.find(" | ").expect("needs middle");

        let to_vec = |s: &str| {
            let vec: Vec<u64> = s.trim().split(" ").filter(|x|!x.is_empty()).map(|x|x.parse::<u64>().expect("not a num")).collect();
            vec
        };

        let winning_nums = to_vec(&l[start+2..middle]);
        let my_nums = to_vec(&l[middle+2..]);

        for m in my_nums.iter() {
            if winning_nums.contains(m) {
                inc();
            }
        }
        value
    }).sum();
    dbg!(result);
}

// 9:48pm second star!
pub fn run() {
    let input = INPUTU;
    let cards: HashMap<u64, u64> = input.lines().map(|l| {
        let mut value = 0;
        let mut inc = || {
            value += 1;
        };

        let start = l.find(": ").expect("needs prefix");
        let middle = l.find(" | ").expect("needs middle");

        let to_vec = |s: &str| {
            let vec: Vec<u64> = s.trim().split(" ").filter(|x|!x.is_empty()).map(|x|x.parse::<u64>().expect("not a num")).collect();
            vec
        };

        let card_id = (&l[5..start]).trim().parse::<u64>().expect(&format!("not a card id {}", &l[5..start]));
        let winning_nums = to_vec(&l[start+2..middle]);
        let my_nums = to_vec(&l[middle+2..]);
        for m in my_nums.iter() {
            if winning_nums.contains(m) {
                inc();
            }
        }
        dbg!(card_id, value);

        (card_id, value)
    }).collect();

    let mut scratchcards = 0u64;
    let last_card_id = cards.len();
    let mut q: VecDeque<u64> = VecDeque::new();

    // initial win check
    for id in 0..last_card_id+1 {
        q.push_front(id as u64);
    }


    while !q.is_empty() {
        //dbg!(&q);
        if let Some(card_id)  = q.pop_back() {
            if let Some(wins) = cards.get(&card_id) {
                if *wins > 0 {
                    for next in 1..(*wins+1) {
                        q.push_front(card_id + next);
                        //dbg!(card_id + next);
                    }
                }
                scratchcards += 1;
            }
        }

    }

    dbg!(scratchcards);
}
