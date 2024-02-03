fn main() {
    let input = include_str!("./input");
    //     let input = "Time:      7  15   30
    // Distance:  9  40  200";
    let ouput = solve(input);

    dbg!(ouput);
}

#[derive(Debug)]
struct Race {
    time: u64,
    record_distance: u64,
}

impl Race {
    fn new(time: u64, distance: u64) -> Self {
        Self {
            time,
            record_distance: distance,
        }
    }

    fn number_of_solutions(&self) -> u64 {
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

fn solve(input: &str) -> u64 {
    let mut lines = input.lines();

    let time = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .fold(String::from(""), |acc, x| format!("{}{}", acc, x))
        .parse::<u64>()
        .unwrap();

    let distance = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .fold(String::from(""), |acc, x| format!("{}{}", acc, x))
        .parse::<u64>()
        .unwrap();

    let race = Race::new(time, distance);

    race.number_of_solutions()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        let output = solve(input);

        assert_eq!(output, 71503);
    }
}
