use std::{fmt::Display, path::PathBuf, str::Lines, time::Instant};

/// Read the file at `./input/<filename>` into a string and then leak the memory
pub fn load_input(filename: &str) -> std::io::Result<&'static str> {
    let path: PathBuf = ["./input", filename].iter().collect();
    Ok(std::fs::read_to_string(path)?.leak())
}

pub struct Challenge {
    day: usize,
    part: usize,
    start: Instant,
    input: &'static str,
}

impl Challenge {
    /// Start the challenge by loading the input and recording the current time.
    pub fn start(day: usize, part: usize) -> Challenge {
        let input = load_input(&format!("day-{:02}-{:02}.txt", day, part))
            .or_else(|_| load_input(&format!("day-{:02}.txt", day)))
            .unwrap();

        let start = Instant::now();

        Challenge {
            day,
            part,
            start,
            input,
        }
    }

    /// Finish the callenge, displaying the solution and some metadata.
    pub fn finish<T: Display>(self, solution: T) {
        let elapsed_ms = self.elapsed_ms();
        println!(
            "[Day-{:02} | Part-{:02} | {:>6.3}ms] Solution: {}",
            self.day, self.part, elapsed_ms, solution
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
