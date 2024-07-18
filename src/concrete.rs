#[derive(Debug)]
pub enum Primitive {
    Movel,
    Mover,
    Print(u8),
}

#[derive(Debug, Clone)]
pub enum Selector {
    All,
    Chars(Vec<u8>),
}

#[derive(Debug)]
pub enum Call {
    Accept,
    Reject,
    State(usize),
}

#[derive(Debug)]
pub struct Branch {
    pub sel: Selector,
    pub primitives: Vec<Primitive>,
    pub call: Call,
}

#[derive(Debug)]
pub struct State {
    pub branches: Vec<Branch>,
}

#[derive(Debug)]
pub struct Machine {
    pub states: Vec<State>,
}
