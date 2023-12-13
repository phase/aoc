use std::{cell::Cell, collections::{HashMap}, thread, sync::{Arc, Mutex}};

const INPUT1: &'static str = include_str!("./day8_input1.txt");
const INPUTU: &'static str = include_str!("./day8_input_unique.txt");

struct Node {
    value: String,
    left: Cell<Option<Box<Node>>>,
    right: Cell<Option<Box<Node>>>,
}

// 9:12pm getting started! I'm a few days behind!
// 9:42pm first star done! two hash maps was a good idea
pub fn _part1() {
    let input = INPUTU;
    let lines = input.lines().collect::<Vec<_>>();

    let mut left_map: HashMap<&str, &str> = HashMap::default();
    let mut right_map: HashMap<&str, &str> = HashMap::default();

    // RLLRLLLRLR
    // reverse() so we can pop off the first letter
    let mut turn_line = lines.get(0).expect("no turn line??").to_string().chars().rev().collect::<String>();
    let turn_backup = turn_line.clone();

    for i in 2..lines.len() {
        let line = lines.get(i).expect("aaa where is the line");
        let parts = line.split(" = ").collect::<Vec<_>>();

        let (value, leaves) = match parts.as_slice() {
            [value, leaves] => (value, leaves),
            _ => panic!("Invalid parts"),
        };

        let leaves_parts = leaves.split(", ").collect::<Vec<_>>();
        let (left, right) = match leaves_parts.as_slice() {
            // trim off ()
            [left, right] => (&left[1..], &right[..right.len() - 1]),
            _ => panic!("invalid parts"),
        };

        left_map.insert(value, left);
        right_map.insert(value, right);
    }

    let mut current = "AAA";
    let mut steps = 0usize;

    while current != "ZZZ" {
        if let Some(turn) = turn_line.pop() {
            let map = if turn == 'L' { &mut left_map } else { &mut right_map };
            let next = map.get(current).expect(&format!("couldn't find {}", current));
            println!("Turning {} to {}", turn, next);
            current = next;
            steps += 1;
        } else {
            turn_line = turn_backup.clone();
        }
    }
    dbg!(steps);
}


pub fn count_steps(
    start: &str,
    turn_line: String,
    left_map: &HashMap<&str, &str>,
    right_map: &HashMap<&str, &str>,
    shared_positions: Arc<Mutex<Vec<&str>>>,
    slot: usize,
) -> usize {
    let mut turn_line = turn_line;
    let turn_backup = turn_line.clone();

    let mut current = start;
    let mut steps = 0usize;

    while !current.ends_with("Z") {
        if let Some(turn) = turn_line.pop() {
            let map = if turn == 'L' { left_map } else { right_map };
            let next = map.get(current).expect(&format!("couldn't find {}", current));
            let t = thread::current();
            let thread_name = t.id();
            // if steps % 10000000 == 1 {
                println!("Turning {} to {} on thread {:?}", turn, next, thread_name);
            // }
            current = next;
            steps += 1;
        } else {
            turn_line = turn_backup.clone();
        }
    }
    dbg!(&steps);
    steps
}

// 9:45 I think I'm going to attempt a multithreaded approach
// 10:09 why on earth did i decide that was a good idea
pub fn run() {
    let input = INPUT1;
    let lines = input.lines().collect::<Vec<_>>();

    let turn_line = lines.get(0).expect("no turn line??").to_string().chars().rev().collect::<String>();

    let mut left_map: HashMap<&str, &str> = HashMap::default();
    let mut right_map: HashMap<&str, &str> = HashMap::default();
    for i in 2..lines.len() {
        let line = lines.get(i).expect("aaa where is the line");
        let parts = line.split(" = ").collect::<Vec<_>>();

        let (value, leaves) = match parts.as_slice() {
            [value, leaves] => (value, leaves),
            _ => panic!("Invalid parts"),
        };

        let leaves_parts = leaves.split(", ").collect::<Vec<_>>();
        let (left, right) = match leaves_parts.as_slice() {
            // trim off ()
            [left, right] => (&left[1..], &right[..right.len() - 1]),
            _ => panic!("invalid parts"),
        };

        left_map.insert(value, left);
        right_map.insert(value, right);
    }

    let starting_positions = left_map.keys().filter(|x| x.ends_with("A")).map(|x|*x).collect::<Vec<_>>();

    let shared_positions = Arc::new(Mutex::new(starting_positions.clone()));

    thread::scope(|s| {
        let handles = starting_positions.iter().enumerate().map(|(i, start)| {
            s.spawn({
                let turn_line = turn_line.clone();
                let start_clone = start.to_string();
                || {
                    dbg!("starting search with {}", start_clone);
                    count_steps(start, turn_line, &left_map, &right_map, shared_positions.clone(), i)
                }
            })
        }).collect::<Vec<_>>();
        let sum = handles.into_iter().map(|x|x.join().expect("fail")).sum::<usize>();
        dbg!(sum);
    });
}
