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
    let mut solutions: u64 = 0;
    let cols = input.lines().next().expect("There is definitely more than one line").split_whitespace().count();
    for col in 0..cols {
        let mut exercise_str_vec = input.split_whitespace().skip(col).step_by(cols).collect::<Vec<&str>>();
        let operator = exercise_str_vec.pop().expect("The last element of the exercise is the operator");
        let exercise_iter = exercise_str_vec.into_iter().map(str::parse::<u64>).map(Result::unwrap);
        let solution: u64 = match operator {
            "+" => exercise_iter.sum(),
            "*" => exercise_iter.fold(1, |acc, x| acc * x),
            _ => unreachable!("Only + or * should be possible"),
        };
        solutions += solution;
    }
    println!("{}", solutions);
}

fn part2(input: String) {
    let mut solutions: u64 = 0;
    let cols = input.lines().next().expect("There is definitely more than one line").chars().count() + 1; // plus one to account for the newline character
    let mut exercise = Vec::new();
    let mut current_operator = None;
    for col in 0..cols {
        let value_raw = input.chars().skip(col).step_by(cols).collect::<String>();
        let value;
        if exercise.is_empty() {
            current_operator = value_raw.chars().last();
            value = value_raw[..value_raw.len()-1].trim();
        } else {
            value = value_raw.trim();
        }
        if !value.is_empty() {
            exercise.push(value.parse::<u64>().expect("This is always a valid number"));
        } else {
            let solution: u64 = match current_operator {
                Some('+') => exercise.into_iter().sum(),
                Some('*') => exercise.into_iter().fold(1, |acc, x| acc * x),
                Some(_) => unreachable!("Only + or * should be possible"),
                None => panic!("My code is wrong"),
            };
            solutions += solution;
            exercise = Vec::new();
        }
    }
    println!("{}", solutions);
}
