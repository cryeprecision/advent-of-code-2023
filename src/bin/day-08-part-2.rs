#![feature(slice_group_by)]

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Dir {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct NodeId(u32);

impl From<&str> for NodeId {
    fn from(value: &str) -> Self {
        assert_eq!(value.len(), 3);
        let shifted = value.as_bytes()[0] as u32
            | (value.as_bytes()[1] as u32) << 8
            | (value.as_bytes()[2] as u32) << 16;
        NodeId(shifted)
    }
}

impl NodeId {
    /// Extract the suffix from the compressed representation
    pub fn suffix(self) -> u8 {
        (self.0 >> 16) as u8
    }
    /// Location lookup based on NodeId
    fn position(self, nodes_sorted: &[Node]) -> usize {
        nodes_sorted
            .binary_search_by_key(&self, |loc| loc.name)
            .unwrap()
    }
}

#[derive(Debug, Clone, Copy)]
struct Node {
    name: NodeId,
    left: (NodeId, usize),
    right: (NodeId, usize),
}

fn least_common_multiple(nums: &[u64]) -> u64 {
    nums.iter().cloned().reduce(num_integer::lcm).unwrap()
}

fn main() {
    let challenge = advent_of_code_2023::Challenge::start(8, 2);

    let (directions, nodes, start_nodes) = {
        let mut lines = challenge.input_lines();

        let directions = lines.next().unwrap().as_bytes();
        let directions = directions
            .iter()
            .map(|&c| match c {
                b'L' => Dir::Left,
                b'R' => Dir::Right,
                _ => panic!("unknown direction {}", c),
            })
            .collect::<Vec<_>>();

        // skip empty line
        let _ = lines.next().unwrap();

        let mut nodes = lines
            .map(|line| Node {
                // 'HMS = (JBS, QFS)'
                name: line[0..3].into(),
                left: (line[7..10].into(), usize::MAX),
                right: (line[12..15].into(), usize::MAX),
            })
            .collect::<Vec<_>>();

        // sort for binary search
        nodes.sort_unstable_by_key(|loc| loc.name);

        // resolve location names to indices
        let lookup = nodes.clone();
        nodes.iter_mut().for_each(|loc| {
            loc.left.1 = loc.left.0.position(&lookup);
            loc.right.1 = loc.right.0.position(&lookup);
        });

        // extract starting fields
        let start_nodes = nodes
            .iter()
            .filter(|loc| loc.name.suffix() == b'A')
            .cloned()
            .collect::<Vec<_>>();

        (directions, nodes, start_nodes)
    };

    let steps_to_z = start_nodes
        .iter()
        .map(|current| {
            let mut steps = 1u64;
            let mut current = *current;

            for direction in directions.iter().cycle() {
                match direction {
                    Dir::Left => current = nodes[current.left.1],
                    Dir::Right => current = nodes[current.right.1],
                }
                if current.name.suffix() == b'Z' {
                    break;
                } else {
                    steps += 1;
                }
            }

            steps
        })
        .collect::<Vec<_>>();

    let solution = least_common_multiple(&steps_to_z);

    challenge.finish(solution);
}
