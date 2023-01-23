# Jsl
Just a stack based language

# Build
```cargo build```

```./target/debug/jsl run source.jsl```

# Debug
Show the final stack with `--stack` flag

example:

```./target/debug/jsl run source.jsl --stack```

this will return 0..8 of stack if you want more or less you can specify size

example:

```./target/debug/jsl run source.jsl --stack 32```

Or if you want to debug a memory(variables) you can use `--memory` flag instead of `--stack`

for example:

```./target/debug/jsl run source.jsl --memory```

# Functions
Functions can defined with `fn` keyword, for example

```
import std

fn callme -> {
  str Helloworld printstr
}

call callme
```

output:

```
HelloWorld
```

also you can specify a arguments of a function after name of function for example:

```
fn callme x y z -> {
  x put
  y put
  z put
}

1 2 3 call callme
```

output:

```
3
2
1
```

# Macros
For now macros removed from jsl but in the next updates its might be added.

# Let
`let` is like global variables

usage:

```
10 let x

x put
```

this will return `10`

You can set let value with `set` keyword

for example:

```
30 set x

x put
```

this will return `30`

# Strings
```
str HelloWorld put
```
result will be
```
HelloWorld
```

# Boolean
`true` -> will push 1 to stack
`false` -> will push 0 to stack

# Eq & NotEq
for Eq if two last elements in stack is equal , this will push bool to stack
```
1 2 eq put
```
will return `0`

For not equal
```
1 2 eq put
```
will return `1`

# Then
`then` scope if the top of stack is true
example:
```
1 1 eq then {
  str GOOD put
}
```
this will return 
```
GOOD
```
# Times
`times` is a keyword like `for` loops

`times` will pop the top of stack and runs the next scope `x` times

example:

```
3 times {
  1 print
}
```

will return:

```
1
1
1
```

# Import
example:

```
import lib.jsl

call test
```

will return:

```
HelloWorld
```

lib.jsl:

```
fn test -> {
  str HelloWorld putstr
}
```

OR import standard library

```
import std
import math
```
