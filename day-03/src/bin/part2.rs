fn main() {
    let input = include_str!("./input");
    //     let input = "467..114..
    // ...*......
    // ..35..633.
    // ......#...
    // 617*......
    // .....+.58.
    // ..592.....
    // ......755.
    // ...$.*....
    // .664.598..";
    let ouput = solve(input);

    dbg!(ouput);
}

fn solve(input: &str) -> u32 {
    let mut matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let result: u32 = (0..matrix.len())
        .map(|y| {
            (0..matrix[y].len())
                .map(|x| {
                    if matrix[y][x] != '*' {
                        return 0;
                    }

                    let mut numbers = Vec::new();

                    // Check if the character to the top, bottom, left, right, diagonals exists and is a number
                    // If it is, walk the number to the beginning read it from there
                    if y != 0 && matrix[y - 1][x].is_ascii_digit() {
                        numbers.push(get_number_from_middle(&mut matrix, x, y - 1));
                    }
                    if y != matrix.len() - 1 && matrix[y + 1][x].is_ascii_digit() {
                        numbers.push(get_number_from_middle(&mut matrix, x, y + 1));
                    }
                    if x != 0 && matrix[y][x - 1].is_ascii_digit() {
                        numbers.push(get_number_from_middle(&mut matrix, x - 1, y));
                    };
                    if x != matrix[y].len() - 1 && matrix[y][x + 1].is_ascii_digit() {
                        numbers.push(get_number_from_middle(&mut matrix, x + 1, y));
                    };

                    if y != 0 && x != 0 && matrix[y - 1][x - 1].is_ascii_digit() {
                        numbers.push(get_number_from_middle(&mut matrix, x - 1, y - 1));
                    };
                    if y != 0 && x != matrix[y].len() - 1 && matrix[y - 1][x + 1].is_ascii_digit() {
                        numbers.push(get_number_from_middle(&mut matrix, x + 1, y - 1));
                    };
                    if y != matrix.len() - 1 && x != 0 && matrix[y + 1][x - 1].is_ascii_digit() {
                        numbers.push(get_number_from_middle(&mut matrix, x - 1, y + 1));
                    };
                    if y != matrix.len() - 1
                        && x != matrix[y].len() - 1
                        && matrix[y + 1][x + 1].is_ascii_digit()
                    {
                        numbers.push(get_number_from_middle(&mut matrix, x + 1, y + 1));
                    };

                    if numbers.len() == 2 {
                        numbers.iter().product::<u32>()
                    } else {
                        0
                    }
                })
                .sum::<u32>()
        })
        .sum::<u32>();

    result
}

fn get_number_from_middle(matrix: &mut [Vec<char>], mut x: usize, y: usize) -> u32 {
    let mut str_number = String::new();

    while x >= 1 && matrix[y][x - 1].is_ascii_digit() {
        x -= 1;
        if x == 0 {
            break;
        }
    }

    while x < matrix[y].len() && matrix[y][x].is_ascii_digit() {
        str_number.push(matrix[y][x]);
        matrix[y][x] = '.';
        x += 1;
    }

    str_number.parse::<u32>().unwrap()
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

        assert_eq!(output, 467835);
    }
}
