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
    let input = input.trim();
    let mut invalid_ids = 0;
    for range in input.split(',') {
        let (start, end) = range.split_once('-').expect("All ranges will include a '-'");
        let start_int: usize = start.parse().expect("This is a valid number");
        let end_int: usize = end.parse().expect("This is a valid number");
        for id in start_int..=end_int {
            let id_str = format!("{}", id);
            if id_str.len() % 2 == 0 {
                let (left, right) = id_str.split_at(id_str.len() / 2);
                if left == right {
                    invalid_ids += id;
                }
            }
        }
    }
    println!("{}", invalid_ids);
}

fn part2(input: String) {
    let input = input.trim();
    let mut invalid_ids = 0;
    for range in input.split(',') {
        let (start, end) = range.split_once('-').expect("All ranges will include a '-'");
        let start_int: usize = start.parse().expect("This is a valid number");
        let end_int: usize = end.parse().expect("This is a valid number");
        for id in start_int..=end_int {
            let id_str = format!("{}", id);
            'outer: for i in (1..=id_str.len()/2).filter(|x| id_str.len()%x==0) {
                let first_part = &id_str[..i];
                for j in (i..id_str.len()).step_by(i) {
                    let part = &id_str[j..j+i];
                    if first_part != part {
                        continue 'outer;
                    }
                }
                invalid_ids += id;
                break;
            }
        }
    }
    println!("{}", invalid_ids);
}
