use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_name = args.get(1).unwrap();

    let lines = fs::read_to_string(file_name).unwrap();
    let mut lines = lines.lines();

    let times = lines.next().unwrap().replace("Time:", "");
    let times = times.trim();
    let times = parse_row("Time: ", times);

    let distances = lines.next().unwrap().replace("Distance:", "");
    let distances = distances.trim();
    let distances = parse_row("Distance:", distances);

    let result: u64 = times.iter().zip(distances)
        .map(|(time, distance)| num_solutions(*time, distance))
        .product();
    println!("Part 1: {result}");

    // Got lazy and hard-coded instead of parsing
    let result = num_solutions(61677571, 430103613071150);
    println!("Part 2: {result}");
}

fn parse_row(header: &str, row: &str) -> Vec<u64> {
    row.replace(header, "")
        .trim()
        .split_ascii_whitespace()
        .map(|number| number.parse().unwrap())
        .collect()
}

fn num_solutions(race_time: u64, distance: u64) -> u64 {
    match solve_race(race_time, distance) {
        Some((min, max)) => max - min + 1,
        None => 0,
    }
}

// h := hold time
// T := race time
// d := distance
// d = h * (T - h) = -h^2 + Th
// => h^2 - Th + d = 0
#[allow(non_snake_case)]
fn solve_race(T: u64, d: u64) -> Option<(u64, u64)> {
    let T = T as f64;
    let d = d as f64;

    let discriminant = (-T).powi(2) - 4f64 * d;
    if discriminant < 0f64 {
        return None;
    }

    let min_solution = (T - discriminant.sqrt()) / 2f64;
    let max_solution = (T + discriminant.sqrt()) / 2f64;

    let min_solution_int = min_solution.ceil();
    let min_solution_int = if min_solution == min_solution_int {
        min_solution_int + 1f64
    } else {
        min_solution_int
    } as u64;

    let max_solution_int = max_solution.floor();
    let max_solution_int = if max_solution == max_solution_int {
        max_solution_int - 1f64
    } else {
        max_solution_int
    } as u64;

    Some((min_solution_int, max_solution_int))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_row() {
        let row = "Time:      7  15   30";

        let times = parse_row("Time:", row);

        let expected_times: Vec<u64> = vec![7, 15, 30];
        assert_eq!(expected_times, times);
    }

    #[test]
    fn should_solve_race_with_solution() {
        let result = solve_race(7, 9);

        assert!(result.is_some());
        let (min, max) = result.unwrap();
        assert_eq!((2, 5), result.unwrap());
    }

    #[test]
    fn should_handle_when_solutions_are_integers() {
        let result = solve_race(30, 200);

        assert!(result.is_some());
        assert_eq!((11, 19), result.unwrap());
    }

    #[test]
    fn should_not_solve_race_with_no_solution() {
        let result = solve_race(5, 9);

        assert!(result.is_none());
    }

    #[test]
    fn should_count_num_solutions() {
        let solutions = num_solutions(7, 9);

        assert_eq!(solutions, 4);
    }

    #[test]
    fn should_not_double_count_degenerate_solution() {
        // Only solution is h = 3
        let solutions = num_solutions(6, 9);

        assert_eq!(solutions, 1);
    }
}
