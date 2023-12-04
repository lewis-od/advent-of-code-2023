use std::{collections::HashSet, fs};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_name = args.get(1).unwrap();

    let total_score: u32 = fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|line| line.split(":").last().unwrap())
        .map(|numbers| {
            let parts: Vec<&str> = numbers.split("|").collect();
            let winning_numbers = parse_numbers(parts[0]);
            let my_numbers = parse_numbers(parts[1]);
            winning_numbers.intersection(&my_numbers).count()
        })
        .filter(|num_matches| *num_matches != 0)
        .map(|num_matches| 2u32.pow((num_matches - 1) as u32))
        .sum();
    println!("Total: {}", total_score)
}

fn parse_numbers(numbers: &str) -> HashSet<u32> {
    numbers
        .trim()
        .split(' ')
        .filter(|number| !number.is_empty())
        .map(|number| number.parse().unwrap())
        .collect()
}
