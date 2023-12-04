use std::{collections::HashSet, fs};

pub struct Scratchcard {
    winning_numbers: HashSet<u32>,
    my_numbers: HashSet<u32>,
}

impl Scratchcard {
    pub fn parse(row: &str) -> Scratchcard {
        let parts: Vec<&str> = row.split("|").collect();
        let winning_numbers = Scratchcard::parse_numbers(parts[0]);
        let my_numbers = Scratchcard::parse_numbers(parts[1]); 
        Scratchcard { winning_numbers, my_numbers }
    }

    fn parse_numbers(numbers: &str) -> HashSet<u32> {
        numbers
            .trim()
            .split(' ')
            .filter(|number| !number.is_empty())
            .map(|number| number.parse().unwrap())
            .collect()
    }

    pub fn num_matches(&self) -> u32 {
        self.winning_numbers.intersection(&self.my_numbers).count() as u32
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_name = args.get(1).unwrap();

    let total_score: u32 = fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|line| line.split(":").last().unwrap())
        .map(|numbers| Scratchcard::parse(numbers))
        .map(|card| card.num_matches())
        .filter(|num_matches| *num_matches != 0)
        .map(|num_matches| 2u32.pow(num_matches - 1))
        .sum();
    println!("Total: {}", total_score)
}
