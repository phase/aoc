use std::cmp::Ordering;

const INPUT1: &'static str = include_str!("./day7_input1.txt");
const INPUTU: &'static str = include_str!("./day7_input_unique.txt");

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfACard,
}
use std::fmt;

impl fmt::Display for HandType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn j_count(hand: &[(char, usize)]) -> usize {
    let mut total = 0usize;
    for (c, i) in hand.iter() {
        if *c == 'J' {
            total += i;
            break;
        }
    }
    total
}

fn hand_type(hand: &[(char, usize)]) -> HandType {
    use HandType::*;
    let j = j_count(hand);

    match hand {
        [(_, 5)] => FiveOfACard,
        [(_, 4), ..] if j == 1 => FiveOfACard,
        [(_, 3), ..] if j == 2 => FiveOfACard,
        [('J', 3), (_, 2)] if j == 3 => FiveOfACard,
        _ if j == 4 => FiveOfACard,
        [(_, 4), ..] if j == 0 => FourOfAKind,
        [(_, 3), ..] if j == 1 || j == 2 => FourOfAKind,
        [(a, 2), ..] if *a != 'J' && j == 2 => FourOfAKind,
        _ if j == 3 => FourOfAKind,
        [(_, 3), (_, 2)] if j == 0 => FullHouse,
        [(_, 2), (_, 2)] if j == 1 => FullHouse,
        [(_, 3), ..] if j == 0 => ThreeOfAKind,
        [(_, 2), ..] if j == 1 => ThreeOfAKind,
        _ if j == 2 => ThreeOfAKind,
        [(_, 2), (_, 2), ..] => TwoPairs,
        [(_, 2), ..] if j == 1 => TwoPairs,
        [(_, 2), ..] if j == 0 => OnePair,
        _ if j == 1 => OnePair,
        _ => HighCard,
    }
}

const _ORDER_part1: &'static str = "AKQJT98765432";
const ORDER: &'static str = "AKQT98765432J";

fn high_card(a: char, b: char) -> Ordering {
    let a_i = ORDER.find(a).expect(&format!("cant find a: {}", a));
    let b_i = ORDER.find(b).expect(&format!("cant find b: {}", a));
    if a_i == b_i {
        Ordering::Equal
    } else if a_i < b_i {
        Ordering::Greater
    } else if a_i > b_i {
        Ordering::Less
    } else {
        panic!("wut");
    }
}

fn high_card_str(a: &str, b: &str) -> Ordering {
    // todo dont clone??? I cant index into chars()??
    let a = a.chars().collect::<Vec<_>>();
    let b = b.chars().collect::<Vec<_>>();

    for i in 0..a.len().min(b.len()) {
        let cmp = high_card(a[i], b[i]);
        if cmp != Ordering::Equal {
            return cmp;
        }
    }
    Ordering::Equal
}

// ignores card order
fn _high_card_of_hands(a: &[(char, usize)], b: &[(char, usize)]) -> Ordering {
    for i in 0..a.len().min(b.len()) {
        let cmp = high_card(a[i].0, b[i].0);
        if cmp != Ordering::Equal {
            return cmp;
        }
    }
    Ordering::Equal
}

fn parse(hand: &str) -> Vec<(char, usize)> {
    let mut store: Vec<(char, usize)> = Vec::with_capacity(hand.len());
    'outer: for ch in hand.chars() {
        for other in store.iter_mut() {
            if other.0 == ch {
                // we saw this char already, increment the value
                other.1 = other.1 + 1;
                continue 'outer;
            }
        }
        store.push((ch, 1));
    }
    store.sort_by(|a, b| {
        if a.1 > b.1 {
            Ordering::Less
        } else if b.1 > a.1 {
            Ordering::Greater
        } else {
            high_card(a.0, b.0)
        }
    });
    store
}

// 9:00pm start!
// 9:15pm type fun done
// 9:30 hand parsing & typing done!
// 10:05 got the right answer for the test data but not the unique data
// 10:23 not 255052048...
// 10:24 not 255060568...
// 10:45 foud the stupid typo in my hand_type... should have used an enum...
// 10:46 first star!
// 11:10 not 253619564...
// 11:18pm not 253781036...
// 11:27pm not 253930126... im tired...
// 11:37pm.... NOT 254398869....
// 11:43pm give up!
pub fn run() {
    let input = INPUTU;

    let mut input = input.lines().map(|line| {
        let parts = line.split(" ").collect::<Vec<_>>();
        let hand = parse(parts[0]);
        let bid = parts[1].trim().parse::<usize>().expect("not a bid");
        let hand_type = hand_type(&hand);
        (line, hand, hand_type, bid)
    }).collect::<Vec<_>>();

    //dbg!("before sort", &input);

    input.sort_by(|a, b| {
        let cmp = a.2.cmp(&b.2);
        if cmp == Ordering::Equal {
            high_card_str(a.0, b.0)
            // NOT THIS _high_card_of_hands(a.1.as_slice(), b.1.as_slice())
        } else {
            cmp
        }
    });

    //dbg!("after sort", &input);
    for (_line, _hand, _hand_type, bid) in input.iter() {
        //println!("{} type={}", _line, _hand_type);
    }

    let total_winnings = input.iter().enumerate().fold(0, |acc, (index, (_line, _hand, _hand_type, bid))| {
        //dbg!(&(index + 1, bid));
        let winnings = (index + 1) * *bid;
        println!("{} type={} winnings={}*{}={} total={}", _line, _hand_type, index+1, *bid, winnings, acc + winnings);
        acc + winnings
    });

    dbg!(&total_winnings);
}
