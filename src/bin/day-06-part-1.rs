#[derive(Debug)]
struct Race {
    time: u64,
    record: u64,
}

impl Race {
    fn possible_solves(&self) -> usize {
        (1..self.time)
            .map(|button_time| (self.time - button_time) * button_time)
            .filter(|&dist| dist > self.record)
            .count()
    }
}

fn main() {
    let challenge = advent_of_code_2023::Challenge::start(6, 1);

    let races = {
        let mut lines = challenge.input_lines();

        let times = lines
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .map(|num| num.parse::<u64>().unwrap());

        let records = lines
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .map(|num| num.parse::<u64>().unwrap());

        times
            .zip(records)
            .map(|(time, record)| Race { time, record })
            .collect::<Vec<_>>()
    };

    let solution = races
        .iter()
        .map(|race| race.possible_solves())
        .reduce(|acc, next| acc * next)
        .unwrap();

    challenge.finish(solution);
}
