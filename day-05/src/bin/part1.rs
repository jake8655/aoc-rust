const NUMBER_OF_MAPS: usize = 7;

fn main() {
    let input = include_str!("./input");
    //     let input = "seeds: 79 14 55 13
    //
    // seed-to-soil map:
    // 50 98 2
    // 52 50 48
    //
    // soil-to-fertilizer map:
    // 0 15 37
    // 37 52 2
    // 39 0 15
    //
    // fertilizer-to-water map:
    // 49 53 8
    // 0 11 42
    // 42 0 7
    // 57 7 4
    //
    // water-to-light map:
    // 88 18 7
    // 18 25 70
    //
    // light-to-temperature map:
    // 45 77 23
    // 81 45 19
    // 68 64 13
    //
    // temperature-to-humidity map:
    // 0 69 1
    // 1 0 69
    //
    // humidity-to-location map:
    // 60 56 37
    // 56 93 4";
    let ouput = solve(input);

    dbg!(ouput);
}

#[derive(Debug)]
struct MapList {
    groups: Vec<MapGroup>,
}

#[derive(Debug)]
struct MapGroup {
    maps: Vec<Map>,
}

impl MapGroup {
    fn new() -> Self {
        Self { maps: Vec::new() }
    }

    fn add_map(&mut self, map: Map) {
        self.maps.push(map);
    }
}

impl MapList {
    fn new() -> Self {
        Self { groups: Vec::new() }
    }

    fn add_group(&mut self, group: MapGroup) {
        self.groups.push(group);
    }

    fn convert_one(&self, value: u64) -> u64 {
        let mut result = value as i64;

        for group in &self.groups {
            for map in &group.maps {
                if (result as u64 >= map.source_range_start)
                    && ((result as u64) < (map.source_range_start + map.range_length))
                {
                    result += map.destination_range_start as i64 - map.source_range_start as i64;
                    break;
                }
            }
        }

        result as u64
    }

    fn convert_set(&self, source: Vec<u64>) -> Vec<u64> {
        let mut result = Vec::new();

        for value in source {
            result.push(self.convert_one(value));
        }

        result
    }
}

#[derive(Debug)]
struct Map {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

impl Map {
    fn new(destination_range_start: u64, source_range_start: u64, range_length: u64) -> Self {
        Self {
            destination_range_start,
            source_range_start,
            range_length,
        }
    }
}

fn solve(input: &str) -> u64 {
    let mut lines = input.lines();

    let seeds = lines
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut map_list = MapList::new();
    let _ = lines.by_ref().take(1).collect::<Vec<_>>();

    for _ in 0..NUMBER_OF_MAPS {
        let _ = lines.by_ref().take(1).collect::<Vec<_>>();
        let mut group = MapGroup::new();

        for line in lines.by_ref().take_while(|l| !l.is_empty()) {
            let mut parts = line.split_whitespace();

            let destination_range_start = parts.next().unwrap().parse::<u64>().unwrap();
            let source_range_start = parts.next().unwrap().parse::<u64>().unwrap();
            let range_length = parts.next().unwrap().parse::<u64>().unwrap();

            let map = Map::new(destination_range_start, source_range_start, range_length);
            group.add_map(map);
        }
        map_list.add_group(group);
    }

    let result = map_list.convert_set(seeds);

    result.into_iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        let output = solve(input);

        assert_eq!(output, 35);
    }
}
