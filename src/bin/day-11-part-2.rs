use std::slice::ChunksExact;

use num_integer::Integer;

const EMPTY_FACTOR: usize = 1_000_000;

struct Image {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

impl Image {
    #[allow(dead_code)]
    fn debug_print(&self) {
        for line in self.data.chunks_exact(self.width) {
            println!("{}", std::str::from_utf8(line).unwrap());
        }
    }
    fn lines(&self) -> ChunksExact<u8> {
        self.data.chunks_exact(self.width)
    }
    fn galaxies(&self) -> Vec<Galaxy> {
        self.data
            .iter()
            .enumerate()
            .filter(|(_, &b)| b == b'#')
            .enumerate()
            .map(|(id, (idx, _))| Galaxy { id: id + 1, idx })
            .collect()
    }
}

#[allow(dead_code)]
struct Galaxy {
    id: usize,
    idx: usize,
}

impl Galaxy {
    fn distance_to(
        &self,
        other: &Galaxy,
        width: usize,
        empty_rows: &[usize],
        empty_cols: &[usize],
    ) -> usize {
        let (my_row, my_col) = self.idx.div_rem(&width);
        let (other_row, other_col) = other.idx.div_rem(&width);

        let (start_row, end_row) = (my_row.min(other_row), my_row.max(other_row));
        let (start_col, end_col) = (my_col.min(other_col), my_col.max(other_col));

        let empty_rows_hit = empty_rows
            .iter()
            .filter(|&&col| col > start_row && col < end_row)
            .count();
        let empty_cols_hit = empty_cols
            .iter()
            .filter(|&&row| row > start_col && row < end_col)
            .count();

        (end_row - start_row)
            + (end_col - start_col)
            + (empty_rows_hit + empty_cols_hit) * (EMPTY_FACTOR - 1)
    }
}

fn main() {
    let mut challenge = advent_of_code_2023::Challenge::start(11, 2);

    let original = {
        let width = challenge.input_lines().next().unwrap().len();
        let height = challenge.input_lines().count();

        let mut data = vec![b'.'; width * height];
        challenge.input_lines().enumerate().for_each(|(row, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .filter(|(_, &b)| b == b'#')
                .for_each(|(col, _)| data[row * width + col] = b'#');
        });

        Image {
            width,
            height,
            data,
        }
    };
    challenge.finish_parsing();

    let (empty_rows, empty_cols) = {
        // contains true, if the row[i] contains a galaxy
        let mut row_galaxy = vec![false; original.height];
        // contains true, if the col[i] contains a galaxy
        let mut col_galaxy = vec![false; original.width];

        original.lines().enumerate().for_each(|(row, line)| {
            // if the row contains any galaxy, mark it as such
            row_galaxy[row] = line.iter().any(|&c| c == b'#');

            line.iter().enumerate().for_each(|(col, &c)| {
                // if the column contains any galxy, mark it as such
                col_galaxy[col] |= c == b'#'
            });
        });

        let empty_rows = row_galaxy
            .iter()
            .enumerate()
            .filter(|(_, &not_empty)| !not_empty)
            .map(|(row_idx, _)| row_idx)
            .collect::<Vec<_>>();

        let empty_cols = col_galaxy
            .iter()
            .enumerate()
            .filter(|(_, &not_empty)| !not_empty)
            .map(|(row_idx, _)| row_idx)
            .collect::<Vec<_>>();

        (empty_rows, empty_cols)
    };

    let galaxies = original.galaxies();

    let solution = {
        (0..galaxies.len()).fold(0, |acc, lhs_idx| {
            acc + (lhs_idx..galaxies.len()).fold(0, |acc, rhs_idx| {
                acc + galaxies[lhs_idx].distance_to(
                    &galaxies[rhs_idx],
                    original.width,
                    &empty_rows,
                    &empty_cols,
                )
            })
        })
    };

    challenge.finish(solution);
}
