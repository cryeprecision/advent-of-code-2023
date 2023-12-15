#![feature(slice_split_once)]

use smallvec::SmallVec;

type Boxes = [Vec<Lense>; 256];

fn hash(input: &[u8]) -> u8 {
    input.iter().fold(0u64, |mut acc, &next| {
        acc += next as u64;
        acc *= 17;
        acc %= 256;
        acc
    }) as u8
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Remove {
        label: &'static [u8],
        box_idx: usize,
    },
    Insert {
        label: &'static [u8],
        box_idx: usize,
        focal_len: u8,
    },
}

impl From<&'static [u8]> for Op {
    fn from(value: &'static [u8]) -> Self {
        if let Some((label, _)) = value.split_once(|&b| b == b'-') {
            Op::Remove {
                label,
                box_idx: hash(label) as usize,
            }
        } else if let Some((label, focal_len)) = value.split_once(|&b| b == b'=') {
            Op::Insert {
                label,
                box_idx: hash(label) as usize,
                focal_len: focal_len[0] - b'0',
            }
        } else {
            unreachable!();
        }
    }
}

impl Op {
    fn execute(self, boxes: &mut Boxes) {
        match self {
            Op::Remove { label, box_idx } => {
                if let Some(idx) = boxes[box_idx].iter().position(|lense| lense.label == label) {
                    boxes[box_idx].remove(idx);
                }
            }
            Op::Insert {
                label,
                box_idx,
                focal_len,
            } => {
                if let Some(idx) = boxes[box_idx].iter().position(|lense| lense.label == label) {
                    boxes[box_idx][idx].focal_len = focal_len;
                } else {
                    boxes[box_idx].push(Lense { label, focal_len });
                }
            }
        }
    }
}

#[derive(Clone, Copy)]
struct Lense {
    label: &'static [u8],
    focal_len: u8,
}

impl std::fmt::Debug for Lense {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Lense")
            .field("label", &std::str::from_utf8(self.label).unwrap())
            .field("focal_len", &self.focal_len)
            .finish()
    }
}

fn main() {
    let challenge = advent_of_code_2023::Challenge::start(15, 2);

    // cannot init the array of boxes easily, because `Vec` doesn't implement `Copy`
    let mut boxes = (0..256)
        .map(|_| Vec::new())
        .collect::<SmallVec<Boxes>>()
        .into_inner()
        .unwrap();

    // parse input lazily
    let ops = challenge
        .input()
        .as_bytes()
        .split(|&b| b == b',')
        .map(Op::from);

    // run all operations
    ops.for_each(|op| op.execute(&mut boxes));

    let solution = boxes
        .iter()
        .enumerate()
        .map(|(i, box_)| {
            let box_value = box_
                .iter()
                .enumerate()
                .map(|(j, lense)| (j + 1) * (lense.focal_len as usize))
                .sum::<usize>();
            box_value * (i + 1)
        })
        .sum::<usize>();

    challenge.finish(solution);
}
