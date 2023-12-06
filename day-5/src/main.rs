use std::{collections::HashMap, fs};

#[derive(Debug, PartialEq, Eq)]
struct Almanac {
    resource_maps: HashMap<String, ResourceMap>
}

impl Almanac {
    fn parse(rows: &mut dyn Iterator<Item = &str>) -> Almanac {
        let mut resource_maps: HashMap<String, ResourceMap> = HashMap::new();
        while let Some(header) = rows.next() { 
            let mut resource_map_rows = rows.take_while(|row| !row.is_empty());
            let resource_map = ResourceMap::parse(header, &mut resource_map_rows);
            resource_maps.insert(resource_map.from.to_string(), resource_map);
        }
        Almanac { resource_maps }
    }

    fn find_location(&self, resource_type: &String, source_value: u64) -> u64 {
        let map = self.resource_maps.get(resource_type).unwrap();
        let destination_value = map.get(source_value);
        let destination_resource_type = &map.to;
        if destination_resource_type == "location" {
            destination_value
        } else {
            self.find_location(destination_resource_type, destination_value)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Mapping {
    source_from: u64,
    dest_from: u64,
    length: u64,
}

impl Mapping {
    fn get(&self, source: u64) -> Option<u64> {
        if source > self.source_from && source < (self.source_from + self.length) {
            let steps = source.abs_diff(self.source_from);
            Some(self.dest_from + steps)
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ResourceMap {
    from: String,
    to: String,
    mappings: Vec<Mapping>,
}

impl ResourceMap {
    fn parse(header:&str, rows: &mut dyn Iterator<Item = &str>) -> ResourceMap {
        let parts: Vec<&str> = header.split("-to-").collect();
        let from = parts.first().unwrap().to_string();
        let to = parts
            .last()
            .unwrap()
            .split(' ')
            .collect::<Vec<&str>>()
            .first()
            .unwrap()
            .to_string();

        let mut mappings: Vec<Mapping> = vec![];
        for row in rows {
            let parts: Vec<u64> = row.split(' ').map(|num| num.parse().unwrap()).collect();
            let dest_start = parts[0];
            let source_start = parts[1];
            let length = parts[2];
            let mapping = Mapping{
                source_from: source_start,
                dest_from: dest_start,
                length,
            };
            mappings.push(mapping);
        }
        ResourceMap { from, to, mappings }
    }

    fn get(&self, source: u64) -> u64 {
        self.mappings.iter()
            .find(|mapping| mapping.get(source).is_some())
            .map(|mapping| mapping.get(source).unwrap())
            .unwrap_or(source)
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_name = args.get(1).unwrap();

    let file = fs::read_to_string(file_name).unwrap();
    let mut lines = file.lines();
    let seeds = lines.next().unwrap();
    lines.next(); // Skip blank line
    let almanac = Almanac::parse(&mut lines);

    let lowest_location = seeds[7..].split(' ')
        .map(|num| num.parse().unwrap())
        .map(|seed_value| almanac.find_location(&"seed".to_string(), seed_value))
        .min()
        .unwrap();
    println!("Part 1: {}", lowest_location);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_range() {
        let header = "seed-to-soil map:";
        let lines: Vec<String> = vec!["50 98 2".into(), "52 50 3".into()];

        let map = ResourceMap::parse(header, &mut lines.iter().map(|s| s.as_str()));

        let expected_map = ResourceMap {
            from: "seed".into(),
            to: "soil".into(),
            mappings: vec![
                Mapping{source_from: 98, dest_from: 50, length: 2},
                Mapping{source_from: 50, dest_from: 52, length: 3},
            ],
        };
        assert_eq!(expected_map, map);
    }

    #[test]
    fn should_parse_almanac() {
        let lines: Vec<String> = vec![
            "seed-to-soil map:".into(),
            "50 98 2".into(),
            "52 50 3".into(),
            "".into(),
            "soil-to-fertilizer map:".into(),
            "0 15 1".into(),
            "37 52 2".into(),
        ];

        let almanac = Almanac::parse(&mut lines.iter().map(|s| s.as_str()));

        let expected_almanac = Almanac{
            resource_maps: HashMap::from([
                ("seed".into(), ResourceMap {
                    from: "seed".into(),
                    to: "soil".into(),
                    mappings: vec![
                        Mapping{source_from: 98, dest_from: 50, length: 2},
                        Mapping{source_from: 50, dest_from: 52, length: 3},
                    ],
                }),
                ("soil".into(), ResourceMap{
                    from: "soil".into(),
                    to: "fertilizer".into(),
                    mappings: vec![
                        Mapping{source_from: 15, dest_from: 0, length: 1},
                        Mapping{source_from: 52, dest_from: 37, length: 2},
                    ],
                })
            ]),
        };
        assert_eq!(expected_almanac, almanac);
    }
}
