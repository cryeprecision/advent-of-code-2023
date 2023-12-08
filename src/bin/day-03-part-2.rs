#![allow(dead_code)]

#[derive(Debug, Default, Clone)]
struct Number {
    line: usize,
    start: usize,
    end: usize,
    number: u64,
}

#[derive(Debug, Default, Clone)]
struct Symbol {
    line: usize,
    pos: usize,
    character: char,
}

impl Symbol {
    /// Check if the number is adjacent to this symbol
    fn is_adjacent_to(&self, number: &Number) -> bool {
        self.line.abs_diff(number.line) <= 1
            && self.pos + 1 >= number.start
            && self.pos <= number.end
    }

    /// Find all numbers adjacent to this gear
    fn adjacent_numbers(&self, numbers: &[Number]) -> Vec<Number> {
        // start of possible matches by line number
        let start = numbers.partition_point(|number| number.line + 1 < self.line);
        // end of possible matches by line number
        let end = numbers.partition_point(|number| number.line <= self.line + 1);

        // only check numbers which could be adjacent based on the line number
        numbers[start..end]
            .iter()
            .filter(|number| self.is_adjacent_to(number))
            .cloned()
            .collect()
    }
}

struct Line {
    line: String,
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

fn main() {
    let mut challenge = advent_of_code_2023::Challenge::start(3, 2);

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
                    // finish parsing
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
    challenge.finish_parsing();

    // remove all symbols we don't care about
    symbols.retain(|symbol| symbol.character == '*');

    let solution = symbols.iter().fold(0, |acc, next| {
        let adj = next.adjacent_numbers(&numbers);
        if adj.len() == 2 {
            acc + adj[0].number * adj[1].number
        } else {
            acc
        }
    });

    challenge.finish(solution);
}
