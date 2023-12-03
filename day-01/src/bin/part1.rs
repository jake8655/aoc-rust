fn main() {
    let input = include_str!("./input");
    let ouput = solve(input);

    dbg!(ouput);
}

fn solve(input: &str) -> u32 {
    let mut sum: u32 = 0;

    for line in input.lines() {
        let mut first_digit = 0;
        let mut last_digit = 0;

        for character in line.chars() {
            if character.is_ascii_digit() {
                if first_digit == 0 {
                    first_digit = character.to_digit(10).unwrap();
                }
                last_digit = character.to_digit(10).unwrap();
            }
        }
        sum += first_digit * 10 + last_digit;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        let output = solve(input);

        assert_eq!(output, 142);
    }
}
