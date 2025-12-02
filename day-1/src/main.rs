const SAFE_STARTING_SPOT: usize = 50;
const SAFE_WHEEL_SIZE: usize = 100;

#[derive(PartialEq, Eq)]
struct Rollover<const N: usize> ( usize );

impl<const N: usize> Rollover<N> {
    const fn new(n: usize) -> Option<Self> {
        if n >= N {
            None
        } else {
            Some(Rollover(n))
        }
    }

    fn count_rollover_add(self, other: usize) -> (usize, Self) {
        let rollover_count = (self.0 + other) / N;
        (rollover_count, self + other)
    }

    fn count_rollover_sub(self, other: usize) -> (usize, Self) {
        let mut rollover_count = other / N;
        if self.0 < other % N {
            rollover_count += 1
        }
        (rollover_count, self - other)
    }
}

impl<const N: usize> std::ops::Add<usize> for Rollover<N> {
    type Output = Self;
    fn add(self, other: usize) -> Self {
        Rollover::new((self.0 + other) % N).expect("Can't fail because I am doing the modulo operation")
    }
}

impl<const N: usize> std::ops::AddAssign<usize> for Rollover<N> {
    fn add_assign(&mut self, other: usize) {
        self.0 = (self.0 + other) % N;
    }
}

impl<const N: usize> std::ops::Sub<usize> for Rollover<N> {
    type Output = Self;
    fn sub(self, other: usize) -> Self {
        Rollover::new((self.0 + N - other % N) % N).expect("Can't fail because I am doing the modulo operation")
    }
}

impl<const N: usize> std::ops::SubAssign<usize> for Rollover<N> {
    fn sub_assign(&mut self, other: usize) {
        self.0 = (self.0 + N - other % N) % N;
    }
}

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
    let mut current_safe_spot = Rollover::<SAFE_WHEEL_SIZE>::new(SAFE_STARTING_SPOT).expect("I am not dumb");
    let mut zero_count = 0;
    for line in input.lines() {
        let mut line_chars = line.chars();
        match line_chars.next() {
            Some('L') => {current_safe_spot -= line_chars.collect::<String>().parse::<usize>().expect("All instructions should be valid... right?")},
            Some('R') => {current_safe_spot += line_chars.collect::<String>().parse::<usize>().expect("All instructions should be valid... right?")},
            Some(_) | None => panic!(),
        }
        if current_safe_spot.0 == 0 {
            zero_count += 1
        }
    }

    println!("{}", zero_count);
}

fn part2(input: String) {
    let mut current_safe_spot = Rollover::<SAFE_WHEEL_SIZE>::new(SAFE_STARTING_SPOT).expect("I am not dumb");
    let mut zero_count = 0;
    let mut rollovers;
    for line in input.lines() {
        let mut line_chars = line.chars();
        match line_chars.next() {
            Some('L') => {
                if current_safe_spot.0 == 0 {
                    zero_count -= 1
                }
                (rollovers, current_safe_spot) = current_safe_spot.count_rollover_sub(line_chars.collect::<String>().parse::<usize>().expect("All instructions should be valid... right?"));
                zero_count += rollovers;
                if current_safe_spot.0 == 0 {
                    zero_count += 1
                }
            },
            Some('R') => {
                (rollovers, current_safe_spot) = current_safe_spot.count_rollover_add(line_chars.collect::<String>().parse::<usize>().expect("All instructions should be valid... right?"));
                zero_count += rollovers;
            },
            Some(_) | None => panic!(),
        }

    }

    println!("{}", zero_count);
}
