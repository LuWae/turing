use crate::concrete::{Branch, Call, Machine, Primitive, Selector, State};
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;

#[derive(Debug)]
struct RawBranch<'a> {
    sel: Selector,
    primitives: Vec<Primitive>,
    call: Option<&'a str>,
}

#[derive(Debug)]
struct RawState<'a> {
    name: &'a str,
    branches: Vec<RawBranch<'a>>,
}

#[derive(Debug)]
struct RawMachine<'a> {
    states: Vec<RawState<'a>>,
}

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct TuringParser;

fn parse_char(pair: Pair<'_, Rule>) -> u8 {
    let bytes = pair.as_str().as_bytes();
    if bytes.len() == 3 {
        bytes[1]
    } else if bytes.len() == 5 {
        let hexbyte_to_u8 = |c| match c {
            b'0'..=b'9' => c - b'0',
            b'a'..=b'f' => c - b'a' + 10,
            _ => panic!("invalid hex byte: {}", c),
        };
        hexbyte_to_u8(bytes[2]) << 4 | hexbyte_to_u8(bytes[3])
    } else {
        panic!("unexpected char length")
    }
}

fn parse_selector(pair: Pair<'_, Rule>) -> Selector {
    let mut pairs = pair.into_inner();
    match pairs.next() {
        None => Selector::All,
        Some(char_pair) => {
            let mut chars = vec![parse_char(char_pair)];
            for pair in pairs {
                chars.push(parse_char(pair));
            }
            Selector::Chars(chars)
        }
    }
}

fn parse_action(pair: Pair<'_, Rule>) -> Primitive {
    match pair.as_str().as_bytes()[0] {
        b'<' => Primitive::Movel,
        b'>' => Primitive::Mover,
        b'=' => Primitive::Print(parse_char(pair.into_inner().next().unwrap())),
        _ => panic!("unexpected action"),
    }
}

fn parse_branch(pair: Pair<'_, Rule>) -> RawBranch<'_> {
    let mut pairs = pair.into_inner();
    let sel = parse_selector(pairs.next().unwrap());
    let mut primitives = Vec::new();
    let mut call = None;
    pairs.for_each(|pair| match pair.as_rule() {
        Rule::action => primitives.push(parse_action(pair)),
        Rule::id => {
            call = Some(pair.as_str());
        }
        _ => panic!("unexpected rule in parse_branch"),
    });
    RawBranch {
        sel,
        primitives,
        call,
    }
}

pub fn parse_machine(input: &str) -> Machine {
    let result = match TuringParser::parse(Rule::file, input) {
        Ok(mut pairs) => pairs.next().unwrap(),
        Err(e) => {
            panic!("{}", e);
        }
    };
    let m = RawMachine {
        states: result
            .into_inner()
            .filter(|pair| pair.as_rule() == Rule::statedesc)
            .map(|pair| {
                let mut inner_rules = pair.into_inner();
                RawState {
                    name: inner_rules.next().unwrap().as_str(),
                    branches: inner_rules.map(parse_branch).collect(),
                }
            })
            .collect(),
    };

    let state_map: HashMap<&str, usize> =
        HashMap::from_iter(m.states.iter().enumerate().map(|(i, s)| (s.name, i)));

    Machine {
        states: m
            .states
            .into_iter()
            .enumerate()
            .map(|(state_idx, raw_state)| State {
                branches: raw_state
                    .branches
                    .into_iter()
                    .map(|raw_branch| Branch {
                        sel: raw_branch.sel,
                        primitives: raw_branch.primitives,
                        call: match raw_branch.call {
                            Some("accept") => Call::Accept,
                            Some("reject") => Call::Reject,
                            Some(name) => match state_map.get(name) {
                                Some(idx) => Call::State(*idx),
                                None => panic!("state not found: {}", name),
                            },
                            None => Call::State(state_idx),
                        },
                    })
                    .collect(),
            })
            .collect(),
    }
}
