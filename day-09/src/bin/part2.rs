fn main() {
    let input = include_str!("./input");
    //     let input = "0 3 6 9 12 15
    // 1 3 6 10 15 21
    // 10 13 16 21 30 45";
    let ouput = solve(input);

    dbg!(ouput);
}

#[derive(Debug)]
struct History(Vec<Vec<i64>>);

impl History {
    fn from_str(line: &str) -> Self {
        Self(vec![line
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect::<Vec<_>>()])
    }

    fn generate_next(&mut self) {
        let last_sequence = self.0.last().unwrap();
        let mut new_sequence = Vec::new();

        for i in 1..last_sequence.len() {
            let diff = last_sequence[i] - last_sequence[i - 1];
            new_sequence.push(diff);
        }

        self.0.push(new_sequence);
    }

    fn ends_with_zeros(&self) -> bool {
        let last_sequence = self.0.last().unwrap();

        last_sequence.iter().all(|&n| n == 0)
    }

    fn predict(&mut self) {
        // The last sequence is all zeros
        let last_sequence = self.0.last_mut().unwrap();
        // Add a zero to the end of the sequence
        last_sequence.push(0);

        // self.0.len() - 1 because we the last sequence is all zeros
        for i in (0..self.0.len() - 1).rev() {
            let first = self.0[i].first().unwrap();
            let new = first - self.0[i + 1].last().unwrap();

            self.0[i].push(new);
        }
    }
}

#[derive(Debug)]
struct Dataset(Box<[History]>);

impl Dataset {
    fn from_str(input: &str) -> Self {
        let history = input.lines().map(History::from_str).collect::<Vec<_>>();

        Self(history.into_boxed_slice())
    }
}

fn solve(input: &str) -> i64 {
    let mut dataset = Dataset::from_str(input);

    dataset.0.iter_mut().for_each(|history| {
        while !history.ends_with_zeros() {
            history.generate_next();
        }

        history.predict();
    });

    dataset
        .0
        .iter()
        .map(|history| *history.0.first().unwrap().last().unwrap())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        let output = solve(input);

        assert_eq!(output, 2);
    }
}
