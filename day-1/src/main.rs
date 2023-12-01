use regex::Regex;
use std::{fs, io::Error};

fn main() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().collect();
    let file_name = args.get(1).unwrap();
    let not_numbers = Regex::new(r"[^\d]").unwrap();

    let answer: i32 = fs::read_to_string(file_name)?
        .lines()
        .map(letters_to_numbers)
        .map(|line| not_numbers.replace_all(line.as_str(), "").to_string())
        .map(|line| line.chars().collect())
        .map(|chars: Vec<char>| {
            format!(
                "{}{}",
                chars.first().unwrap().to_digit(10).unwrap(),
                chars.last().unwrap().to_digit(10).unwrap()
            )
        })
        .map(|digits| digits.parse::<i32>().unwrap())
        .sum();
    println!("{}", answer);

    return Ok(());
}

fn letters_to_numbers(line: &str) -> String {
    let pairs = vec![
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    let mut result: String = line.into();
    for (word, digit) in pairs.iter() {
        let chars: Vec<char> = word.chars().collect();
        let first = chars.first().unwrap();
        let last = chars.last().unwrap();
        let replace_with = format!("{}{}{}", first, digit, last);
        result = result.replace(word, &replace_with);
    }

    return result;
}
