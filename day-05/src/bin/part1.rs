const NUMBER_OF_MAPS: usize = 7;

fn main() {
    // let input = include_str!("./input");
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
    let ouput = solve(input);

    dbg!(ouput);
}

#[derive(Debug)]
struct MapList {
    maps: Vec<Map>,
}

impl MapList {
    fn new() -> Self {
        Self { maps: Vec::new() }
    }

    fn add_map(&mut self, map: Map) {
        self.maps.push(map);
    }

    fn convert_one(&self, value: u32) -> u32 {
        let mut result = value as i32;

        for map in &self.maps {
            if value == 14 && map.i == 3 {
                println!(
                    "{} >= {} && {} < {} + {} -> {}",
                    result,
                    map.source_range_start,
                    result,
                    map.source_range_start,
                    map.range_length,
                    result as u32 >= map.source_range_start
                        && (result as u32) < map.source_range_start + map.range_length,
                );
                // println!("result {result} i {} {:?}", map.i, map);
            }

            if (result as u32 >= map.source_range_start)
                && ((result as u32) < (map.source_range_start + map.range_length))
            {
                if value == 14 && map.i == 3 {
                    println!(
                        "{} += {} - {} -> {}",
                        result,
                        map.destination_range_start,
                        map.source_range_start,
                        result
                            + (map.destination_range_start as i32 - map.source_range_start as i32),
                    );
                }

                result += map.destination_range_start as i32 - map.source_range_start as i32;
            }
        }

        result as u32
    }

    fn convert_set(&self, source: Vec<u32>) -> Vec<u32> {
        let mut result = Vec::new();

        for value in source {
            result.push(self.convert_one(value));
        }

        result
    }
}

#[derive(Debug)]
struct Map {
    destination_range_start: u32,
    source_range_start: u32,
    range_length: u32,
    i: usize,
}

impl Map {
    fn new(
        destination_range_start: u32,
        source_range_start: u32,
        range_length: u32,
        i: usize,
    ) -> Self {
        Self {
            destination_range_start,
            source_range_start,
            range_length,
            i,
        }
    }
}

fn solve(input: &str) -> u32 {
    let mut lines = input.lines();

    let seeds = lines
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let mut map_list = MapList::new();
    let _ = lines.by_ref().take(1).collect::<Vec<_>>();

    for i in 0..NUMBER_OF_MAPS {
        let _ = lines.by_ref().take(1).collect::<Vec<_>>();

        for line in lines.by_ref().take_while(|l| !l.is_empty()) {
            let mut parts = line.split_whitespace();

            let destination_range_start = parts.next().unwrap().parse::<u32>().unwrap();
            let source_range_start = parts.next().unwrap().parse::<u32>().unwrap();
            let range_length = parts.next().unwrap().parse::<u32>().unwrap();

            let map = Map::new(
                destination_range_start,
                source_range_start,
                range_length,
                i + 1,
            );
            map_list.add_map(map);
        }
    }

    let result = map_list.convert_set(seeds);

    println!("result: {:?}", &result);

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
