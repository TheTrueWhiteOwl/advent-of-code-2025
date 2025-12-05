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
    let mut joltage_sum: u32 = 0;
    for bank in input.lines() {
        let mut joltages = bank[0..bank.len()-1].chars().enumerate();
        let (mut highest_num_1_idx, mut highest_num_1) = joltages.next().expect("Every bank must include more than one joltage");
        for (idx, num) in joltages {
            if num > highest_num_1 {
                highest_num_1 = num;
                highest_num_1_idx = idx;
                if highest_num_1 == '9' {
                    break
                }
            }
        }
        let mut joltages_2 = bank.chars();
        let mut highest_num_2 = joltages_2.nth(highest_num_1_idx+1).expect("Every bank must include more than one joltage");
        for num in joltages_2 {
            if num > highest_num_2 {
                highest_num_2 = num
            }
        }
        let highest_joltage: u32 = [highest_num_1, highest_num_2].iter().collect::<String>().parse().expect("Will always be a valid combination of two chars representing digits");
        joltage_sum += highest_joltage;
    }
    println!("{joltage_sum}");
}

const PART2_DIGITS: usize = 12;

fn part2(input: String) {
    let mut joltage_sum: u64 = 0;
    for bank in input.lines() {
        let mut highest_nums = ['\0'; PART2_DIGITS];
        let mut highest_num_idx: usize = usize::MAX;
        for digit in 0..PART2_DIGITS {
            let mut joltages = bank[0..bank.len()-PART2_DIGITS+digit+1].chars().enumerate();
            let mut highest_num;
            (highest_num_idx, highest_num) = joltages.nth((highest_num_idx.wrapping_add(1)) as usize).expect("Every bank must include at least this many voltages");
            for (idx, num) in joltages {
                if num > highest_num {
                    highest_num = num;
                    highest_num_idx = idx;
                    if highest_num == '9' {
                        break
                    }
                }
            }
            highest_nums[digit] = highest_num;
        }
        let highest_joltage: u64 = highest_nums.iter().collect::<String>().parse().expect("Will always be a valid combination of chars representing digits");
        joltage_sum += highest_joltage;
    }
    println!("{joltage_sum}");
}
