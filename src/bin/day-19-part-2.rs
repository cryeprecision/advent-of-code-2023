#![allow(dead_code)]

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
    fn is_empty(&self) -> bool {
        self.cool.is_none() && self.musical.is_none() && self.aero.is_none() && self.shiny.is_none()
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
    fn passing_subrange(self, range: &Range<u16>) -> Option<Range<u16>> {
        match self {
            CheckOp::LessThan(n) if range.start >= n => None,
            CheckOp::LessThan(n) => Some(range.start..n),
            CheckOp::GreaterThan(n) if range.end <= n + 1 => None,
            CheckOp::GreaterThan(n) => Some((n + 1)..range.end),
        }
    }
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
    prop: Property,
    op: CheckOp,
    dst: &'static str,
}

impl Check {
    /// Split into passing and not passing part ranges `[passing, non-passing]`
    fn split(self, part: &PartRange) -> [PartRange; 2] {
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
    fn check(&self, part: &PartRange) -> Result<&'static str, bool> {
        for check in &self.checks {
            match check.passing_subrange(part) {
                Some("R") => return Err(false),
                Some("A") => return Err(true),
                Some(next) => return Ok(next),
                None => (),
            }
        }
        match self.no_match {
            "R" => Err(false),
            "A" => Err(true),
            _ => Ok(self.no_match),
        }
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

    fn push(&mut self, name: &'static str, workflow: Workflow) {
        let hash = hash_name(name);
        let idx = self
            .inner
            .binary_search_by_key(&hash, |&(hash, _)| hash)
            .unwrap_err();
        self.inner.insert(idx, (hash, workflow));
    }
    fn get(&self, name: &'static str) -> Option<&Workflow> {
        self.inner
            .binary_search_by_key(&hash_name(name), |&(hash, _)| hash)
            .ok()
            .map(|idx| &self.inner[idx].1)
    }

    fn is_accepted(&self, part: &PartRange) -> bool {
        let mut current = self.get("in").unwrap();
        loop {
            match current.check(part) {
                Ok(next) => current = self.get(next).unwrap(),
                Err(accepted) => return accepted,
            }
        }
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

            // px{a<2006:qkq,m>2090:A,rfg}
            let (name, line) = line.split_once('{').unwrap();
            let mut checks = line[..line.len() - 1].split(',').collect::<Vec<_>>();
            let no_match = checks.pop().unwrap();

            let checks = checks
                .into_iter()
                .map(|check| {
                    // a<2006:qkq
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

    challenge.finish(0);
}
