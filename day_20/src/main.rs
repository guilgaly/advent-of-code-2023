use common::itertools::Itertools;
use std::collections::{HashMap, VecDeque};

static INPUT: &str = include_str!("input");

fn main() {
    let init_system = parse_input(INPUT);

    let res1 = part_1(&init_system);
    println!("Part 1 result: {}", res1);

    let res2 = part_2(&init_system);
    println!("Part 2 result: {}", res2);
}

fn part_1(init_system: &System) -> usize {
    let mut system = init_system.clone();
    let mut cmd_queue = VecDeque::new();
    let mut low_count = 0;
    let mut high_count = 0;

    let mut send_cmd =
        |cmd_queue: &mut VecDeque<Command>, from: &str, tos: &Vec<String>, pulse: Pulse| {
            match pulse {
                Pulse::High => high_count += tos.len(),
                Pulse::Low => low_count += tos.len(),
            };
            tos.iter().for_each(|to| {
                cmd_queue.push_front(Command { from: from.to_owned(), to: to.to_owned(), pulse })
            });
        };

    for _ in 0..1000 {
        send_cmd(
            &mut cmd_queue,
            "button",
            &vec!["broadcaster".to_owned()],
            Pulse::Low,
        );
        while let Some(Command { from, to, pulse }) = cmd_queue.pop_back() {
            if let Some(module) = system.get_mut(&to) {
                match module {
                    Module::Broadcaster { targets } => {
                        send_cmd(&mut cmd_queue, &to, targets, pulse);
                    }
                    Module::FlipFlop { targets, is_on } if pulse == Pulse::Low => {
                        if *is_on {
                            *is_on = false;
                            send_cmd(&mut cmd_queue, &to, targets, Pulse::Low);
                        } else {
                            *is_on = true;
                            send_cmd(&mut cmd_queue, &to, targets, Pulse::High);
                        }
                    }
                    Module::Conjunction { targets, last_received } => {
                        last_received.insert(from, pulse);
                        if last_received.values().contains(&Pulse::Low) {
                            send_cmd(&mut cmd_queue, &to, targets, Pulse::High);
                        } else {
                            send_cmd(&mut cmd_queue, &to, targets, Pulse::Low);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    low_count * high_count
}

fn part_2(init_system: &System) -> usize {
    let mut system = init_system.clone();
    let mut cmd_queue = VecDeque::new();
    let mut button_count = 0;

    fn send_cmd(cmd_queue: &mut VecDeque<Command>, from: &str, tos: &Vec<String>, pulse: Pulse) {
        tos.iter().for_each(|to| {
            cmd_queue.push_front(Command { from: from.to_owned(), to: to.to_owned(), pulse })
        });
    }

    // the four modules which feed into nc, which itself feeds into rx
    let mut lk_count = 0;
    let mut fn_count = 0;
    let mut fh_count = 0;
    let mut hh_count = 0;

    loop {
        button_count += 1;
        send_cmd(
            &mut cmd_queue,
            "button",
            &vec!["broadcaster".to_owned()],
            Pulse::Low,
        );
        while let Some(Command { from, to, pulse }) = cmd_queue.pop_back() {
            // Note: this works because the four counts found are prime. Otherwise, we'd need to fin the lcm.
            if to == "nc" && pulse == Pulse::High {
                println!(
                    "nc received high pulse from {} on count {}",
                    from, button_count
                );
                if from == "lk" {
                    lk_count = button_count;
                }
                if from == "fn" {
                    fn_count = button_count;
                }
                if from == "fh" {
                    fh_count = button_count;
                }
                if from == "hh" {
                    hh_count = button_count;
                }
            }
            if lk_count != 0 && fn_count != 0 && fh_count != 0 && hh_count != 0 {
                return lk_count * fn_count * fh_count * hh_count;
            }
            if let Some(module) = system.get_mut(&to) {
                match module {
                    Module::Broadcaster { targets } => {
                        send_cmd(&mut cmd_queue, &to, targets, pulse);
                    }
                    Module::FlipFlop { targets, is_on } if pulse == Pulse::Low => {
                        if *is_on {
                            *is_on = false;
                            send_cmd(&mut cmd_queue, &to, targets, Pulse::Low);
                        } else {
                            *is_on = true;
                            send_cmd(&mut cmd_queue, &to, targets, Pulse::High);
                        }
                    }
                    Module::Conjunction { targets, last_received } => {
                        last_received.insert(from, pulse);
                        if last_received.values().contains(&Pulse::Low) {
                            send_cmd(&mut cmd_queue, &to, targets, Pulse::High);
                        } else {
                            send_cmd(&mut cmd_queue, &to, targets, Pulse::Low);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

fn parse_input(input: &str) -> System {
    let mut s = system_parser::system(input).unwrap();
    let targets = s
        .iter()
        .flat_map(|(l, m)| m.targets().iter().map(|t| (l.to_owned(), t.to_owned())))
        .collect_vec();

    for (source, target) in targets {
        if let Some(Module::Conjunction { last_received, .. }) = s.get_mut(&target) {
            last_received.insert(source, Pulse::Low);
        }
    }
    s
}

peg::parser! {
    grammar system_parser() for str {
        rule label() -> String = l:$(['a'..='z']+) { l.to_owned() }
        rule module() -> (String, Module) = p:$(['%' | '&']?) l:label() " -> " targets:(label() ** ", ") {
            let m = match p {
                "%" => Module::FlipFlop { targets, is_on: false },
                "&" => Module::Conjunction { targets, last_received: HashMap::new() },
                _ => Module::Broadcaster { targets },
            };
            (l, m)
        }
        pub rule system() -> System = s:(module() ** "\n") { s.into_iter().collect() }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug)]
struct Command {
    from: String,
    to: String,
    pulse: Pulse,
}

type System = HashMap<String, Module>;

impl Module {
    fn targets(&self) -> &Vec<String> {
        match self {
            Module::Broadcaster { targets, .. } => targets,
            Module::FlipFlop { targets, .. } => targets,
            Module::Conjunction { targets, .. } => targets,
        }
    }
}

#[derive(Debug, Clone)]
enum Module {
    Broadcaster {
        targets: Vec<String>,
    },
    FlipFlop {
        targets: Vec<String>,
        is_on: bool,
    },
    Conjunction {
        targets: Vec<String>,
        last_received: HashMap<String, Pulse>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_1: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    static TEST_INPUT_2: &str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

    #[test]
    fn test_part_1_input_1() {
        assert_eq!(part_1(&parse_input(TEST_INPUT_1)), 32000000);
    }

    #[test]
    fn test_part_1_input_2() {
        assert_eq!(part_1(&parse_input(TEST_INPUT_2)), 11687500);
    }
}
