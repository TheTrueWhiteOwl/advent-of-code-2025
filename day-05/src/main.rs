#![feature(new_range_api)]

use std::range::RangeInclusive;
use std::cmp::{max, min};

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

fn merge_inclusive_ranges<T>(ranges_iter: T) -> Vec<RangeInclusive<usize>>
    where
        T: Iterator<Item = RangeInclusive<usize>>,
{
    let mut merged_ranges: Vec<RangeInclusive<usize>> = Vec::new();
    for range in ranges_iter {
        let found = merged_ranges.binary_search_by_key(&(range.start, range.last), |probe| (probe.start, probe.last));
        let insert_idx = match found {
            Ok(_) => continue,
            Err(insert_idx) => insert_idx,
        };

        match (merged_ranges.get(insert_idx.wrapping_sub(1)).map(|r| r.last >= range.start-1).unwrap_or(false), merged_ranges.get(insert_idx).map(|r| r.start <= range.last+1).unwrap_or(false)) {
            (true, true) => {
                let after_range = merged_ranges.get(insert_idx).unwrap();
                let last = max(after_range.last, range.last);
                let before_range = merged_ranges.get_mut(insert_idx-1).unwrap();
                before_range.start = min(before_range.start, range.start);
                before_range.last = last;
                merged_ranges.remove(insert_idx);
            },
            (true, false) => {
                let before_range = merged_ranges.get_mut(insert_idx-1).unwrap();
                before_range.last = max(before_range.last, range.last);
            },
            (false, true) => {
                let after_range = merged_ranges.get_mut(insert_idx).unwrap();
                after_range.start = min(after_range.start, range.start);
                after_range.last = max(after_range.last, range.last);
            },
            (false, false) => {
                merged_ranges.insert(insert_idx, range);
            },
        }
    }
    merged_ranges
}



fn part1(input: String) {
    let (fresh_ingredient_ranges_str, available_ingredients) = input.split_once("\n\n").expect("An empty line always separates both halves of the instructions");
    let ranges_iter = fresh_ingredient_ranges_str
        .lines()
        .map(|s| {
            let (lower, upper) = s.split_once('-').expect("All ranges will be split at a '-'");
            RangeInclusive {
                start: lower.parse::<usize>().expect("Always valid"),
                last: upper.parse::<usize>().expect("Always valid"),
            }
        });
    let fresh_ingredient_ranges: Vec<RangeInclusive<usize>> = merge_inclusive_ranges(ranges_iter);

    let mut fresh_ingredient_count = 0;
    for ingredient_id in available_ingredients.lines().map(|id| id.parse::<usize>().expect("Always valid")) {
        if fresh_ingredient_ranges.iter().any(|range| range.contains(&ingredient_id)) {
            fresh_ingredient_count += 1;
        }
    }

    println!("{}", fresh_ingredient_count);
}

fn part2(input: String) {
    let (fresh_ingredient_ranges_str, _) = input.split_once("\n\n").expect("An empty line always separates both halves of the instructions");
    let ranges_iter = fresh_ingredient_ranges_str
        .lines()
        .map(|s| {
            let (lower, upper) = s.split_once('-').expect("All ranges will be split at a '-'");
            RangeInclusive {
                start: lower.parse::<usize>().expect("Always valid"),
                last: upper.parse::<usize>().expect("Always valid"),
            }
        });
    let fresh_ingredient_ranges: Vec<RangeInclusive<usize>> = merge_inclusive_ranges(ranges_iter);

    let total_fresh_ingredients: usize = fresh_ingredient_ranges.iter().map(|range| range.iter().count()).sum();

    println!("{}", total_fresh_ingredients);
}
