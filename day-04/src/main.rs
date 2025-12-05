struct Grid {
    rows: usize,
    cols: usize,
    contents: Vec<bool>,
}

impl Grid {
    fn count_neighbours(&self, row: usize, col: usize) -> u8 {
        let mut count = 0;
        for r in -1..=1 {
            for c in -1..=1 {
                if r == 0 && c == 0 {
                    continue;
                }
                let neighbour_row = row.wrapping_add_signed(r);
                let neighbour_col = col.wrapping_add_signed(c);
                if self.get(neighbour_row, neighbour_col).unwrap_or(false) {
                    count += 1
                }
            }
        }
        count
    }

    fn get(&self, row: usize, col: usize) -> Option<bool> {
        if row >= self.rows || col >= self.cols {
            None
        } else {
            self.contents.get(row*self.cols + col).copied()
        }
    }

    fn set(&mut self, row: usize, col: usize, val: bool) {
        self.contents[row*self.cols + col] = val
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
    let mut accessible_toilet_rolls = 0;

    let rows = input.lines().count();
    let cols = input.lines().next().expect("There will always be at least one line").chars().count();
    assert!(input.lines().all(|l| l.chars().count() == cols));
    let contents: Vec<bool> = input.lines()
        .flat_map(|line| line.chars())
        .map(|symbol| match symbol { '.' => false, '@' => true, _ => unreachable!("The grid only ever contains '.' or '@'") })
        .collect();

    let grid = Grid { rows, cols, contents };
    for row in 0..rows {
        for col in 0..cols {
            if grid.get(row, col).expect("Always within bounds") && grid.count_neighbours(row, col) < 4 {
                accessible_toilet_rolls += 1;
            }
        }
    }
    println!("{}", accessible_toilet_rolls);
}

fn part2(input: String) {
    let rows = input.lines().count();
    let cols = input.lines().next().expect("There will always be at least one line").chars().count();
    assert!(input.lines().all(|l| l.chars().count() == cols));
    let contents: Vec<bool> = input.lines()
        .flat_map(|line| line.chars())
        .map(|symbol| match symbol { '.' => false, '@' => true, _ => unreachable!("The grid only ever contains '.' or '@'") })
        .collect();

    let mut grid = Grid { rows, cols, contents };

    let mut removed_toilet_rolls = 0;
    let mut any_toilet_rolls_removed = true;
    while any_toilet_rolls_removed {
        any_toilet_rolls_removed = false;
        for row in 0..rows {
            for col in 0..cols {
                if grid.get(row, col).expect("Always within bounds") && grid.count_neighbours(row, col) < 4 {
                    grid.set(row, col, false);
                    any_toilet_rolls_removed = true;
                    removed_toilet_rolls += 1;
                }
            }
        }
    }
    println!("{}", removed_toilet_rolls);
}
