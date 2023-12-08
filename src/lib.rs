use std::{
    fmt::{Debug, Display},
    path::PathBuf,
    str::{FromStr, Lines},
    time::{Duration, Instant},
};

/// Read the file at `./input/<filename>` into a string and then leak the memory
pub fn load_input(filename: &str) -> std::io::Result<&'static str> {
    let path: PathBuf = ["./input", filename].iter().collect();
    Ok(std::fs::read_to_string(path)?.leak())
}

#[derive(Debug, Default)]
pub struct Solution {
    part_1: Option<&'static str>,
    part_2: Option<&'static str>,
}

impl Solution {
    pub fn check<T>(&self, part: usize, solution: &T) -> char
    where
        T: FromStr + Eq,
        T::Err: Debug,
    {
        let real_solution: Option<T> = match part {
            1 => self.part_1,
            2 => self.part_2,
            n => panic!("no solution for part {}", n),
        }
        .map(|opt| opt.parse().unwrap());

        match (real_solution, solution) {
            (Some(s1), s2) if &s1 == s2 => '✅',
            (Some(_), _) => '❌',
            (None, _) => '❔',
        }
    }
}

pub struct Challenge {
    day: usize,
    part: usize,
    start: Instant,
    parsing: Option<Duration>,
    input: &'static str,
    solution: Solution,
}

impl Challenge {
    /// Start the challenge by loading the input and recording the current time.
    pub fn start(day: usize, part: usize) -> Challenge {
        assert!((1..=24).contains(&day), "day {} is out of range", day);
        assert!((1..=2).contains(&part), "part {} is out of range", part);

        // load the puzzle input
        let input = load_input(&format!("day-{:02}-{:02}.txt", day, part))
            .or_else(|_| load_input(&format!("day-{:02}.txt", day)))
            .unwrap();

        // load the corresponding solutions
        let solution = load_input("solutions.txt")
            .map(|text| {
                let Some(line) = text.lines().nth(day - 1) else {
                    return Solution::default();
                };

                match line.split_once(' ') {
                    Some((part_1, part_2)) => Solution {
                        part_1: Some(part_1),
                        part_2: Some(part_2),
                    },
                    None => Solution {
                        part_1: Some(line),
                        part_2: None,
                    },
                }
            })
            .unwrap_or_default();

        let start = Instant::now();

        Challenge {
            day,
            part,
            start,
            parsing: None,
            input,
            solution,
        }
    }

    pub fn finish_parsing(&mut self) {
        self.parsing = Some(self.start.elapsed());
    }

    /// Finish the callenge, displaying the solution and some metadata.
    pub fn finish<T>(self, solution: T)
    where
        T: Display + Eq + FromStr,
        T::Err: Debug,
    {
        let elapsed_ms = self.elapsed_ms();
        let parsing = self
            .parsing
            .map(|p| format!("{:>6.3}ms", p.as_secs_f64() * 1e3))
            .unwrap_or("        ".to_string());

        println!(
            "[Day-{:02} | Part-{:02} | {} | {:>6.3}ms] Solution: {} ({})",
            self.day,
            self.part,
            parsing,
            elapsed_ms,
            solution,
            self.solution.check(self.part, &solution)
        );
    }

    /// Get the challenge input.
    pub fn input(&self) -> &'static str {
        self.input
    }

    /// Get the challenge input as lines.
    pub fn input_lines(&self) -> Lines<'static> {
        self.input.lines()
    }

    /// Get the time elapsed since starting the challenge.
    pub fn elapsed_ms(&self) -> f64 {
        self.start.elapsed().as_secs_f64() * 1e3
    }
}
