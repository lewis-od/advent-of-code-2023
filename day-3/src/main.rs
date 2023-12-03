use std::fs;

pub struct Grid<'a> {
    rows: &'a Vec<Vec<char>>,
}

impl<'a> Grid<'a> {
    pub fn new(rows: &'a Vec<Vec<char>>) -> Grid {
        Grid { rows }
    }

    pub fn adjacent_values(&self, x: usize, y: usize) -> Vec<char> {
        let mut adjacent: Vec<char> = vec![];
        let delta: Vec<i32> = vec![-1, 0, 1];
        for dy in delta.iter() {
            for dx in delta.iter() {
                if (x as i32) + dx < 0 || (y as i32) + dy < 0 {
                    continue;
                }
                let row = match self.rows.get(((y as i32) + dy) as usize) {
                    Some(value) => value,
                    None => continue,
                };
                match row.get(((x as i32) + dx) as usize) {
                    Some(value) => adjacent.push(*value),
                    None => continue,
                }
            }
        }
        adjacent
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_name = args.get(1).unwrap();

    let grid: Vec<Vec<char>> = fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let grid = Grid::new(&grid);

    let mut part_numbers: Vec<u32> = vec![];
    for (y, row) in grid.rows.iter().enumerate() {
        let mut number: Vec<char> = vec![];
        let mut is_part_number = false;
        for (x, char) in row.iter().enumerate() {
            match char.to_digit(10) {
                Some(_) => number.push(*char),
                None => {
                    if is_part_number {
                        part_numbers.push(chars_to_number(&number));
                    }
                    number = vec![];
                    is_part_number = false;
                    continue;
                }
            }
            let adjacent = grid.adjacent_values(x, y);
            if adjacent.iter().any(|c| is_symbol(c)) {
                is_part_number = true;
            }
        }
        if number.len() > 0 && is_part_number {
            part_numbers.push(chars_to_number(&number))
        }
    }

    let sum: u32 = part_numbers.iter().sum();
    println!("Sum: {}", sum);
}

fn is_symbol(character: &char) -> bool {
    if character.to_digit(10).is_some() {
        return false;
    }
    return *character != '.';
}

fn chars_to_number(chars: &Vec<char>) -> u32 {
    chars.iter().collect::<String>().parse().unwrap()
}
