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

mod concrete;
mod parse;
use parse::{parse_machine, ParseData};

fn main() {
    let input = std::fs::read_to_string("test_machine.tm").unwrap();
    match parse_machine(&input) {
        Ok(ParseData {
            machine,
            state_names: _,
        }) => println!("{:?}", machine),
        Err(e) => println!("{}", e),
    }
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
