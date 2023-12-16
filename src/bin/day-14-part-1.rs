use std::fmt::Write;

#[derive(Hash)]
struct Image {
    data: Vec<u8>,
    width: usize,
}

impl Image {
    fn height(&self) -> usize {
        self.data.len() / self.width
    }

    fn row(&self, row_idx: usize) -> &[u8] {
        &self.data[(row_idx * self.width)..((row_idx + 1) * self.width)]
    }

    fn tilt_up(&mut self) {
        let height = self.height();
        (0..self.width).for_each(|col_idx| {
            let mut free_spot = None;
            (0..height).for_each(|row_idx| {
                let from_idx = row_idx * self.width + col_idx;
                match (self.data[from_idx], free_spot) {
                    (b'.', None) => free_spot = Some(from_idx),
                    (b'#', Some(_)) => free_spot = None,
                    (b'O', Some(to_idx)) => {
                        self.move_rock(from_idx, to_idx);
                        free_spot = Some(to_idx + self.width);
                    }
                    _ => (),
                }
            });
        });
    }

    fn move_rock(&mut self, from: usize, to: usize) {
        self.data[from] = b'.';
        self.data[to] = b'O';
    }

    fn weight(&self) -> u64 {
        let height = self.height();
        self.data
            .iter()
            .enumerate()
            .filter(|(_, &b)| b == b'O')
            .map(|(idx, _)| (height - (idx / self.width)) as u64)
            .sum::<u64>()
    }
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut rows = 0..self.height();
        if let Some(row) = rows.next() {
            let row = std::str::from_utf8(self.row(row)).unwrap();
            f.write_str(row)?;

            for row in rows {
                let row = std::str::from_utf8(self.row(row)).unwrap();
                f.write_char('\n')?;
                f.write_str(row)?;
            }
        }
        Ok(())
    }
}

fn main() {
    let mut challenge = advent_of_code_2023::Challenge::start(14, 1);

    let mut image = {
        let width = challenge.input_lines().next().unwrap().len();
        let mut data = Vec::new();
        challenge
            .input_lines()
            .for_each(|line| data.extend_from_slice(line.as_bytes()));
        Image { data, width }
    };
    challenge.finish_parsing();

    image.tilt_up();
    let solution = image.weight();

    challenge.finish(solution);
}
