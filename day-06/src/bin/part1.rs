use std::iter::zip;

fn main() {
    let input = include_str!("./input");
    //     let input = "Time:      7  15   30
    // Distance:  9  40  200";
    let ouput = solve(input);

    dbg!(ouput);
}

#[derive(Debug)]
struct Race {
    time: u32,
    record_distance: u32,
}

impl Race {
    fn new(time: u32, distance: u32) -> Self {
        Self {
            time,
            record_distance: distance,
        }
    }

    fn number_of_solutions(&self) -> u32 {
        let mut solutions = 0;

        for i in 1..self.time {
            let time_to_run = self.time - i;
            let distance = time_to_run * i;

            if distance > self.record_distance {
                solutions += 1;
            }
        }

        solutions
    }
}

fn solve(input: &str) -> u32 {
    let mut lines = input.lines();

    let times = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u32>>();
    let distances = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u32>>();

    let races: Vec<Race> = zip(times, distances)
        .map(|(time, distance)| Race::new(time, distance))
        .collect();

    races.iter().map(|r| r.number_of_solutions()).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        let output = solve(input);

        assert_eq!(output, 288);
    }
}
