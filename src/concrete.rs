#[derive(Debug)]
pub enum Primitive {
    Movel,
    Mover,
    Print(u8),
}

#[derive(Debug)]
pub struct RawBranch<'a> {
    pub syms: Vec<u8>,
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
