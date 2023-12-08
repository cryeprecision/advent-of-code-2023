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

fn main() {
    let mut challenge = advent_of_code_2023::Challenge::start(8, 1);

    let (directions, nodes) = {
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

        (directions, nodes)
    };
    challenge.finish_parsing();

    let start: NodeId = "AAA".into();
    let end: NodeId = "ZZZ".into();

    let mut steps = 1u64;
    let mut current = nodes[start.position(&nodes)];

    for direction in directions.iter().cycle() {
        match direction {
            Dir::Left => current = nodes[current.left.1],
            Dir::Right => current = nodes[current.right.1],
        }
        if current.name == end {
            break;
        } else {
            steps += 1;
        }
    }

    challenge.finish(steps);
}
