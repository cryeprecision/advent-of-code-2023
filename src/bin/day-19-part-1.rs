#![allow(dead_code)]

#[derive(Debug, Clone, Copy)]
struct Part {
    cool: u16,
    musical: u16,
    aero: u16,
    shiny: u16,
}

#[derive(Debug, Clone, Copy)]
enum Property {
    Cool,
    Musical,
    Aero,
    Shiny,
}

impl Property {
    fn get(self, part: Part) -> u16 {
        match self {
            Property::Cool => part.cool,
            Property::Musical => part.musical,
            Property::Aero => part.aero,
            Property::Shiny => part.shiny,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum CheckOp {
    LessThan(u16),
    GreaterThan(u16),
}

impl CheckOp {
    fn check(self, value: u16) -> bool {
        match self {
            CheckOp::LessThan(n) => value < n,
            CheckOp::GreaterThan(n) => value > n,
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
    fn check(self, part: Part) -> Option<&'static str> {
        let value = self.prop.get(part);
        self.op.check(value).then_some(self.dst)
    }
}

#[derive(Debug, Clone)]
struct Workflow {
    name: &'static str,
    checks: Vec<Check>,
    no_match: &'static str,
}

fn main() {
    let mut challenge = advent_of_code_2023::Challenge::start(19, 1);

    let (workflows, parts) = {
        let mut lines = challenge.input_lines();

        let mut workflows = Vec::new();
        for line in lines.by_ref() {
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

            workflows.push(Workflow {
                name,
                checks,
                no_match,
            });
        }

        let mut parts = Vec::new();
        for line in lines {
            // {x=787,m=2655,a=1222,s=2876}
            let mut properties = line[1..line.len() - 1].split(',');

            parts.push(Part {
                cool: properties.next().unwrap()[2..].parse().unwrap(),
                musical: properties.next().unwrap()[2..].parse().unwrap(),
                aero: properties.next().unwrap()[2..].parse().unwrap(),
                shiny: properties.next().unwrap()[2..].parse().unwrap(),
            })
        }

        (workflows, parts)
    };

    challenge.finish_parsing();

    workflows
        .iter()
        .for_each(|workflow| println!("{:?}", workflow));
    parts.iter().for_each(|part| println!("{:?}", part));

    challenge.finish(0);
}
