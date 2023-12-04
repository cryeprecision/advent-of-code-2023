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
    let input = advent_of_code_2023::load_input("day-03.txt");
    let start = std::time::Instant::now();

    let mut numbers = Vec::new();
    let mut symbols = Vec::new();

    // parse the input into a data structure
    for (i, line) in input.lines().enumerate() {
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

    let result = numbers
        .iter()
        .filter_map(|num| {
            if num.has_adjacent_symbol(&symbols) {
                Some(num.number)
            } else {
                None
            }
        })
        .sum::<u64>();

    let elapsed = start.elapsed().as_secs_f64() * 1e3;
    println!("{} ({:.3})", result, elapsed);
}
