#[derive(Debug)]
pub enum Primitive {
    Movel,
    Mover,
    Print(u8),
}

#[derive(Debug)]
pub enum Selector {
    All,
    Chars(Vec<u8>),
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
