mod tape;
use crate::tape::Tape;
mod concrete;
use crate::concrete::{Branch, Call, Machine, Primitive, Selector, State};
mod parse;
use parse::{parse_machine, ParseData};

mod parse_abstract;

struct Execution<'a> {
    machine: &'a Machine,
    state_idx: usize,
    pos: i32,
    tape: Tape,
}

enum StepEvent {
    Continue,
    Accept,
    Reject,
}

impl<'a> Execution<'a> {
    fn step(&mut self) -> StepEvent {
        let scan = self.tape.get(self.pos);
        let branch = self.machine.states[self.state_idx]
            .branches
            .iter()
            .find(|branch| branch.sel.matches(scan))
            .expect("no matching branch");

        for primitive in branch.primitives.iter() {
            match primitive {
                Primitive::Movel => {
                    self.pos -= 1;
                }
                Primitive::Mover => {
                    self.pos += 1;
                }
                Primitive::Print(sym) => {
                    self.tape.set(self.pos, *sym);
                }
            }
        }
        match branch.call {
            Call::State(idx) => {
                self.state_idx = idx;
                StepEvent::Continue
            }
            Call::Accept => StepEvent::Accept,
            Call::Reject => StepEvent::Reject,
        }
    }
}

fn main_concrete() {
    let input = std::fs::read_to_string("test_machine.tm").unwrap();
    match parse_machine(&input) {
        Ok(ParseData {
            machine,
            state_names: _,
        }) => {
            println!("{:?}", machine);
            let mut exec = Execution {
                machine: &machine,
                state_idx: 0,
                pos: 0,
                tape: Tape::from("1011"),
            };
            while let StepEvent::Continue = exec.step() {
                // nothing.
            }
            println!("{:?}", exec.tape);
        }
        Err(e) => println!("{}", e),
    }
}

fn main() {
    let input = std::fs::read_to_string("machine.atm").unwrap();
    parse_abstract::parse_abstract(&input);
}
