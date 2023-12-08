use smallvec::SmallVec;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Dir {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Name([u8; 3]);

impl From<&str> for Name {
    fn from(value: &str) -> Self {
        assert_eq!(value.len(), 3);
        let vec = value
            .chars()
            .map(|c| c as u8)
            .collect::<SmallVec<[u8; 3]>>();
        Name(vec.into_inner().unwrap())
    }
}

#[derive(Debug, Clone, Copy)]
struct Location {
    name: Name,
    left: Name,
    right: Name,
}

fn main() {
    let challenge = advent_of_code_2023::Challenge::start(8, 1);

    let (directions, locations) = {
        let mut lines = challenge.input_lines();

        let directions = lines
            .next()
            .unwrap()
            .chars()
            .map(|c| match c {
                'L' => Dir::Left,
                'R' => Dir::Right,
                _ => panic!("unknown direction {}", c),
            })
            .collect::<Vec<_>>();

        // skip empty line
        let _ = lines.next().unwrap();

        let mut locations = lines
            .map(|line| {
                // 'HMS = (JBS, QFS)'
                let (name, left, right) = (&line[0..3], &line[7..10], &line[12..15]);
                Location {
                    name: name.into(),
                    left: left.into(),
                    right: right.into(),
                }
            })
            .collect::<Vec<_>>();
        locations.sort_unstable_by_key(|loc| loc.name);

        (directions, locations)
    };

    fn find_location(locs: &[Location], next: Name) -> Location {
        let next_idx = locs.binary_search_by_key(&next, |loc| loc.name).unwrap();
        locs[next_idx]
    }

    let start: Name = "AAA".into();
    let end: Name = "ZZZ".into();

    let mut steps = 0u64;
    let mut current = find_location(&locations, start);

    for direction in directions.iter().cycle() {
        steps += 1;
        match direction {
            Dir::Left => current = find_location(&locations, current.left),
            Dir::Right => current = find_location(&locations, current.right),
        }
        if current.name == end {
            break;
        }
    }

    challenge.finish(steps);
}
