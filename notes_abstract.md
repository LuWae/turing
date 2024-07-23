# Datatypes
- Sym: `'a'`, `'x00'` (unsigned byte)
  - special character: `$` is the character currently under the tape; can only be evaluated in an action-context!
- Selector: we use bracket syntax to distinguish single-sym selector from sym
  - no, we don't use bracket syntax! We want to be forced to place selectors explicitly in brackets, especially if they are arguments
  - `['a']`, `['a'|'b']`
    - may need to rethink using '|' here, as it's already used in functions! Comma for now.
    - commas are also difficult: consider f('a','b','c'): could be three arguments, or 2 with a selector!
    - keyword 'or' for now.
  - `['a'|['b']]` also works (this appears e.g. when we pass a selector as argument)
  - eventually we may want NOT, OR, AND, RANGE for selectors. but that can wait.
- Action: `<`, `>`, `# Sym`, `fr('a')`
  - we use `'#'` for printing because equals sign is used for assignments.
- ActionChain: sequence of Action
  - these are implicitly greedily gathered, e.g. we collect actions until we hit a let statement, another branch, or the end of the current block
  - think about having a better way to separate this, e.g. using a semicolon at the end!
- there is no 'State' datatype. A state is simply a match on the current tape character, which returns an action chain.
  - TODO think about if this results in any ambiguities with self and return; do we need a self::super?

- actlist cannot include return; we can't do find('a', >> return). parse returns outside.
  accepts, rejects must be inside through!
  - does return even make sense when we don't have "classical" states?
  - I guess return doesn't make sense, but self does. this conflicts with the concrete grammar though
  - maybe return does make sense. It just ends the current action chain.
  - would be unintuitive then why the > in fr('a', >) is not an infinite loop
  - should we parametrize action chains somehow based on return/self? make it different types?
  - let s = match $ {}
- we cannot allow top-level parentheses. `find(x)` could be:
  - action-chain comprised of find and x
  - function call to find with argument x
  - option: if no space between id and parens, treat it like a function call. otherwise, treat it like a chain.
  - perhaps use a simple call symbol for now. @f(main) is a call to function f with argument main. f(main) is an action chain with 2 parts: f and main.
- we have a problem: when should $ be evaluated? example:
  ```
  let main = fn(in) match in {
    let f = fn(a) $ // evaluating here not possible!
    ['x00']: chain { #f($) > f($) } // I actually don't know what I'm trying to show here.
  }
  ```
  - $ has to be evaluated immediately! throw error if not in dynamic context. treat $ not as a character, but a function call to get the current symbol. $ is not allowed outside of chains (but for traditional "states")
- differentiate between open chain and closed chain? otherwise we have multiple stays behind each other, e.g. in find and fr
- `let x = chain { >> self }` translated to: `let x = chain { >> x }`
- `let x = chain { >> return }` translated to: `let x = fn(E) chain { >> E }`
- there are also helf-open chains. `let x = match $ { ['0']: chain { self } ['1']: chain { return } }`
- an upper let also needs to know all lets coming after it
- challenge: mapping all language constructs onto the "original" abstract language

- the advantage of not having functions returning functions is that we never have a f()(); we only have one parentheses at most, and we know where it belongs to.
  - maybe this is a worthwhile restriction? but functions returning functions is cool, e.g. for partially applying arguments
# Language constructs

## Functions/Macros
```
let fr = |sel| {
  [sel] return
  [def] > self
}
let fr_a = fr('a')
```
- we use fn() for now, because it is clearer than the pipe |. We might use the pipe | for selectors instead.
### Nested
this, or calling functions that return functions, may be an ambiguity in the parser.
I have to take a closer look at it.
```
let fr = |x| |y| match x {
  [y] '1'
  [def] '0'
}
let a_equals_b = fr('a')('b')
```


## Match
```
let subst = |c| match c {
  ['0'] 'a'
  ['1'] 'b'
}
let string = |i| match i {
  ['x00'] 'h'
  ['x01'] 'i'
  ['x00'] '!'
}
```



# Simplifications

## Implicit match $
```
let s = {
  ['a'] #'b' accept
  ['b'] #'a' accept
}
```
is shorthand for 
```
let s = match $ {
  ['a'] #'b' accept
  ['b'] #'a' accept
}
```
because this is the most common case.
