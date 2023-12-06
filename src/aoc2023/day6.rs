
const INPUT1: &'static str = include_str!("./day6_input1.txt");
const INPUTU: &'static str = include_str!("./day6_input_unique.txt");

// 9:20pm one star!
pub fn _part1() {
    let input = INPUTU;

    let lines = input.lines().map(|x|{
        x.split(": ").collect::<Vec<_>>()[1].trim().split(" ").filter(|x|!x.trim().is_empty()).map(|x|x.trim().parse::<usize>().expect("not a num")).collect::<Vec<usize>>()
    }).collect::<Vec<_>>();
    let times = &lines[0];
    let distances = &lines[1];
    dbg!(&times);

    let mut total_wins = 1;
    for (i, time) in times.iter().enumerate() {
        let distance = distances[i];
        println!("Checking record time={} distance={}", time, distance);
        let mut wins = 0usize;
        for t in 0..*time {
            let d = (time - t) * t;
            if d > distance {
                wins += 1;
            }
        }
        total_wins *= wins;
    }
    dbg!(total_wins);
}

// 9:23pm second star!
pub fn run() {
    let input = INPUTU;

    let lines = input.lines().map(|x|{
        x.split(": ").collect::<Vec<_>>()[1].replace(" ", "").parse::<usize>().expect("not a num")
    }).collect::<Vec<_>>();
    let time = &lines[0];
    let distance = &lines[1];
    dbg!(&time);

    let mut total_wins = 1;
    //for (i, time) in times.iter().enumerate() {
     //   let distance = distances[i];
        println!("Checking record time={} distance={}", time, distance);
        let mut wins = 0usize;
        for t in 0..*time {
            let d = (time - t) * t;
            if d > *distance {
                wins += 1;
            }
        }
        total_wins *= wins;

    dbg!(total_wins);
}