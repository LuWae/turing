f(2) -> (1)
g(2)

call
  call
    f
	('a', call
	  g
	  ('b')
	)
  ('c')

f('a', g('b'))('c')

call = { (id | x_func) ~ "(" expr ~ ("," ~ expr)* ~  ")" }
# f is a function with two args returning a function with one arg.
|arg1, arg2| |arg3| arg3 ('a', 'b')
this is an ambiguity, because it could be
(|arg1, arg2| |arg3| arg3) ('a', 'b')
call {
	fn {
	    args: |arg1, arg2|,
	    return: fn {
	    	args: |arg3|
			return: arg3
	    },
	},
	fnargs: ('a', 'b')
}
or
|arg1, arg2| (|arg3| arg3 ('a', 'b'))
fn {
    args: |arg1, arg2|,
    return: call {
	    fn {
    	    args: |arg3|
		    return: arg3
        },
		fnargs: ('a', 'b')
	}
},
solutions:
- introduce call, like @f('a', 'b')
- see if you can massage the grammar so it can handle normal parentheses and func call parentheses
- disallow calling an anonymous function immediately
  - we do this for now!
