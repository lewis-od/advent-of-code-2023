use std::{fs, io::Error};

#[derive(PartialEq, Eq, Debug)]
struct Game {
    number: u32,
    rounds: Vec<Balls>,
}

impl Game {
    fn parse(text: &str) -> Game {
        let game_and_rounds: Vec<&str> = text.split(": ").collect();
        let game_number = game_and_rounds
            .first()
            .unwrap()
            .replace("Game ", "")
            .parse()
            .unwrap();
        let rounds = game_and_rounds
            .last()
            .unwrap()
            .split(';')
            .map(|round_text| round_text.trim())
            .map(|round_text| Balls::parse(round_text))
            .collect();

        Game {
            number: game_number,
            rounds: rounds,
        }
    }

    fn is_possible(&self, bag_contents: &Balls) -> bool {
        self.rounds
            .iter()
            .all(|round| round.is_round_possible(bag_contents))
    }

    fn min_bag_contents(&self) -> Balls {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for round in self.rounds.iter() {
            max_red = std::cmp::max(max_red, round.red);
            max_green = std::cmp::max(max_green, round.green);
            max_blue = std::cmp::max(max_blue, round.blue);
        }
        Balls::new(max_red, max_green, max_blue)
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Balls {
    red: u32,
    green: u32,
    blue: u32,
}

impl Balls {
    fn new(red: u32, green: u32, blue: u32) -> Balls {
        Balls {
            red: red,
            green: green,
            blue: blue,
        }
    }

    fn parse(text: &str) -> Balls {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        let colours = text.split(", ");
        for number_and_colour in colours {
            let parts: Vec<&str> = number_and_colour.split(' ').collect();
            let number = parts.first().map(|n| n.parse::<u32>().unwrap()).unwrap();
            let colour = *parts.last().unwrap();
            match colour {
                "red" => red = number,
                "green" => green = number,
                "blue" => blue = number,
                &_ => panic!("Unknown colour"),
            }
        }

        Balls::new(red, green, blue)
    }

    fn is_round_possible(&self, bag_contents: &Balls) -> bool {
        return self.red <= bag_contents.red
            && self.green <= bag_contents.green
            && self.blue <= bag_contents.blue;
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().collect();
    let file_name = args.get(1).unwrap();

    let bag_contents = Balls::new(12, 13, 14);

    let games: Vec<Game> = fs::read_to_string(file_name)?
        .lines()
        .map(|line| Game::parse(line))
        .collect();

    let answer1: u32 = games
        .iter()
        .filter(|game| game.is_possible(&bag_contents))
        .map(|game| game.number)
        .sum();
    println!("Part 1: {}", answer1);

    let answer2: u32 = games
        .iter()
        .map(Game::min_bag_contents)
        .map(|contents| contents.power())
        .sum();
    println!("Part 2: {}", answer2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_round_with_all_colours() {
        let text = "1 blue, 2 green, 6 red";

        let round = Balls::parse(text);

        let expected_round = Balls {
            red: 6,
            green: 2,
            blue: 1,
        };
        assert_eq!(expected_round, round);
    }

    #[test]
    fn should_parse_round_with_some_colours() {
        let text = "12 blue, 69 red";

        let round = Balls::parse(text);

        let expected_round = Balls {
            red: 69,
            green: 0,
            blue: 12,
        };
        assert_eq!(expected_round, round);
    }

    #[test]
    fn should_parse_game() {
        let text = "Game 12: 7 green, 2 red; 9 blue, 1 red, 21 green";

        let game = Game::parse(text);

        let expected_game = Game {
            number: 12,
            rounds: vec![Balls::new(2, 7, 0), Balls::new(1, 21, 9)],
        };
        assert_eq!(expected_game, game);
    }

    #[test]
    fn should_calculate_bag_contents() {
        let game = Game {
            number: 1,
            rounds: vec![
                Balls::new(20, 8, 6),
                Balls::new(4, 13, 5),
                Balls::new(1, 0, 5),
            ],
        };

        let contents = game.min_bag_contents();

        let expected_contents = Balls::new(20, 13, 6);
        assert_eq!(expected_contents, contents);
    }
}
