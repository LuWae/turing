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

use pest::Parser;
use pest_derive::Parser;

mod concrete;
use concrete::{Primitive, RawBranch, RawMachine, RawState};

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct TuringParser;

fn main() {
    let input = "main { [def] < ='0' main } add { ['x00'] < }";
    let mut m = RawMachine { states: Vec::new() };
    let result = TuringParser::parse(Rule::file, input)
        .unwrap()
        .next()
        .unwrap();
    for i in result.into_inner() {
        if let Rule::statedesc = i.as_rule() {
            let mut inner_rules = i.into_inner();
            let state = RawState {
                name: inner_rules.next().unwrap().as_str(),
                branches: inner_rules
                    .map(|pair| {
                        println!("{:?}", pair);
                        RawBranch {
                            syms: Vec::new(),
                            primitives: Vec::new(),
                            call: None,
                        }
                    })
                    .collect(),
            };
            m.states.push(state);
        }
    }
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
