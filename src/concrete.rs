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

impl std::default::Default for Selector {
    fn default() -> Self {
        Selector::Chars(Vec::new())
    }
}

#[derive(Debug)]
pub struct RawBranch<'a> {
    pub sel: Selector,
    pub primitives: Vec<Primitive>,
    pub call: Option<&'a str>,
}

#[derive(Debug)]
pub struct RawState<'a> {
    pub name: &'a str,
    pub branches: Vec<RawBranch<'a>>,
}

#[derive(Debug)]
pub struct RawMachine<'a> {
    pub states: Vec<RawState<'a>>,
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
