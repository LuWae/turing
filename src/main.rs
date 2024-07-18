/*
mod tape;
use crate::tape::Tape;
mod concrete;
use crate::concrete::{Primitive, Call, Selector, Branch, State, TuringMachine};

struct Execution<'a> {
    machine: &'a TuringMachine,
    state_idx: usize,
    pos: i32,
    tape: Tape,
}

impl<'a> Execution<'a> {
    fn step(&mut self) {
        let scan = self.tape.get(self.pos);
        let branch = self.machine.states[self.state_idx].branches.iter()
            .find(|branch| branch.syms.iter().any(|sel| sel.matches(scan)))
            .expect("no matching branch");

        for primitive in branch.primitives.iter() {
            match primitive {
                Primitive::Movel => { self.pos -= 1; },
                Primitive::Mover => { self.pos += 1; },
                Primitive::Print(sym) => { self.tape.set(self.pos, *sym); },
            }
        }
        match branch.call {
            Call::StateIdx(idx) => { self.state_idx = idx; },
            Call::Accept => todo!(),
            Call::Reject => todo!(),
        }
    }
}
*/

use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

mod concrete;
use concrete::{Primitive, RawBranch, RawMachine, RawState, Selector};

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

fn main() {
    let input = std::fs::read_to_string("test_machine.tm").unwrap();
    let result = match TuringParser::parse(Rule::file, &input) {
        Ok(mut pairs) => pairs.next().unwrap(),
        Err(e) => {
            println!("{}", e);
            return;
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
    println!("{:?}", m);
    /*
    let m = TuringMachine { states: vec![
        State {
            name: "main".to_string(),
            branches: vec![
                Branch {
                    syms: vec![Selector::Single(0)],
                    primitives: vec![Primitive::Movel],
                    call: Call::StateIdx(1),
                },
                Branch {
                    syms: vec![Selector::All],
                    primitives: vec![Primitive::Mover],
                    call: Call::StateIdx(0),
                }
            ],
        },
        State {
            name: "add".to_string(),
            branches: vec![
                Branch {
                    syms: vec![Selector::Single(b'1')],
                    primitives: vec![Primitive::Print(b'0'), Primitive::Movel],
                    call: Call::StateIdx(1),
                },
                Branch {
                    syms: vec![Selector::All],
                    primitives: vec![Primitive::Print(b'1')],
                    call: Call::Accept,
                }
            ],
        },
    ]};

    let mut exec = Execution {
        machine: &m,
        state_idx: 0,
        pos: 0,
        tape: Tape::from("1011"),
    };
    loop {
        exec.step();
    }
    */
}
