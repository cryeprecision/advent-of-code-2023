use std::slice::ChunksExact;

use num_integer::Integer;

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
    fn at(&self, row: usize, col: usize) -> u8 {
        self.data[row * self.width + col]
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
    fn distance_to(&self, other: &Galaxy, width: usize) -> usize {
        let (my_row, my_col) = self.idx.div_rem(&width);
        let (other_row, other_col) = other.idx.div_rem(&width);

        my_row.abs_diff(other_row) + my_col.abs_diff(other_col)
    }
}

fn main() {
    let mut challenge = advent_of_code_2023::Challenge::start(11, 1);

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

    let expanded = {
        // contains false, if the row[i] does not contain a galaxy
        let mut row_galaxy = vec![false; original.height];
        // contains false, if the col[i] does not contain a galaxy
        let mut col_galaxy = vec![false; original.width];

        original.lines().enumerate().for_each(|(row, line)| {
            // if the row contains any galaxy, mark it as such
            row_galaxy[row] = line.iter().any(|&c| c == b'#');

            line.iter().enumerate().for_each(|(col, &c)| {
                // if the column contains any galxy, mark it as such
                col_galaxy[col] |= c == b'#'
            });
        });

        let free_rows = row_galaxy.iter().filter(|&&g| !g).count();
        let free_cols = col_galaxy.iter().filter(|&&g| !g).count();

        let width = original.width + free_cols;
        let height = original.height + free_rows;

        let mut data = vec![b'.'; width * height];
        let (mut row_offset, mut col_offset) = (0, 0);

        for row in 0..original.height {
            if !row_galaxy[row] {
                row_offset += 1;
            }
            for col in 0..original.width {
                if !col_galaxy[col] {
                    col_offset += 1;
                }
                if original.at(row, col) == b'#' {
                    data[(row + row_offset) * width + (col + col_offset)] = b'#';
                }
            }
            col_offset = 0;
        }

        Image {
            width,
            height,
            data,
        }
    };

    let galaxies = expanded.galaxies();

    let solution = {
        let galaxies_width = galaxies.len();
        (0..galaxies_width).fold(0, |acc, lhs_idx| {
            acc + (lhs_idx..galaxies_width).fold(0, |acc, rhs_idx| {
                acc + galaxies[lhs_idx].distance_to(&galaxies[rhs_idx], expanded.width)
            })
        })
    };

    challenge.finish(solution);
}
