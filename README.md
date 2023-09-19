# Jsl
Just a stack based language

# Build
```cargo build```

```./target/debug/jsl source.jsl```

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

# Let
`let` is like global variables, unlike macros let cant hold expression only holds value `float64`

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
import std

str HelloWorld printstr
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
`then` runs function if the top of stack is true
example:
```
1 1 eq then {
    1 put
}
```

# Times
`times` is a keyword like `for` loops

times will pop the top of stack and do body of times x times

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

# Resource
Reverse Polish notation. (2023, August 14). In Wikipedia. https://en.wikipedia.org/wiki/Reverse_Polish_notation

Stack-oriented programming. (2023, July 30). In Wikipedia. https://en.wikipedia.org/wiki/Stack-oriented_programming
