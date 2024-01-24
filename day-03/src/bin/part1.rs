fn main() {
    // let input = include_str!("./input");
    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    let ouput = solve(input);

    dbg!(ouput);
}

fn solve(input: &str) -> u32 {
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let _ = matrix
        .iter()
        .enumerate()
        .map(|(row, line)| {
            line.iter().enumerate().for_each(|(col, character)| {
                if character == &'.' || character.is_ascii_digit() {
                    return;
                }

                println!("Found symbol {character} at col: {col} row: {row}");

                // Check top bottom left right and diagonal
                let top = if row == 0 {
                    None
                } else {
                    Some(matrix[row - 1][col])
                };
                let bottom = if row == matrix.len() - 1 {
                    None
                } else {
                    Some(matrix[row + 1][col])
                };
                let left = if col == 0 {
                    None
                } else {
                    Some(matrix[row][col - 1])
                };
                let right = if col == matrix[row].len() - 1 {
                    None
                } else {
                    Some(matrix[row][col + 1])
                };

                // println!("right: {:?}", right);
            })
        })
        .collect::<Vec<_>>();
    12
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let output = solve(input);

        assert_eq!(output, 4361);
    }
}
