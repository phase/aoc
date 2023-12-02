const INPUT: &'static str = include_str!("./day1_input_unique.txt");

pub fn run() {
    let mut total_acc = 0;

    let mut digit_acc = [0u32, 0u32];
    let mut digit_count = 0u32;
    let mut last_value = 0u32;

    // to get part 1's answer, don't do this preprocessing
    let mut input = INPUT
        // these top three stumped me!
        // it's expecting overlapping numbers to treated as *both* possibilities
        .replace("twone", "21")
        .replace("eightwo", "82")
        .replace("oneight", "18")
        // normal 1-9
        .replace("one", "1")
        .replace("two", "2")
        .replace("three", "3")
        .replace("four", "4")
        .replace("five", "5")
        .replace("six", "6")
        .replace("seven", "7")
        .replace("eight", "8")
        .replace("nine", "9");

    // add a \0 to act as an EOF flag in the for loop
    // it's treated as if the file ended with a \n
    input.push('\0');

    for byte in input.as_bytes().iter() {
        let c = *byte;

        if c >= b'0' && c <= b'9' {
            let value = (c as u32) - (b'0' as u32);
            if digit_count == 0 {
                digit_acc[0] = value;
            }
            last_value = value;
            digit_count += 1;
        } else if c == b'\n' || c == b'\0' {
            // use the second digit and the stored first digit
            // to build the number we add to the total accumulator
            digit_count = 0;
            digit_acc[1] = last_value;
            total_acc += (digit_acc[0] * 10) + digit_acc[1];

            // reset trackers
            digit_acc = [0, 0];
            last_value = 0;
        }
    }

    dbg!(total_acc);
}
