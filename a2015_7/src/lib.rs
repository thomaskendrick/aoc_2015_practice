use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Value<'a> {
    Signal(u16),
    Wire(&'a str),
}

#[derive(Debug, PartialEq)]
enum Instruction<'a> {
    RShift(&'a str, u8),
    LShift(&'a str, u8),
    And(Value<'a>, Value<'a>),
    Or(Value<'a>, Value<'a>),
    Not(&'a str),
    Direct(Value<'a>),
}

#[derive(Debug, PartialEq)]
struct WireGuide<'a> {
    instruction: Instruction<'a>,
    target_wire: &'a str,
}

fn to_guide(s: &str) -> WireGuide {
    let mut split = s.split(" -> ");
    WireGuide {
        instruction: to_instruction(split.next().unwrap()),
        target_wire: split.next().unwrap(),
    }
}

fn to_instruction(s: &str) -> Instruction {
    match s {
        x if x.contains("RSHIFT") => {
            let mut split = s.split(" RSHIFT ");
            Instruction::RShift(
                split.next().unwrap(),
                split.next().unwrap().parse::<u8>().unwrap(),
            )
        }
        x if x.contains("LSHIFT") => {
            let mut split = s.split(" LSHIFT ");
            Instruction::LShift(
                split.next().unwrap(),
                split.next().unwrap().parse::<u8>().unwrap(),
            )
        }
        x if x.contains("OR") => {
            let mut split = s.split(" OR ");
            Instruction::Or(
                to_value(split.next().unwrap()),
                to_value(split.next().unwrap()),
            )
        }
        x if x.contains("AND") => {
            let mut split = s.split(" AND ");
            Instruction::And(
                to_value(split.next().unwrap()),
                to_value(split.next().unwrap()),
            )
        }
        x if x.contains("NOT") => {
            let split = s.split("NOT ");
            Instruction::Not(split.last().unwrap())
        }
        x => Instruction::Direct(to_value(x)),
    }
}

fn to_value(s: &str) -> Value {
    match s.parse::<u16>() {
        Ok(x) => Value::Signal(x),
        Err(_) => Value::Wire(s),
    }
}

fn run_wires<'a>(
    mut computed_values: HashMap<&'a str, u16>,
    guides: &'a Vec<WireGuide>,
) -> HashMap<&'a str, u16> {
    while guides.len() != computed_values.len() {
        for g in guides.clone() {
            // Check to see if this wire is already in the computed values
            if computed_values.contains_key(g.target_wire) {
                continue;
            }
            // Otherwise try to compute the wire;
            match &g.instruction {
                Instruction::RShift(w, amt) => {
                    let r = computed_values.get(w);
                    if let Some(v) = r {
                        computed_values.insert(g.target_wire, v >> amt);
                    }
                }
                Instruction::LShift(w, amt) => {
                    let r = computed_values.get(w);
                    if let Some(signal) = r {
                        computed_values.insert(g.target_wire, signal << amt);
                    }
                }
                Instruction::And(v1, v2) => {
                    let sig1: u16 = match v1 {
                        Value::Wire(w) => {
                            let res = computed_values.get(w);
                            if let Some(sig) = res {
                                *sig
                            } else {
                                continue;
                            }
                        }
                        Value::Signal(s) => *s,
                    };
                    let sig2: u16 = match v2 {
                        Value::Wire(w) => {
                            let res = computed_values.get(w);
                            if let Some(sig) = res {
                                *sig
                            } else {
                                continue;
                            }
                        }
                        Value::Signal(s) => *s,
                    };
                    computed_values.insert(g.target_wire, sig1 & sig2);
                }
                Instruction::Or(v1, v2) => {
                    let sig1: u16 = match v1 {
                        Value::Wire(w) => {
                            let res = computed_values.get(w);
                            if let Some(sig) = res {
                                *sig
                            } else {
                                continue;
                            }
                        }
                        Value::Signal(s) => *s,
                    };
                    let sig2: u16 = match v2 {
                        Value::Wire(w) => {
                            let res = computed_values.get(w);
                            if let Some(sig) = res {
                                *sig
                            } else {
                                continue;
                            }
                        }
                        Value::Signal(s) => *s,
                    };
                    computed_values.insert(g.target_wire, sig1 | sig2);
                }
                Instruction::Not(w) => {
                    let res = computed_values.get(w);
                    if let Some(sig) = res {
                        computed_values.insert(g.target_wire, !*sig);
                    } else {
                        continue;
                    }
                }
                Instruction::Direct(v) => match v {
                    Value::Signal(sig) => {
                        computed_values.insert(g.target_wire, *sig);
                    }
                    Value::Wire(w) => {
                        let res = computed_values.get(w);
                        if let Some(sig) = res {
                            computed_values.insert(g.target_wire, *sig);
                        } else {
                            continue;
                        }
                    }
                },
            }
        }
    }
    computed_values
}

pub fn part_a_b(input: &str) -> (u16, u16) {
    let guides: Vec<WireGuide> = input.trim().lines().map(|l| to_guide(l)).collect();
    let map: HashMap<&str, u16> = HashMap::new();
    let completed_run = run_wires(map, &guides);
    let first_result = completed_run.get("a").unwrap();

    let mut rerun_map = HashMap::new();
    rerun_map.insert("b", *first_result);
    let second_run = run_wires(rerun_map, &guides);
    println!("{:?} second run", second_run);
    (*first_result, *second_run.get("a").unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("input.txt");

    #[test]
    fn part_a_b() {
        assert_eq!(super::part_a_b(INPUT), (46065, 14134));
    }

    #[test]
    fn to_instruction_test() {
        let test_instruction = "bn RSHIFT 2 -> bo";
        assert_eq!(
            to_guide(test_instruction),
            WireGuide {
                instruction: Instruction::RShift("bn", 2),
                target_wire: "bo"
            }
        )
    }
}
