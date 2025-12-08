#![feature(iter_array_chunks)]

use std::cell::{RefCell};
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::rc::Rc;

use itertools::Itertools;

#[derive(Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Position(u32, u32, u32);

impl Position {
    fn dist(&self, other: Self) -> f64 {
        f64::from(self.0.abs_diff(other.0).pow(2) + self.1.abs_diff(other.1).pow(2) + self.2.abs_diff(other.2).pow(2)).sqrt()
    }
}

impl From<[u32; 3]> for Position {
    fn from(arr: [u32; 3]) -> Self {
        Position(arr[0], arr[1], arr[2])
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
struct OrdF64(f64);

impl Eq for OrdF64 {} // I am ignoring the existence of NaN

impl Ord for OrdF64 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).expect("I don't expect to encounter any NaN")
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
    let mut distances: BTreeMap<OrdF64, (Position, Position)> = BTreeMap::new();
    let positions = input.lines()
        .flat_map(|line| line.split(','))
        .map(|num| num.parse::<u32>().expect("All valid nums"))
        .array_chunks::<3>()
        .map(Position::from)
        .collect::<Vec<_>>();
    for (i, &pos) in positions.iter().enumerate() {
        for &other_pos in positions[i+1..].iter() {
            distances.insert(OrdF64(pos.dist(other_pos)), (pos, other_pos));
        }
    }

    let mut pos_to_circuit: HashMap<Position, Rc<RefCell<HashSet<Position>>>> = positions.into_iter().map(|pos| (pos, Rc::new(RefCell::new(HashSet::from([pos]))))).collect();
    for (pos_1, pos_2) in distances.values().take(1000) {
        let circuit_1 = pos_to_circuit.get(pos_1).unwrap().clone();
        let circuit_2 = pos_to_circuit.get(pos_2).unwrap().clone();
        if circuit_1 == circuit_2 {
            continue
        }
        let (small, large) = if circuit_1.borrow().len() < circuit_2.borrow().len() {
            (circuit_1, circuit_2)
        } else {
            (circuit_2, circuit_1)
        };
        for pos in small.borrow().iter() {
            large.borrow_mut().insert(*pos);
            let circuit = pos_to_circuit.get_mut(pos).unwrap();
            *circuit = large.clone();
        }
    }
    let mut circuit_lens = pos_to_circuit.values().unique_by(|rc_1| Rc::as_ptr(rc_1)).map(|rc_refcell_circuit| rc_refcell_circuit.borrow().len()).collect::<Vec<usize>>();
    circuit_lens.sort_unstable();
    println!("{:?}", circuit_lens);
    assert_eq!(circuit_lens.iter().sum::<usize>(), input.lines().count());
    let final_answer: usize = circuit_lens.iter().rev().take(3).fold(1, |acc, x| acc*x);
    println!("{}", final_answer);
}

fn part2(input: String) {
    todo!()
}
