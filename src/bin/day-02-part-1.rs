#[derive(Debug, Default)]
struct Cubes {
    red: usize,
    green: usize,
    blue: usize,
}

const MAX_CUBES: Cubes = Cubes {
    red: 12,
    green: 13,
    blue: 14,
};

impl Cubes {
    pub fn is_possible(&self, limit: &Cubes) -> bool {
        self.red <= limit.red && self.green <= limit.green && self.blue <= limit.blue
    }
}

#[derive(Debug)]
struct Game {
    id: usize,
    rounds: Vec<Cubes>,
}

impl Game {
    pub fn is_possible(&self, limit: &Cubes) -> bool {
        self.rounds.iter().all(|round| round.is_possible(limit))
    }
}

fn main() {
    let input = advent_of_code_2023::load_input("day-02.txt");
    let start = std::time::Instant::now();

    let games = input
        .lines()
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

    let solution = games.iter().fold(0usize, |acc, game| {
        if game.is_possible(&MAX_CUBES) {
            acc + game.id
        } else {
            acc
        }
    });

    let elapsed = start.elapsed().as_secs_f64() * 1e3;
    println!("{} ({:.3}ms)", solution, elapsed);
}
