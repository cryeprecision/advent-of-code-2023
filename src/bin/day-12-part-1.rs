#![allow(dead_code)]
#![feature(slice_group_by)]

#[derive(Clone)]
struct Spring {
    report: Vec<u8>,
    damaged_lens: Vec<usize>,
}

impl std::fmt::Debug for Spring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let report = std::str::from_utf8(&self.report).unwrap();
        let groups = self
            .groups()
            .map(|g| std::str::from_utf8(g).unwrap())
            .collect::<Vec<_>>();

        write!(f, "Spring {{ groups: {:?}, ", groups)?;
        write!(f, "damaged_lens: {:?}, ", self.damaged_lens)?;
        write!(f, "report: {:?} }}", report)
    }
}

impl Spring {
    fn damaged_spans(&self) -> impl Iterator<Item = &[u8]> {
        self.report
            .group_by(|&lhs, &rhs| lhs == b'#' && rhs == b'#')
            .filter(|&group| group[0] == b'#')
    }
    fn unknown_indices(&self) -> impl Iterator<Item = usize> + '_ {
        self.report
            .iter()
            .enumerate()
            .filter(|(_, &b)| b == b'?')
            .map(|(idx, _)| idx)
    }

    fn groups(&self) -> impl Iterator<Item = &[u8]> {
        self.report
            .group_by(|&lhs, &rhs| lhs != b'.' && rhs != b'.')
            .filter(|group| group[0] != b'.')
    }

    fn is_valid(&self) -> bool {
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

    springs.iter().for_each(|spring| println!("{:?}", spring));

    challenge.finish(0);
}
