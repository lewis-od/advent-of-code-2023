use std::{collections::HashSet, fs, iter};

pub struct Grid<'a> {
    rows: &'a Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl<'a> Grid<'a> {
    pub fn new(rows: &'a Vec<Vec<char>>) -> Grid {
        let height = rows.len();
        let width = rows[0].len();
        Grid {
            rows,
            width,
            height,
        }
    }

    pub fn adjacent_values(&self, x: usize, y: usize) -> Vec<char> {
        self.adjacent_points(x, y)
            .iter()
            .map(|(x_adj, y_adj)| self.rows[*y_adj][*x_adj])
            .collect()
    }

    pub fn adjacent_points(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let delta: Vec<i32> = vec![-1, 0, 1];
        let mut adjacent = vec![];
        for dy in delta.iter() {
            for dx in delta.iter() {
                if (x as i32) + dx < 0 || (y as i32) + dy < 0 {
                    continue;
                }
                let y_adj = ((y as i32) + dy) as usize;
                let x_adj = ((x as i32) + dx) as usize;
                if x_adj < self.width && y_adj < self.height {
                    adjacent.push((x_adj, y_adj))
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
    let mut part_number_grid = create_part_number_grid(grid.width, grid.height);
    for (y, row) in grid.rows.iter().enumerate() {
        let mut number: Vec<char> = vec![];
        let mut is_part_number = false;
        for (x, char) in row.iter().enumerate() {
            match char.to_digit(10) {
                Some(_) => number.push(*char),
                None => {
                    if is_part_number {
                        record_part_number(&mut part_numbers, &mut part_number_grid, x, y, &number);
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
            record_part_number(
                &mut part_numbers,
                &mut part_number_grid,
                grid.width - 1,
                y,
                &number,
            );
        }
    }

    let sum: u32 = part_numbers.iter().sum();
    println!("Sum: {}", sum);

    let mut sum_of_gear_ratios: u32 = 0;
    for (y, row) in grid.rows.iter().enumerate() {
        for (x, character) in row.iter().enumerate() {
            if *character != '*' {
                continue;
            }
            let adjacent_points = grid.adjacent_points(x, y);
            let adjacent_part_numbers: HashSet<u32> = adjacent_points
                .iter()
                .map(|(dx, dy)| part_number_grid[*dy][*dx])
                .filter(|val| val.is_some())
                .map(|val| val.unwrap())
                .collect();
            if adjacent_part_numbers.len() == 2 {
                sum_of_gear_ratios += adjacent_part_numbers.iter().fold(1, |acc, v| acc * v)
            }
        }
    }

    println!("Gears: {}", sum_of_gear_ratios);
}

fn record_part_number(
    part_numbers: &mut Vec<u32>,
    grid: &mut Vec<Vec<Option<u32>>>,
    x: usize,
    y: usize,
    number: &Vec<char>,
) {
    let part_number = chars_to_number(number);
    part_numbers.push(part_number);
    for dx in 0..number.len() {
        grid[y][x - 1 - dx] = Some(part_number);
    }
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

fn create_part_number_grid(width: usize, height: usize) -> Vec<Vec<Option<u32>>> {
    iter::repeat(iter::repeat(None).take(width).collect())
        .take(height)
        .collect()
}
