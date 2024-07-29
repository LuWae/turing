use bitvec::prelude as bv;
use std::boxed::Box;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct StateDef<'a> {
    pub name: &'a str,
    pub params: Vec<&'a str>,
    pub branches: Vec<Branch<'a>>,
}

#[derive(Debug, Clone)]
pub struct Branch<'a> {
    pub sel: Selector<'a>,
    pub chain: Chain<'a>,
}

#[derive(Debug, Clone)]
pub enum Selector<'a> {
    Or(Vec<Selector<'a>>),
    And(Vec<Selector<'a>>),
    Not(Box<Selector<'a>>),
    All,
    Range(SelectorElem<'a>, SelectorElem<'a>),
    Elem(SelectorElem<'a>),
}

impl<'a> Selector<'a> {
    fn resolve(self, args: &HashMap<&'a str, ResolvedCallArg<'a>>) -> ResolvedSelector {
        use ResolvedSelector as R;
        use Selector as S;
        match self {
            S::Or(v) => R::Or(v.into_iter().map(|s| s.resolve(args)).collect()),
            S::And(v) => R::And(v.into_iter().map(|s| s.resolve(args)).collect()),
            S::Not(s) => R::Not(Box::new(s.resolve(args))),
            S::All => R::All,
            S::Range(start, end) => R::Range(start.resolve(args), end.resolve(args)),
            S::Elem(elem) => R::Elem(elem.resolve(args)),
        }
    }
}

#[derive(Debug, Clone)]
enum ResolvedSelector {
    Or(Vec<ResolvedSelector>),
    And(Vec<ResolvedSelector>),
    Not(Box<ResolvedSelector>),
    All,
    Range(u8, u8),
    Elem(u8),
}

impl ResolvedSelector {
    fn contains(&self, sym: u8) -> bool {
        use ResolvedSelector as RS;
        match self {
            RS::Or(v) => v.iter().any(|s| s.contains(sym)),
            RS::And(v) => v.iter().all(|s| s.contains(sym)),
            RS::Not(s) => !s.contains(sym),
            RS::All => true,
            RS::Range(start, end) => sym >= *start && sym <= *end,
            RS::Elem(sym2) => sym == *sym2,
        }
    }

    fn to_bitvec(&self) -> bv::BitVec {
        (0..256).map(|c| self.contains(c as u8)).collect()
    }
}

impl PartialEq for ResolvedSelector {
    fn eq(&self, other: &Self) -> bool {
        self.to_bitvec() == other.to_bitvec()
    }
}
impl Eq for ResolvedSelector {}

#[derive(Debug, Clone)]
pub enum SelectorElem<'a> {
    Sym(u8),
    Id(&'a str),
}

impl<'a> SelectorElem<'a> {
    fn resolve(self, args: &HashMap<&'a str, ResolvedCallArg<'a>>) -> u8 {
        use SelectorElem as SE;
        match self {
            SE::Sym(sym) => sym,
            SE::Id(s) => match args.get(s) {
                Some(arg) => match arg {
                    ResolvedCallArg::Sym(sym) => *sym,
                    ResolvedCallArg::Sel(sel) => match sel {
                        ResolvedSelector::Elem(sym) => *sym,
                        _ => panic!(
                            "attempted to substitute selector \"{:?}\" into SelectorElem",
                            sel
                        ),
                    },
                    ResolvedCallArg::Chain(c) => panic!(
                        "attempted to substitute chain \"{:?}\" into SelectorElem",
                        c
                    ),
                },
                None => panic!("unresolved in selector: {}", s),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct Chain<'a> {
    pub parts: Vec<ChainElem<'a>>,
}

impl<'a> Chain<'a> {
    fn resolve(self, args: &HashMap<&'a str, ResolvedCallArg<'a>>) -> ResolvedChain<'a> {
        // resolved chains are ALWAYS in chained form. This makes comparison trivial later.
        ResolvedChain {
            parts: self
                .parts
                .into_iter()
                .map(|part| part.resolve(args))
                .collect(),
        }
        .into_chained_repr()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ResolvedChain<'a> {
    parts: Vec<ResolvedChainElem<'a>>,
}

impl<'a> ResolvedChain<'a> {
    fn into_chained_repr(self) -> Self {
        let mut new_parts = Vec::new();
        for part in self.parts {
            match part {
                ResolvedChainElem::Call { id, mut args } => {
                    let rest = match args.pop() {
                        Some(ResolvedCallArg::Chain(c)) => Some(c),
                        Some(e) => {
                            args.push(e);
                            None
                        }
                        None => None,
                    };
                    new_parts.push(ResolvedChainElem::Call { id, args });
                    if let Some(c) = rest {
                        let mut c = c.into_chained_repr(); // TODO TCO?
                        new_parts.append(&mut c.parts);
                    }
                }
                e => new_parts.push(e),
            }
        }
        ResolvedChain { parts: new_parts }
    }
}

#[derive(Debug, Clone)]
pub enum ChainElem<'a> {
    Prim(Primitive<'a>),
    Call { id: &'a str, args: Vec<CallArg<'a>> },
    Accept,
    Reject,
}

impl<'a> ChainElem<'a> {
    fn resolve(self, args: &HashMap<&'a str, ResolvedCallArg<'a>>) -> ResolvedChainElem<'a> {
        match self {
            ChainElem::Prim(prim) => ResolvedChainElem::Prim(prim.resolve(args)),
            ChainElem::Call {
                id,
                args: call_args,
            } => ResolvedChainElem::Call {
                id,
                args: call_args
                    .into_iter()
                    .map(|call_arg| call_arg.resolve(args))
                    .collect(),
            },
            ChainElem::Accept => ResolvedChainElem::Accept,
            ChainElem::Reject => ResolvedChainElem::Reject,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ResolvedChainElem<'a> {
    Prim(ResolvedPrimitive),
    Call {
        id: &'a str,
        args: Vec<ResolvedCallArg<'a>>,
    },
    Accept,
    Reject,
}

#[derive(Debug, Clone)]
pub enum Primitive<'a> {
    Movel,
    Mover,
    Print(SelectorElem<'a>),
}

impl<'a> Primitive<'a> {
    fn resolve(self, args: &HashMap<&'a str, ResolvedCallArg<'a>>) -> ResolvedPrimitive {
        use Primitive as P;
        use ResolvedPrimitive as RP;
        match self {
            P::Movel => RP::Movel,
            P::Mover => RP::Mover,
            P::Print(elem) => RP::Print(match elem {
                SelectorElem::Sym(sym) => sym,
                SelectorElem::Id(s) => match args.get(s) {
                    Some(arg) => match arg {
                        ResolvedCallArg::Sym(sym) => *sym,
                        _ => panic!("expected sym for \"{}\", got {:?}", s, arg),
                    },
                    None => panic!("could not resolve \"{}\"", s),
                },
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ResolvedPrimitive {
    Movel,
    Mover,
    Print(u8),
}

#[derive(Debug, Clone)]
pub enum CallArg<'a> {
    Sym(u8),
    Sel(Selector<'a>),
    Chain(Chain<'a>),
    Id(&'a str),
}

impl<'a> CallArg<'a> {
    fn resolve(self, args: &HashMap<&'a str, ResolvedCallArg<'a>>) -> ResolvedCallArg<'a> {
        match self {
            CallArg::Sym(sym) => ResolvedCallArg::Sym(sym),
            CallArg::Sel(sel) => ResolvedCallArg::Sel(sel.resolve(args)),
            CallArg::Chain(chain) => ResolvedCallArg::Chain(chain.resolve(args)),
            CallArg::Id(s) => match args.get(s) {
                Some(call_arg) => call_arg.clone(),
                None => ResolvedCallArg::Chain(ResolvedChain {
                    // id that we haven't found: must be a state!
                    parts: vec![ResolvedChainElem::Call {
                        id: s,
                        args: Vec::new(),
                    }],
                }),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ResolvedCallArg<'a> {
    Sym(u8),
    Sel(ResolvedSelector),
    Chain(ResolvedChain<'a>),
}
