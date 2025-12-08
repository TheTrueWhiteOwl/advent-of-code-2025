#![feature(iter_array_chunks)]

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Position(u32, u32, u32);

impl Position {
    fn idist(&self, other: Self) -> u32 {
        (self.0.abs_diff(other.0).pow(2) + self.1.abs_diff(other.1).pow(2) + self.2.abs_diff(other.2).pow(2)).isqrt()
    }
}

impl From<[u32; 3]> for Position {
    fn from(arr: [u32; 3]) -> Self {
        Position(arr[0], arr[1], arr[2])
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
    let mut cached_distances: HashMap<(Position, Position), u32> = HashMap::new();
    let mut positions = input.lines()
        .flat_map(|line| line.split(','))
        .map(|num| num.parse::<u32>().expect("All valid nums"))
        .array_chunks::<3>()
        .map(Position::from)
        .collect::<Vec<_>>();
    positions.sort_unstable();
    for (i, &pos) in positions.iter().enumerate() {
        for &other_pos in positions[i..].iter() {
            cached_distances.insert((pos, other_pos), pos.idist(other_pos));
        }
    }
    let circuits: HashMap<Position, Rc<RefCell<u32>>> = positions.into_iter().zip((0..).map(|idx| Rc::new(RefCell::new(idx)))).collect();
    println!("how long was that?");
}

fn part2(input: String) {
    todo!()
}
