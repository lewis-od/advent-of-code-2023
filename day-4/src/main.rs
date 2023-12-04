use std::{collections::HashSet, fs};

#[derive(Clone)]
pub struct Scratchcard {
    number: usize,
    num_matches: usize,
}

impl Scratchcard {
    pub fn parse(number: usize, row: &str) -> Scratchcard {
        let parts: Vec<&str> = row.split("|").collect();
        let winning_numbers = Scratchcard::parse_numbers(parts[0]);
        let my_numbers = Scratchcard::parse_numbers(parts[1]);
        let num_matches = Scratchcard::num_matches(winning_numbers, my_numbers);
        Scratchcard {
            number,
            num_matches,
        }
    }

    fn parse_numbers(numbers: &str) -> HashSet<u32> {
        numbers
            .trim()
            .split(' ')
            .filter(|number| !number.is_empty())
            .map(|number| number.parse().unwrap())
            .collect()
    }

    fn num_matches(winning_numbers: HashSet<u32>, my_numbers: HashSet<u32>) -> usize {
        winning_numbers.intersection(&my_numbers).count()
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_name = args.get(1).unwrap();

    let all_cards: Vec<Scratchcard> = fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|line| line.split(":").last().unwrap())
        .enumerate()
        .map(|(row_num, values)| Scratchcard::parse(row_num + 1, values))
        .collect();

    let total_score: u32 = all_cards
        .iter()
        .map(|card| card.num_matches)
        .filter(|num_matches| *num_matches != 0)
        .map(|num_matches| 2u32.pow((num_matches - 1) as u32))
        .sum();
    println!("Part 1: {}", total_score);

    let final_cards = score_cards(&all_cards, all_cards.clone(), all_cards.len());
    println!("Part 2: {}", final_cards);
}

fn score_cards(
    all_cards: &Vec<Scratchcard>,
    my_cards: Vec<Scratchcard>,
    count: usize,
) -> usize {
    let mut copies: Vec<Scratchcard> = vec![];
    for scratchcard in my_cards.iter() {
        let num_copies = scratchcard.num_matches;
        if num_copies == 0 {
            continue;
        }

        let from = scratchcard.number;
        let until = scratchcard.number + num_copies;
        let mut to_add: Vec<Scratchcard> = all_cards[from..until]
            .iter()
            .map(|c| c.clone())
            .collect();
        copies.append(&mut to_add)
    }

    let new_count = count + copies.len();
    if copies.len() == 0 {
        return new_count;
    }

    score_cards(all_cards, copies, new_count)
}
