use std::{collections::HashMap, fs, ops::Add, str::Lines};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_name = args.get(1).unwrap();

    let file = fs::read_to_string(file_name).unwrap();
    let map = Map::parse(file.lines());
    print!("{}", map.display());
    println!("Start: {:?}", map.start);

    let mut distances: HashMap<Point, u32> = HashMap::new();
    distances.insert(map.start, 0);

    let start_tile = map.infer_start();
    let first_steps: Vec<(Point, u32)> = start_tile
        .connects_to()
        .iter()
        // Will break if start tile is on edge of grid
        .map(|delta| (map.start + *delta, 1))
        .collect();
    for (point, distance) in first_steps.iter() {
        distances.insert(*point, *distance);
    }

    calc_distances(&mut distances, &map, first_steps[0].0);
    calc_distances(&mut distances, &map, first_steps[1].0);

    let furthest = distances.iter().max_by_key(|pair| *pair.1).unwrap();

    println!("Furthest point: {:?}", furthest);
}

fn calc_distances(distances: &mut HashMap<Point, u32>, map: &Map, first_step: Point) {
    let mut to_visit: Vec<(Point, u32)> = vec![(first_step, 1)];
    let mut visited: Vec<Point> = vec![];

    while let Some((current_position, current_distance)) = to_visit.pop() {
        visited.push(current_position);
        let tile = map.get(&current_position);
        let connects_to: Vec<(Point, u32)> = tile
            .connects_to()
            .iter()
            .map(|delta| current_position + *delta)
            .map(|new_position| (new_position, current_distance + 1))
            .collect();
        let mut not_yet_visited: Vec<(Point, u32)> = connects_to
            .iter()
            .filter(|(point, _)| !visited.contains(point))
            .map(|pair| *pair)
            .collect();
        for (position, distance) in connects_to.iter() {
            let new_distance = match distances.get(position) {
                Some(old_distance) => {
                    if distance < old_distance {
                        distance
                    } else {
                        old_distance
                    }
                }
                None => distance,
            };
            distances.insert(*position, *new_distance);
        }
        to_visit.append(&mut not_yet_visited);
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Point {
    const UP: Point = Point { x: 0, y: 1 };
    const DOWN: Point = Point { x: 0, y: -1 };
    const LEFT: Point = Point { x: -1, y: 0 };
    const RIGHT: Point = Point { x: 1, y: 0 };

    fn x(&self) -> usize {
        self.x.try_into().unwrap()
    }

    fn y(&self) -> usize {
        self.y.try_into().unwrap()
    }
}

struct Map {
    tiles: Vec<Vec<Tile>>,
    start: Point,
}

impl Map {
    fn parse(lines: Lines) -> Map {
        let tiles: Vec<Vec<Tile>> = lines
            .map(|line| line.chars().map(|input| Tile::parse(input)).collect())
            .collect();
        let start = Map::find_start(&tiles);

        Map {
            tiles,
            start,
        }
    }

    fn find_start(tiles: &Vec<Vec<Tile>>) -> Point {
        for (y, row) in tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if *tile == Tile::Start {
                    return Point {
                        x: x.try_into().unwrap(),
                        y: y.try_into().unwrap(),
                    };
                }
            }
        }
        panic!("No start found");
    }

    fn get(&self, point: &Point) -> Tile {
        self.tiles[point.y()][point.x()]
    }

    fn infer_start(&self) -> Tile {
        let left = self.start + Point::LEFT;
        let connects_left = self.get(&left).connects_to().contains(&Point::RIGHT);

        let right = self.start + Point::RIGHT;
        let connects_right = self.get(&right);
        let connects_right = connects_right.connects_to();
        let connects_right = connects_right.contains(&Point::LEFT);

        let above = self.start + Point::UP;
        let connects_up = self.get(&above).connects_to().contains(&Point::DOWN);

        let below = self.start + Point::DOWN;
        let connects_down = self.get(&below).connects_to().contains(&Point::UP);

        if connects_left {
            if connects_right {
                return Tile::Directional(Direction::East, Direction::West);
            }
            if connects_up {
                return Tile::Directional(Direction::South, Direction::West);
            }
            if connects_down {
                return Tile::Directional(Direction::North, Direction::West);
            }
        }

        if connects_right {
            if connects_up {
                return Tile::Directional(Direction::South, Direction::East);
            }
            if connects_down {
                return Tile::Directional(Direction::North, Direction::East);
            }
        }

        if connects_up && connects_down {
            return Tile::Directional(Direction::North, Direction::South);
        }

        panic!("Unable to infer start");
    }

    fn display(&self) -> String {
        self.tiles
            .iter()
            .map(|row| row.iter().map(|tile| tile.display()).collect())
            .map(|row: String| row + "\n")
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Start,
    Ground,
    Directional(Direction, Direction),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Tile {
    fn parse(input: char) -> Tile {
        match input {
            '|' => Tile::Directional(Direction::North, Direction::South),
            '-' => Tile::Directional(Direction::East, Direction::West),
            'L' => Tile::Directional(Direction::North, Direction::East),
            'J' => Tile::Directional(Direction::North, Direction::West),
            '7' => Tile::Directional(Direction::South, Direction::West),
            'F' => Tile::Directional(Direction::South, Direction::East),
            '.' => Tile::Ground,
            'S' => Tile::Start,
            _ => panic!("Unkown pipe"),
        }
    }

    fn display(&self) -> char {
        match self {
            Tile::Start => 'S',
            Tile::Ground => '.',
            Tile::Directional(from, to) => match (from, to) {
                (Direction::North, Direction::South) => '|',
                (Direction::East, Direction::West) => '─',
                (Direction::North, Direction::East) => '└',
                (Direction::North, Direction::West) => '┘',
                (Direction::South, Direction::West) => '┐',
                (Direction::South, Direction::East) => '┌',
                _ => panic!("Invalid direction combination"),
            },
        }
    }

    fn connects_to(&self) -> Vec<Point> {
        match self {
            Tile::Start => vec![],
            Tile::Ground => vec![],
            Tile::Directional(from, to) => match (from, to) {
                (Direction::North, Direction::South) => vec![Point::UP, Point::DOWN],
                (Direction::East, Direction::West) => vec![Point::LEFT, Point::RIGHT],
                (Direction::North, Direction::East) => vec![Point::DOWN, Point::RIGHT],
                (Direction::North, Direction::West) => vec![Point::DOWN, Point::LEFT],
                (Direction::South, Direction::West) => vec![Point::UP, Point::LEFT],
                (Direction::South, Direction::East) => vec![Point::UP, Point::RIGHT],
                _ => panic!("Invalid direction combination"),
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tile_should_parse_vertical() {
        let input = '|';

        let tile = Tile::parse(input);

        let expected = Tile::Directional(Direction::North, Direction::South);
        assert_eq!(expected, tile);
    }

    #[test]
    fn tile_should_parse_horizontal() {
        let input = '-';

        let tile = Tile::parse(input);

        let expected = Tile::Directional(Direction::East, Direction::West);
        assert_eq!(expected, tile);
    }

    #[test]
    fn tile_should_parse_l() {
        let input = 'L';

        let tile = Tile::parse(input);

        let expected = Tile::Directional(Direction::North, Direction::East);
        assert_eq!(expected, tile);
    }

    #[test]
    fn tile_should_parse_j() {
        let input = 'J';

        let tile = Tile::parse(input);

        let expected = Tile::Directional(Direction::North, Direction::West);
        assert_eq!(expected, tile);
    }

    #[test]
    fn tile_should_parse_7() {
        let input = '7';

        let tile = Tile::parse(input);

        let expected = Tile::Directional(Direction::South, Direction::West);
        assert_eq!(expected, tile);
    }

    #[test]
    fn tile_should_parse_f() {
        let input = 'F';

        let tile = Tile::parse(input);

        let expected = Tile::Directional(Direction::South, Direction::East);
        assert_eq!(expected, tile);
    }

    #[test]
    fn tile_should_parse_ground() {
        let input = '.';

        let tile = Tile::parse(input);

        let expected = Tile::Ground;
        assert_eq!(expected, tile);
    }
}
