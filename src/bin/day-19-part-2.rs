use std::{
    hash::{Hash, Hasher},
    ops::Range,
};

#[derive(Debug, Clone)]
struct PartRange {
    cool: Option<Range<u16>>,
    musical: Option<Range<u16>>,
    aero: Option<Range<u16>>,
    shiny: Option<Range<u16>>,
}

impl PartRange {
    fn new() -> PartRange {
        PartRange {
            cool: Some(1..4001),
            musical: Some(1..4001),
            aero: Some(1..4001),
            shiny: Some(1..4001),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Property {
    Cool,
    Musical,
    Aero,
    Shiny,
}

impl Property {
    fn get(self, part: &PartRange) -> Option<Range<u16>> {
        match self {
            Property::Cool => part.cool.clone(),
            Property::Musical => part.musical.clone(),
            Property::Aero => part.aero.clone(),
            Property::Shiny => part.shiny.clone(),
        }
    }
    fn set(self, part: &PartRange, range: Option<Range<u16>>) -> PartRange {
        let mut part = part.clone();
        match self {
            Property::Cool => part.cool = range,
            Property::Musical => part.musical = range,
            Property::Aero => part.aero = range,
            Property::Shiny => part.shiny = range,
        }
        part
    }
}

#[derive(Debug, Clone, Copy)]
enum CheckOp {
    /// Passes every value that is strictly less than
    LessThan(u16),
    /// Passes every value that is strictly greater than
    GreaterThan(u16),
}

impl CheckOp {
    /// Return the part of the range that passes this check
    fn passing_subrange(self, range: &Range<u16>) -> Option<Range<u16>> {
        match self {
            CheckOp::LessThan(n) if range.start >= n => None,
            CheckOp::LessThan(n) => Some(range.start..n),
            CheckOp::GreaterThan(n) if range.end <= n + 1 => None,
            CheckOp::GreaterThan(n) => Some((n + 1)..range.end),
        }
    }

    /// Return the part of the range that doesn't pass this check
    fn non_passing_subrange(self, range: &Range<u16>) -> Option<Range<u16>> {
        match self {
            // does not pass anything greater or equal to n
            CheckOp::LessThan(n) => CheckOp::GreaterThan(n - 1).passing_subrange(range),
            // does not pass anything smaller or equal to n
            CheckOp::GreaterThan(n) => CheckOp::LessThan(n + 1).passing_subrange(range),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Check {
    /// Which property this check applies to
    prop: Property,
    /// Which kind of check this is
    op: CheckOp,
    /// Next workflow, if the check passes
    dst: &'static str,
}

impl Check {
    /// Split into passing and not passing part ranges `((passing, dst), non_passing)`
    fn split_range(self, part: &PartRange) -> [PartRange; 2] {
        let passing_range = self
            .prop
            .get(part)
            .and_then(|range| self.op.passing_subrange(&range));
        let passing = self.prop.set(part, passing_range);

        let non_passing_range = self
            .prop
            .get(part)
            .and_then(|range| self.op.non_passing_subrange(&range));
        let non_passing = self.prop.set(part, non_passing_range);

        [passing, non_passing]
    }
}

#[derive(Debug, Clone)]
struct Workflow {
    checks: Vec<Check>,
    no_match: &'static str,
}

impl Workflow {
    fn process_range(&self, mut range: PartRange, queue: &mut Vec<(&'static str, PartRange)>) {
        for check in &self.checks {
            let [passing, non_passing] = check.split_range(&range);
            queue.push((check.dst, passing));
            range = non_passing;
        }
        queue.push((self.no_match, range));
    }
}

#[derive(Debug, Clone)]
struct Workflows {
    inner: Vec<(u64, Workflow)>,
}

fn hash_name(name: &str) -> u64 {
    let mut hasher = std::hash::DefaultHasher::new();
    name.hash(&mut hasher);
    hasher.finish()
}

impl Workflows {
    fn new() -> Workflows {
        Workflows { inner: Vec::new() }
    }

    fn push(&mut self, name: &str, workflow: Workflow) {
        let hash = hash_name(name);
        let idx = self
            .inner
            .binary_search_by_key(&hash, |&(hash, _)| hash)
            .unwrap_err();
        self.inner.insert(idx, (hash, workflow));
    }
    fn get(&self, name: &str) -> Option<&Workflow> {
        self.inner
            .binary_search_by_key(&hash_name(name), |&(hash, _)| hash)
            .ok()
            .map(|idx| &self.inner[idx].1)
    }

    /// Process ranges and return final ranges with partition point `p`.
    /// All elements in `[..p]` are accepted and all in `[p..]` are rejected.
    fn process_range(&self, range: PartRange) -> (Vec<PartRange>, usize) {
        let mut queue = Vec::new();
        self.get("in").unwrap().process_range(range, &mut queue);

        // find the next part range to process, remove it from the queue and return it
        fn pop_range(
            ranges: &mut Vec<(&'static str, PartRange)>,
        ) -> Option<(&'static str, PartRange)> {
            let idx_rev = ranges
                .iter()
                .rev() // pop from the back so fewer elements have to be moved
                .position(|&(name, _)| name != "R" && name != "A")?;
            Some(ranges.remove(ranges.len() - idx_rev - 1))
        }

        while let Some((next_name, next_range)) = pop_range(&mut queue) {
            self.get(next_name)
                .unwrap()
                .process_range(next_range, &mut queue);
        }

        // sort and determine partition point
        queue.sort_unstable_by_key(|&(name, _)| name);
        let partition_point = queue.partition_point(|&(name, _)| name == "A");

        // omit names of workstations as they can be inferred
        let queue = queue.into_iter().map(|(_, range)| range).collect();

        (queue, partition_point)
    }
}

fn main() {
    let mut challenge = advent_of_code_2023::Challenge::start(19, 2);

    let workflows = {
        let mut workflows = Workflows::new();
        for line in challenge.input_lines() {
            if line.is_empty() {
                // workflows and parts are separated by an empty line
                break;
            }

            // parse `px{a<2006:qkq,m>2090:A,rfg}`
            let (name, line) = line.split_once('{').unwrap();
            let mut checks = line[..line.len() - 1].split(',').collect::<Vec<_>>();
            let no_match = checks.pop().unwrap();

            let checks = checks
                .into_iter()
                .map(|check| {
                    // parse `a<2006:qkq`
                    let (check, dst) = check.split_once(':').unwrap();

                    let prop = match check.as_bytes()[0] {
                        b'x' => Property::Cool,
                        b'm' => Property::Musical,
                        b'a' => Property::Aero,
                        b's' => Property::Shiny,
                        _ => panic!(),
                    };

                    let value = check[2..].parse::<u16>().unwrap();
                    let op = match check.as_bytes()[1] {
                        b'<' => CheckOp::LessThan(value),
                        b'>' => CheckOp::GreaterThan(value),
                        _ => panic!(),
                    };

                    Check { dst, op, prop }
                })
                .collect::<Vec<_>>();

            workflows.push(name, Workflow { checks, no_match });
        }
        workflows
    };
    challenge.finish_parsing();

    let (ranges, partition_point) = workflows.process_range(PartRange::new());

    let solution = ranges[..partition_point]
        .iter()
        .map(|range| {
            range.cool.as_ref().map(|r| r.len()).unwrap_or(0)
                * range.musical.as_ref().map(|r| r.len()).unwrap_or(0)
                * range.aero.as_ref().map(|r| r.len()).unwrap_or(0)
                * range.shiny.as_ref().map(|r| r.len()).unwrap_or(0)
        })
        .sum::<usize>();

    challenge.finish(solution);
}
