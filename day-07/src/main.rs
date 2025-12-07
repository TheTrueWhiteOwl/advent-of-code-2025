use std::collections::HashSet;
use std::collections::HashMap;

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
    let tachyon_beam_start_location = input.lines()
        .next()
        .expect("There is always more than one line")
        .find('S')
        .expect("The first line always includes the start position");

    let mut tachyon_beam_locations = HashSet::from([tachyon_beam_start_location]);

    let mut split_count = 0;

    for line in input.lines().skip(1) {
        for (splitter_position, _) in line.match_indices('^') {
            if tachyon_beam_locations.contains(&splitter_position) {
                split_count += 1;
                // splitters are never touching eachother, or the edge, so this is fine
                tachyon_beam_locations.remove(&splitter_position);
                tachyon_beam_locations.insert(splitter_position-1);
                tachyon_beam_locations.insert(splitter_position+1);
            }
        }
    }
    println!("{}", split_count);
}

fn part2(input: String) {
    let tachyon_beam_start_location = input.lines()
        .next()
        .expect("There is always more than one line")
        .find('S')
        .expect("The first line always includes the start position");

    let mut tachyon_beam_locations = HashMap::from([(tachyon_beam_start_location, 1)]);

    for line in input.lines().skip(1) {
        for (splitter_position, _) in line.match_indices('^') {
            if let Some(count) = tachyon_beam_locations.remove(&splitter_position) {
                // splitters are never touching eachother, or the edge, so this is fine
                tachyon_beam_locations.entry(splitter_position-1).and_modify(|prev_count| *prev_count +=  count).or_insert(count);
                tachyon_beam_locations.entry(splitter_position+1).and_modify(|prev_count| *prev_count +=  count).or_insert(count);
            }
        }
    }
    println!("{}", tachyon_beam_locations.values().sum::<usize>());
}
