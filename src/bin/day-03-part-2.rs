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
    fn adjacent_numbers(&self, numbers: &[Number]) -> Vec<Number> {
        // Symbol OBB
        let symbol_min_line = self.line.saturating_sub(1);
        let symbol_max_line = self.line.saturating_add(1);
        let symbol_min_pos = self.pos.saturating_sub(1);
        let symbol_max_pos = self.pos.saturating_add(1);

        numbers
            .iter()
            .filter(|number| {
                // Number OBB
                let min_line = number.line.saturating_sub(1);
                let max_line = number.line.saturating_add(1);
                let min_pos = number.start.saturating_sub(1);
                let max_pos = number.end; // end is one past the end

                // OBB intersection
                symbol_max_line > min_line
                    && symbol_max_pos > min_pos
                    && symbol_min_line < max_line
                    && symbol_min_pos < max_pos
            })
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
    let lines = advent_of_code_2023::load_lines("./input/day-03-part-1.txt");

    let mut numbers = Vec::new();
    let mut symbols = Vec::new();

    for (i, line) in lines.iter().enumerate() {
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
                    number_buf.number = (&line[number_buf.start..number_buf.end]).parse().unwrap();

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

        // check for number at the end of the line
        if is_parsing {
            number_buf.end = line.len();
            number_buf.number = (&line[number_buf.start..number_buf.end]).parse().unwrap();

            numbers.push(number_buf);
        }
    }

    println!(
        "{}",
        symbols
            .iter()
            .filter_map(|sym| {
                let adj = sym.adjacent_numbers(&numbers);
                if adj.len() == 2 {
                    Some(adj[0].number * adj[1].number)
                } else {
                    None
                }
            })
            .sum::<u64>()
    )
}
