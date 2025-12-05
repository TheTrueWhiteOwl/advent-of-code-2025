use day_01::Wrapping;

const STARTING_SPOT: usize = 50;
const WHEEL_SIZE: usize = 100;
const ZERO: Wrapping<WHEEL_SIZE> = unsafe { Wrapping::new_unchecked(0) };

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input_filepath = args.get(1).expect("Must pass at least one parameter for the filepath");

    let input: String = std::fs::read_to_string(input_filepath).expect("The filepath given was not valid");

    let part2_flag: Option<&str> = args.get(2).map(String::as_str);

    match part2_flag {
        Some("--part2") => part2(input),
        Some(_) => panic!("wtf are you trying to do?"),
        None => part1(input),
    }
}

fn part1(input: String) {
    let mut current_safe_spot = Wrapping::<WHEEL_SIZE>::new(STARTING_SPOT).expect("I am not dumb");
    let mut zero_count = 0;
    for line in input.lines() {
        let mut line_chars = line.chars();
        match line_chars.next() {
            Some('L') => {current_safe_spot -= line_chars.collect::<String>().parse::<usize>().expect("All instructions should be valid... right?")},
            Some('R') => {current_safe_spot += line_chars.collect::<String>().parse::<usize>().expect("All instructions should be valid... right?")},
            Some(_) | None => unreachable!("Every instruction will always start with either an L or an R"),
        }
        if current_safe_spot == ZERO {
            zero_count += 1
        }
    }

    println!("{}", zero_count);
}

fn part2(input: String) {
    let mut current_safe_spot = Wrapping::<WHEEL_SIZE>::new(STARTING_SPOT).expect("I am not dumb");
    let mut zero_count = 0;
    let mut wraps;
    for line in input.lines() {
        let mut line_chars = line.chars();
        match line_chars.next() {
            Some('L') => {
                if current_safe_spot == ZERO {
                    zero_count -= 1
                }
                (current_safe_spot, wraps) = current_safe_spot.wrap_counting_sub(line_chars.collect::<String>().parse::<usize>().expect("All instructions should be valid... right?"));
                zero_count += wraps;
                if current_safe_spot == ZERO {
                    zero_count += 1
                }
            },
            Some('R') => {
                (current_safe_spot, wraps) = current_safe_spot.wrap_counting_add(line_chars.collect::<String>().parse::<usize>().expect("All instructions should be valid... right?"));
                zero_count += wraps;
            },
            Some(_) | None => unreachable!("Every instruction will always start with either an L or an R"),
        }

    }

    println!("{}", zero_count);
}
