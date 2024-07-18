use crate::concrete::{Branch, Call, Machine, Primitive, Selector, State};
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;
use thiserror::Error;

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

#[derive(Error, Debug)]
pub enum ParseError<'a> {
    #[error(transparent)]
    LowerParse(#[from] pest::error::Error<Rule>),
    #[error("unknown state name: {0}")]
    NameNotFound(&'a str),
}

pub fn parse_machine(input: &str) -> Result<Machine, ParseError<'_>> {
    let result = TuringParser::parse(Rule::file, input)?.next().unwrap();
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

    // TODO I'm thinking this is one of the situations where for loops would be better
    Ok(Machine {
        states: m
            .states
            .into_iter()
            .enumerate()
            .map(|(state_idx, raw_state)| {
                Ok(State {
                    branches: raw_state
                        .branches
                        .into_iter()
                        .map(|raw_branch| {
                            Ok(Branch {
                                sel: raw_branch.sel,
                                primitives: raw_branch.primitives,
                                call: match raw_branch.call {
                                    Some("accept") => Ok(Call::Accept),
                                    Some("reject") => Ok(Call::Reject),
                                    Some(name) => state_map
                                        .get(name)
                                        .map(|idx| Call::State(*idx))
                                        .ok_or(ParseError::NameNotFound(name)),
                                    None => Ok(Call::State(state_idx)),
                                }?,
                            })
                        })
                        .collect::<Result<Vec<Branch>, ParseError<'_>>>()?,
                })
            })
            .collect::<Result<Vec<State>, ParseError<'_>>>()?,
    })
}
