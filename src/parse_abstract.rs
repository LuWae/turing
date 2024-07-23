struct StateDef<'a> {
    name: &'a str,
    params: Vec<&'a str>,
    branches: Vec<(Selector<'a>, Chain<'a>)>,
}

enum Selector<'a> {
    Or(Vec<Selector<'a>>),
    And(Vec<Selector<'a>>),
    Not(Box<Selector<'a>>),
    All,
    Range(SelectorElem<'a>, SelectorElem<'a>),
    Elem(SelectorElem<'a>),
}

enum SelectorElem<'a> {
    Sym(u8),
    Id(&'a str),
}

struct Chain<'a> {
    parts: Vec<ChainElem<'a>>,
    term: Option<Termination>,
}

enum ChainElem<'a> {
    Prim(Primitive),
    Call { id: &'a str, args: Vec<CallArg> },
}

enum Primitive {
    Movel,
    Mover,
    Print(u8),
}

enum CallArg<'a> {
    Sym(u8),
    Sel(Selector),
    Call(Call),
    Id(&'a str),
}

enum Termination {
    Accept,
    Reject,
}
