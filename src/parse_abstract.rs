use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use std::boxed::Box;

#[derive(Debug)]
struct StateDef<'a> {
    name: &'a str,
    params: Vec<&'a str>,
    branches: Vec<(Selector<'a>, Chain<'a>)>,
}

#[derive(Debug)]
enum Selector<'a> {
    Or(Vec<Selector<'a>>),
    And(Vec<Selector<'a>>),
    Not(Box<Selector<'a>>),
    All,
    Range(SelectorElem<'a>, SelectorElem<'a>),
    Elem(SelectorElem<'a>),
}

#[derive(Debug)]
enum SelectorElem<'a> {
    Sym(u8),
    Id(&'a str),
}

#[derive(Debug)]
struct Chain<'a> {
    parts: Vec<ChainElem<'a>>,
    term: Option<Termination>,
}

#[derive(Debug)]
enum ChainElem<'a> {
    Prim(Primitive<'a>),
    Call { id: &'a str, args: Vec<CallArg<'a>> },
}

#[derive(Debug)]
enum Primitive<'a> {
    Movel,
    Mover,
    Print(SelectorElem<'a>),
}

#[derive(Debug)]
enum CallArg<'a> {
    Sym(u8),
    Sel(Selector<'a>),
    Chain(Chain<'a>),
    Id(&'a str),
}

#[derive(Debug)]
enum Termination {
    Accept,
    Reject,
}

#[derive(Parser)]
#[grammar = "abstract_grammar6.pest"]
struct AbstractParser;

fn parse_sym(pair: Pair<'_, Rule>) -> u8 {
    assert!(pair.as_rule() == Rule::char);
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

fn parse_selector_elem(pair: Pair<'_, Rule>) -> SelectorElem<'_> {
    match pair.as_rule() {
        Rule::char => SelectorElem::Sym(parse_sym(pair)),
        Rule::id => SelectorElem::Id(pair.as_str()),
        _ => panic!("unexpected rule type: {:?}", pair.as_rule()),
    }
}

fn parse_selector(pair: Pair<'_, Rule>) -> Selector<'_> {
    match pair.as_rule() {
        Rule::sel => parse_selector(pair.into_inner().next().unwrap()),
        Rule::sel_or => Selector::Or(pair.into_inner().map(|p| parse_selector(p)).collect()),
        Rule::sel_and => Selector::And(pair.into_inner().map(|p| parse_selector(p)).collect()),
        Rule::sel_not => match pair.into_inner().next() {
            Some(p) => Selector::Not(Box::new(parse_selector(p))),
            None => Selector::All,
        },
        Rule::sel_range => {
            let mut inner = pair.into_inner();
            Selector::Range(
                parse_selector_elem(inner.next().unwrap()),
                parse_selector_elem(inner.next().unwrap()),
            )
        }
        _ => Selector::Elem(parse_selector_elem(pair)),
    }
}

fn parse_primitive(pair: Pair<'_, Rule>) -> Primitive<'_> {
    match pair.as_str().as_bytes()[0] {
        b'<' => Primitive::Movel,
        b'>' => Primitive::Mover,
        b'#' => Primitive::Print(parse_selector_elem(pair.into_inner().next().unwrap())),
        _ => panic!("unexpected primitive: {}", pair.as_str()),
    }
}

fn parse_call_arg(pair: Pair<'_, Rule>) -> CallArg<'_> {
    let pair = pair.into_inner().next().unwrap();
    match pair.as_rule() {
        Rule::sel => CallArg::Sel(parse_selector(pair)),
        Rule::call_chain => CallArg::Chain(parse_call_chain(pair)),
        Rule::char => CallArg::Sym(parse_sym(pair)),
        Rule::id => CallArg::Id(pair.as_str()),
        _ => panic!("unexpected rule type: {:?}", pair.as_rule()),
    }
}

fn parse_call_chain(pair: Pair<'_, Rule>) -> Chain<'_> {
    let mut parts = Vec::new();
    let mut term = None;
    for p in pair.into_inner() {
        match p.as_rule() {
            Rule::primitive => parts.push(ChainElem::Prim(parse_primitive(p))),
            Rule::call => {
                let mut inner = p.into_inner();
                let id = inner.next().unwrap().as_str();
                let mut args = Vec::new();
                let maybe_tail = inner.next();
                if let Some(Rule::call_tail) = maybe_tail.as_ref().map(|p| p.as_rule()) {
                    for p in maybe_tail.unwrap().into_inner() {
                        args.push(parse_call_arg(p));
                    }
                }
                parts.push(ChainElem::Call { id, args });
            }
            Rule::keyword_accept => {
                term = Some(Termination::Accept);
            }
            Rule::keyword_reject => {
                term = Some(Termination::Reject);
            }
            _ => panic!("unexpected rule type: {:?}", p.as_rule()),
        }
    }
    Chain { parts, term }
}

fn parse_branch(pair: Pair<'_, Rule>) -> (Selector<'_>, Chain<'_>) {
    assert!(pair.as_rule() == Rule::branch);
    let mut inner = pair.into_inner();
    (
        parse_selector(inner.next().unwrap()),
        parse_call_chain(inner.next().unwrap()),
    )
}

fn parse_statedef(pair: Pair<'_, Rule>) -> StateDef<'_> {
    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str();

    let mut maybe_pair = inner.next();
    let mut params = Vec::new();
    if let Some(Rule::params) = maybe_pair.as_ref().map(|p| p.as_rule()) {
        for pair in maybe_pair.unwrap().into_inner() {
            params.push(pair.as_str())
        }
        maybe_pair = inner.next();
    }
    let branches = if let Some(Rule::call_chain) = maybe_pair.as_ref().map(|p| p.as_rule()) {
        vec![(Selector::All, parse_call_chain(maybe_pair.unwrap()))]
    } else {
        let mut b = Vec::new();
        while maybe_pair.is_some() {
            b.push(parse_branch(maybe_pair.unwrap()));
            maybe_pair = inner.next();
        }
        b
    };
    StateDef {
        name,
        params,
        branches,
    }
}

// TODO actually resolve this!
pub fn parse_abstract(input: &str) {
    let result = AbstractParser::parse(Rule::file, input)
        .unwrap()
        .next()
        .unwrap();
    let m: Vec<StateDef> = result
        .into_inner()
        .filter(|p| p.as_rule() == Rule::statedef)
        .map(|p| parse_statedef(p))
        .collect();
    for state in m {
        println!("{:?}", state);
    }
}
