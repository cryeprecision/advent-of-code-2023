#![allow(dead_code)]

#[derive(Debug, Default)]
struct Number {
    line: usize,
    start: usize,
    end: usize,
    number: u64,
}

impl Number {
    fn has_adjacent_symbol(&self, symbols: &[Symbol]) -> bool {
        let min_pos = self.start.saturating_sub(1);
        let max_pos = self.end; // end is one past the last digit
        let min_line = self.line.saturating_sub(1);
        let max_line = self.line.saturating_add(1);

        // check if any symbol is adjacent to this number
        symbols.iter().any(|symbol| {
            symbol.pos >= min_pos
                && symbol.pos <= max_pos
                && symbol.line >= min_line
                && symbol.line <= max_line
        })
    }
}

#[derive(Debug, Default)]
struct Symbol {
    line: usize,
    pos: usize,
    character: char,
}

struct Line {
    line: String,
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

fn main() {
    let challenge = advent_of_code_2023::Challenge::start(3, 1);

    let mut numbers = Vec::new();
    let mut symbols = Vec::new();

    // parse the input into a data structure
    for (i, line) in challenge.input_lines().enumerate() {
        let mut is_parsing = false;
        let mut number_buf = Number::default();

        for (j, c) in line.chars().enumerate() {
            if c.is_ascii_digit() {
                if is_parsing {
                    // keep parsing
                    continue;
                } else {
                    // start parsing
                    is_parsing = true;
                    number_buf.line = i;
                    number_buf.start = j;
                }
            } else {
                if is_parsing {
                    // stop parsing
                    is_parsing = false;
                    number_buf.end = j;
                    number_buf.number = line[number_buf.start..number_buf.end].parse().unwrap();

                    numbers.push(number_buf);
                    number_buf = Number::default();
                }
                if c != '.' {
                    symbols.push(Symbol {
                        line: i,
                        pos: j,
                        character: c,
                    });
                }
            }
        }

        // check for a number at the end of the line
        if is_parsing {
            number_buf.end = line.len();
            number_buf.number = line[number_buf.start..number_buf.end].parse().unwrap();

            numbers.push(number_buf);
        }
    }

    let solution = numbers
        .iter()
        .filter_map(|num| {
            if num.has_adjacent_symbol(&symbols) {
                Some(num.number)
            } else {
                None
            }
        })
        .sum::<u64>();

    challenge.finish(solution);
}
