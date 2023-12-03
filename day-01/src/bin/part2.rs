fn main() {
    let input = include_str!("./input");
    let output = solve(input);

    dbg!(output);
}

use strum::{EnumIter, IntoEnumIterator};

#[repr(u32)]
#[derive(Debug, Clone, Copy, EnumIter)]
enum Digit {
    One = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}
impl Digit {
    fn as_spelling(&self) -> &'static str {
        match self {
            Digit::One => "one",
            Digit::Two => "two",
            Digit::Three => "three",
            Digit::Four => "four",
            Digit::Five => "five",
            Digit::Six => "six",
            Digit::Seven => "seven",
            Digit::Eight => "eight",
            Digit::Nine => "nine",
        }
    }

    fn as_char(&self) -> char {
        match self {
            Digit::One => '1',
            Digit::Two => '2',
            Digit::Three => '3',
            Digit::Four => '4',
            Digit::Five => '5',
            Digit::Six => '6',
            Digit::Seven => '7',
            Digit::Eight => '8',
            Digit::Nine => '9',
        }
    }

    pub fn find_first_and_last(input: &str) -> (Digit, Digit) {
        let mut first_index = usize::MAX;
        let mut last_index = usize::MIN;

        let mut first_digit = Digit::One;
        let mut last_digit = Digit::One;

        for digit in Self::iter() {
            let digit_spelling = digit.as_spelling();
            let digit_char = digit.as_char();

            let digit_first_index = {
                match (input.find(digit_spelling), input.find(digit_char)) {
                    (Some(s), Some(c)) => s.min(c),
                    (Some(s), None) => s,
                    (None, Some(c)) => c,
                    (None, None) => continue,
                }
            };

            let digit_last_index = {
                match (input.rfind(digit_spelling), input.rfind(digit_char)) {
                    (Some(s), Some(c)) => s.max(c),
                    (Some(s), None) => s,
                    (None, Some(c)) => c,
                    _ => unreachable!(),
                }
            };

            if digit_first_index <= first_index {
                first_index = digit_first_index;
                first_digit = digit;
            }
            if digit_last_index >= last_index {
                last_index = digit_last_index;
                last_digit = digit;
            }
        }

        (first_digit, last_digit)
    }
}

fn solve(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (first_digit, last_digit) = Digit::find_first_and_last(line);
            first_digit as u32 * 10 + last_digit as u32
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        let output = solve(input);
        assert_eq!(output, 281);
    }
}
