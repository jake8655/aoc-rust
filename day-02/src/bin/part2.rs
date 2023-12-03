fn main() {
    let input = include_str!("./input");
    let ouput = solve(input);

    dbg!(ouput);
}

struct MaxCube {
    blue: u32,
    green: u32,
    red: u32,
}

impl MaxCube {
    pub fn get_amount_by_color(&self, key: &str) -> &u32 {
        match key {
            "red" => &self.red,
            "green" => &self.green,
            "blue" => &self.blue,
            _ => unreachable!(),
        }
    }

    pub fn set_amount_by_color(&mut self, key: &str, amount: u32) {
        match key {
            "red" => self.red = amount,
            "green" => self.green = amount,
            "blue" => self.blue = amount,
            _ => unreachable!(),
        };
    }
}

fn solve(input: &str) -> u32 {
    input.lines().fold(0, |acc, line| {
        let mut parts = line.split(": ");
        let cube_part = parts.nth(1).unwrap();

        let max_cubes = cube_part.split("; ").fold(
            MaxCube {
                red: 0,
                green: 0,
                blue: 0,
            },
            |mut acc, round| {
                let cube_group = round.split(", ");

                cube_group.for_each(|cube| {
                    let mut amount_and_color = cube.split(' ');
                    let amount = amount_and_color.next().unwrap().parse::<u32>().unwrap();
                    let color = amount_and_color.next().unwrap();

                    if *acc.get_amount_by_color(color) < amount {
                        acc.set_amount_by_color(color, amount);
                    }
                });
                acc
            },
        );

        acc + max_cubes.red * max_cubes.green * max_cubes.blue
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let output = solve(input);

        assert_eq!(output, 2286);
    }
}
