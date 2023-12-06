#![allow(dead_code)]

#[derive(Debug, Default)]
struct Cubes {
    red: usize,
    green: usize,
    blue: usize,
}

impl Cubes {
    pub fn pow(&self) -> usize {
        self.red * self.green * self.blue
    }
}

#[derive(Debug)]
struct Game {
    id: usize,
    rounds: Vec<Cubes>,
}

impl Game {
    pub fn min_cube_set(&self) -> Cubes {
        let mut buf = Cubes::default();
        self.rounds.iter().for_each(|round| {
            buf.red = buf.red.max(round.red);
            buf.green = buf.green.max(round.green);
            buf.blue = buf.blue.max(round.blue);
        });
        buf
    }
}

fn main() {
    let challenge = advent_of_code_2023::Challenge::start(2, 2);

    let games = challenge
        .input_lines()
        .map(|line| {
            // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            let (_, line) = line.split_once("Game ").unwrap();
            let (id, line) = line.split_once(": ").unwrap();
            let id = id.parse::<usize>().unwrap();

            let rounds = line
                .split("; ")
                .map(|game| {
                    let mut cubes = Cubes::default();
                    game.split(", ").for_each(|draw| {
                        let (number, color) = draw.split_once(' ').unwrap();
                        let number = number.parse::<usize>().unwrap();
                        match color {
                            "red" => cubes.red = number,
                            "green" => cubes.green = number,
                            "blue" => cubes.blue = number,
                            _ => panic!("encountered unknown color: {}", color),
                        };
                    });
                    cubes
                })
                .collect::<Vec<_>>();

            Game { id, rounds }
        })
        .collect::<Vec<_>>();

    let solution = games
        .iter()
        .fold(0usize, |acc, game| acc + game.min_cube_set().pow());

    challenge.finish(solution);
}
