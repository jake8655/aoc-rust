fn main() {
    let input = include_str!("./input");
    let ouput = solve(input);

    dbg!(ouput);
}

struct Game {
    id: u32,
    max_red: u32,
    max_green: u32,
    max_blue: u32,
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
    let mut games = Vec::<Game>::new();

    input.lines().for_each(|l| {
        let mut parts = l.split(": ");
        let game_part = parts.next().unwrap();
        let cube_part = parts.next().unwrap();

        let game_id = game_part.split(' ').nth(1).unwrap().parse::<u32>().unwrap();

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

        games.push(Game {
            id: game_id,
            max_red: max_cubes.red,
            max_green: max_cubes.green,
            max_blue: max_cubes.blue,
        });
    });

    let possible_games: Vec<Game> = games
        .into_iter()
        .filter(|game| game.max_red <= 12 && game.max_green <= 13 && game.max_blue <= 14)
        .collect();

    possible_games
        .into_iter()
        .fold(0, |acc, game| acc + game.id)
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

        assert_eq!(output, 8);
    }
}
