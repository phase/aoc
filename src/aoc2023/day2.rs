const INPUT: &'static str = include_str!("./day2_input_unique.txt");

pub fn _part1() {
    let real_red_cubes = 12;
    let real_green_cubes = 13;
    let real_blue_cubes = 14;

    let sum: u32 = INPUT.lines()
        .map(|line| {
            let colon_idx = line.find(":").expect("no game ID?");
            let space_idx = line.find(" ").expect("space??") + 1;

            let game_id: u32 = (&line[space_idx..colon_idx]).parse().expect("failed to parse int");
            let game_state = &line[colon_idx+2..];

            let valid = !game_state.split("; ")
                .flat_map(|set| {
                    set.split(", ")
                        .map(|colored_value| {
                            let space_idx = colored_value.find(" ").expect("no space here?");
                            let value: u32 = (&colored_value[..space_idx]).parse().expect("failed to parse inner");
                            let color = &colored_value[space_idx..];

                            if color.ends_with("green") {
                                value <= real_green_cubes
                            } else if color.ends_with("red") {
                                value <= real_red_cubes
                            } else if color.ends_with("blue") {
                                value <= real_blue_cubes
                            } else {
                                false
                            }
                        })
                })
                .any(|x|!x);


            if valid {
                game_id
            } else {
                0
            }
        })
        .sum();
    dbg!(sum);
}


pub fn run() {

    let sum: u32 = INPUT.lines()
        .map(|line| {
            let colon_idx = line.find(":").expect("no game ID?");
            let game_state = &line[colon_idx+2..];

            let mut max_red_cubes = 0;
            let mut max_green_cubes = 0;
            let mut max_blue_cubes = 0;

            game_state.split("; ")
                .for_each(|set| {
                    set.split(", ")
                        .for_each(|colored_value| {
                            let space_idx = colored_value.find(" ").expect("no space here?");
                            let value: u32 = (&colored_value[..space_idx]).parse().expect("failed to parse inner");
                            let color = &colored_value[space_idx..];

                            if color.ends_with("green") {
                                max_green_cubes = max_green_cubes.max(value);
                            } else if color.ends_with("red") {
                                max_red_cubes = max_red_cubes.max(value);
                            } else if color.ends_with("blue") {
                                max_blue_cubes = max_blue_cubes.max(value);
                            }
                        })
                });


            max_red_cubes * max_green_cubes * max_blue_cubes
        })
        .sum();
    dbg!(sum);
}
