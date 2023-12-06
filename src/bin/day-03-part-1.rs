#![allow(dead_code)]

#[derive(Debug, Default)]
struct Number {
    line: usize,
    start: usize,
    end: usize,
    number: u64,
}

impl Number {
    /// Check if the symbol is adjacent to this number
    fn is_adjacent_to(&self, symbol: &Symbol) -> bool {
        self.line.abs_diff(symbol.line) <= 1
            && self.start <= symbol.pos + 1
            && self.end >= symbol.pos
    }

    /// Check if this number has at least one adjacent symbol
    fn has_adjacent_symbol(&self, symbols: &[Symbol]) -> bool {
        // start of possible matches by line number
        let start = symbols.partition_point(|symbol| symbol.line + 1 < self.line);
        // end of possible matches by line number
        let end = symbols.partition_point(|symbol| symbol.line <= self.line + 1);

        symbols[start..end]
            .iter()
            .any(|symbol| self.is_adjacent_to(symbol))
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
