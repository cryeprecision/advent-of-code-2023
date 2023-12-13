#![feature(slice_group_by)]

#[derive(Clone)]
struct Spring {
    report: Vec<u8>,
    damaged_lens: Vec<usize>,
}

impl std::fmt::Debug for Spring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let report = std::str::from_utf8(&self.report).unwrap();
        write!(f, "Spring {{ ")?;
        write!(f, "report: {:<20}, ", report)?;
        write!(f, "lens: {:<15} }}", format!("{:?}", self.damaged_lens))
    }
}

impl Spring {
    pub fn damaged_spans(&self) -> impl Iterator<Item = &[u8]> + '_ {
        self.report
            .group_by(|&lhs, &rhs| lhs == b'#' && rhs == b'#')
            .filter(|&group| group[0] == b'#')
    }
    pub fn unknown_indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.report
            .iter()
            .enumerate()
            .filter(|(_, &b)| b == b'?')
            .map(|(idx, _)| idx)
    }

    pub fn is_valid(&self) -> bool {
        self.report.iter().all(|&b| b != b'?')
            && self
                .damaged_spans()
                .map(|span| span.len())
                .eq(self.damaged_lens.iter().cloned())
    }
}

fn main() {
    let mut challenge = advent_of_code_2023::Challenge::start(12, 1);

    let springs = challenge
        .input_lines()
        .map(|line| {
            let (report, damaged_lens) = line.split_once(' ').unwrap();

            let report = report.as_bytes().to_vec();

            // parse lengths
            let damaged_lens = damaged_lens
                .split(',')
                .map(|num| num.parse().unwrap())
                .collect::<Vec<_>>();

            Spring {
                report,
                damaged_lens,
            }
        })
        .collect::<Vec<_>>();
    challenge.finish_parsing();

    let solution = springs
        .into_iter()
        .map(|mut spring| {
            let unknowns = spring.unknown_indices().collect::<Vec<_>>();
            let possibilities = 2u64.pow(unknowns.len() as u32);
            let mut count = 0usize;

            for bits in 0..possibilities {
                // set the bits
                unknowns.iter().enumerate().for_each(|(idx, &unknown_idx)| {
                    if bits & (1 << idx) != 0 {
                        spring.report[unknown_idx] = b'.';
                    } else {
                        spring.report[unknown_idx] = b'#';
                    }
                });
                // check for valid report
                if spring.is_valid() {
                    count += 1;
                }
            }

            count
        })
        .sum::<usize>();

    challenge.finish(solution);
}
