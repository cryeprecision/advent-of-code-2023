#![allow(dead_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Pulse {
    Low,
    High,
}

impl Pulse {
    fn flip(self) -> Self {
        match self {
            Pulse::Low => Pulse::High,
            Pulse::High => Pulse::Low,
        }
    }
}

type Name = &'static str;

#[derive(Debug)]
enum Module {
    /// Flip-flop modules (prefix %) are either on or off; they are initially off.
    /// If a flip-flop module receives a high pulse, it is ignored and nothing happens.
    /// However, if a flip-flop module receives a low pulse, it flips between on and off.
    /// If it was off, it turns on and sends a high pulse.
    /// If it was on, it turns off and sends a low pulse.
    FlipFlop {
        name: Name,
        state: bool,
        outputs: Vec<Name>,
    },
    /// Conjunction modules (prefix &) remember the type of the most recent pulse
    /// received from each of their connected input modules; they initially default
    /// to remembering a low pulse for each input. When a pulse is received,
    /// the conjunction module first updates its memory for that input. Then,
    /// if it remembers high pulses for all inputs, it sends a low pulse;
    /// otherwise, it sends a high pulse.
    Conjunction {
        name: Name,
        inputs: Vec<(Name, Pulse)>,
        outputs: Vec<Name>,
    },
    /// There is a single broadcast module (named broadcaster).
    /// When it receives a pulse, it sends the same pulse to all of its destination modules.
    Broadcaster { name: Name, outputs: Vec<Name> },
}

impl Module {
    fn name(&self) -> Name {
        match self {
            Module::FlipFlop { name, .. } => name,
            Module::Conjunction { name, .. } => name,
            Module::Broadcaster { name, .. } => name,
        }
    }
    fn outputs(&self) -> &[Name] {
        match self {
            Module::FlipFlop { outputs, .. } => outputs,
            Module::Conjunction { outputs, .. } => outputs,
            Module::Broadcaster { outputs, .. } => outputs,
        }
    }
}

impl std::hash::Hash for Module {
    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
        match self {
            Module::FlipFlop { name, state, .. } => {
                name.hash(hasher);
                state.hash(hasher);
            }
            Module::Conjunction { name, inputs, .. } => {
                name.hash(hasher);
                for (_, pulse) in inputs {
                    pulse.hash(hasher);
                }
            }
            Module::Broadcaster { name, .. } => {
                name.hash(hasher);
            }
        }
    }
}

#[derive(Debug)]
struct PulseState {
    module: Name,
    pulse: Pulse,
}

fn main() {
    let mut challenge = advent_of_code_2023::Challenge::start(20, 1);

    let modules = {
        let parsed_input = challenge.input_lines().map(|line| {
            let (name, outputs) = line.split_once(" -> ").unwrap();

            let outputs: Vec<Name> = outputs.split(", ").collect();

            match name.as_bytes()[0] {
                b'%' => Module::FlipFlop {
                    name: &name[1..],
                    state: false,
                    outputs,
                },
                b'&' => Module::Conjunction {
                    name: &name[1..],
                    inputs: Vec::new(),
                    outputs,
                },
                _ => Module::Broadcaster { name, outputs },
            }
        });

        // first, parse all the modules
        let mut modules: Vec<Module> = parsed_input.clone().collect();
        modules.sort_unstable_by_key(Module::name);

        // then, determine inputs for the conjunction module
        for module in parsed_input {
            for output in module.outputs() {
                let Ok(output_idx) = modules.binary_search_by_key(output, Module::name) else {
                    eprintln!("unknown output module: {}", output);
                    continue;
                };
                if let Module::Conjunction { inputs, .. } = &mut modules[output_idx] {
                    inputs.push((module.name(), Pulse::Low));
                }
            }
        }

        modules
    };

    challenge.finish_parsing();

    modules.iter().for_each(|m| println!("{:?}", m));

    challenge.finish(0);
}
