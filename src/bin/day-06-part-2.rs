#[derive(Debug)]
struct Race {
    time: u64,
    record: u64,
}

impl Race {
    fn zero_crossings(&self) -> [f64; 2] {
        let time = self.time as f64;
        let record = self.record as f64;

        let root = ((time.powi(2) / 4.0) - record).sqrt();
        let t_1 = (time / 2.0) + root;
        let t_2 = (time / 2.0) - root;

        [t_1.min(t_2), t_1.max(t_2)]
    }

    fn is_solution(&self, hold_time: u64) -> bool {
        (self.time - hold_time) * hold_time > self.record
    }

    fn possible_solves(&self) -> u64 {
        let [start, end] = self.zero_crossings();
        let mut start = start.ceil() as u64;
        let mut end = end.floor() as u64;

        if !self.is_solution(start) {
            start += 1;
        }
        if !self.is_solution(end) {
            end -= 1;
        }

        end - start + 1
    }
}

fn main() {
    let challenge = advent_of_code_2023::Challenge::start(6, 2);

    let race = {
        let mut lines = challenge.input_lines();
        let mut parse_buf = String::new();

        lines
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .for_each(|num| parse_buf.push_str(num));
        let time = parse_buf.parse::<u64>().unwrap();

        parse_buf.clear();
        lines
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .for_each(|num| parse_buf.push_str(num));
        let record = parse_buf.parse::<u64>().unwrap();

        Race { time, record }
    };

    let solution = race.possible_solves();

    challenge.finish(solution);
}
