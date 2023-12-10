use std::fs;
use std::fs::read;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_name = args.get(1).unwrap();

    let file = fs::read_to_string(file_name).unwrap();
    let readings: Vec<Vec<i64>> = file.lines().map(|line| parse_row(line)).collect();
    let answer: i64 = readings.iter().map(|reading| get_next(reading)).sum();
    println!("Part 1: {answer}");

    let answer: i64 = readings.iter().map(|reading| get_previous(reading)).sum();
    println!("Part 2: {answer}");
}

fn parse_row(line: &str) -> Vec<i64> {
    line.split_ascii_whitespace()
        .map(|value| value.parse().unwrap())
        .collect()
}

fn get_next(sequence: &Vec<i64>) -> i64 {
    if all_same(sequence) {
        return sequence[0];
    }

    let differences: Vec<i64> = sequence.windows(2).map(|pair| pair[1] - pair[0]).collect();
    sequence.last().unwrap() + get_next(&differences)
}

fn get_previous(sequence: &Vec<i64>) -> i64 {
    if all_same(sequence) {
        return sequence[0];
    }

    let differences: Vec<i64> = sequence.windows(2).map(|pair| pair[1] - pair[0]).collect();
    sequence[0] - get_previous(&differences)
}

fn all_same(vec: &Vec<i64>) -> bool {
    let first = vec[1];
    vec.iter().all(|elem| first == *elem)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_pass_example1() {
        let sequence = vec![0, 3, 6, 9, 12, 15];

        let next = get_next(&sequence);

        assert_eq!(18, next);
    }

    #[test]
    fn should_pass_example2() {
        let sequence = vec![1, 3, 6, 10, 15, 21];

        let next = get_next(&sequence);

        assert_eq!(28, next);
    }

    #[test]
    fn should_pass_example3() {
        let sequence = vec![10, 13, 16, 21, 30, 45];

        let next = get_next(&sequence);

        assert_eq!(68, next);
    }

    #[test]
    fn should_parse_row() {
        let row = "1 25 28 16 -15 -58";

        let sequence = parse_row(row);

        let expected: Vec<i64> = vec![1, 25, 28, 16, -15, -58];
        assert_eq!(expected, sequence);
    }

    #[test]
    fn should_pass_example4() {
        let sequence = vec![10, 13, 16, 21, 30, 45];

        let previous = get_previous(&sequence);

        assert_eq!(5, previous);
    }
}
