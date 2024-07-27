# State Resolve
## Representation
We need a way to unambiguously represent state arguments, which can be
- symbols
- selectors
- chains
We need this to check if we already resolved a given state. For example, if we already have
`find('0'..'4' & !2, accept)`
we want to use it, even if the actual thing we try to resolve is
`find('0'|'1'|'3'|'4', accept)`.
Meaning we don't care about syntactical similarity, but semantic similarity. [^1]

Do we want this semantic equivalence?
Why we would want it: consider
```
do_if(sel, T, F) { [sel] T [!] F }
state {
  ['X'] > do_if('0'..'4' & !2, <<>) >>>
  ['Y'] <>> do_if('0'|'1'|'3'|'4', <, >>>)
}
```
These things may look completely different, but it's the same state.
- actually, `<<>` should NOT be equivalent to `<`, because of the amount of steps the machine takes!
- selectors, however, should be equivalent. Also, chaining and explicit argument passing.
Technically we want semantic equivalence on a state level: if we try to resolve a state, we want to know if we already have one with the same name, and the same content.
- -> yes, but "semantic equivalence at state level" <=> equivalence of state name and arguments.
- `ConcreteState` struct, or adapt the concrete representation


The simplest way to do this would be with an `Eq` implementation.

At the time where we do these equality checks, there cannot be any free variables; any free variables are assumed to be a state in a chain!
- for `Symbol`: trivial
- for `Selector`:
	- All `SelectorElem` must be `Sym`; we can't have unresolved `Id`s here.
	- perhaps we can represent this through a checked newtype.
- for `Chain`:
	- all unresolved things are treated as states.

[^1]: or do we? Perhaps this is a feature, so the user knows better where a given state came from. We could have different resolving modes