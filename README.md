# Jsl
Just a stack based language

# Build
```cargo build```

```./target/debug/jsl source.jsl```

# Debug
Show the final stack with `--stack` flag

example:

```./target/debug/jsl source.jsl --stack```

this will return 0..8 of stack if you want more or less you can specify size

example:

```./target/debug/jsl source.jsl --stack 32```

Or if you want to debug a memory(variables) you can use `--memory` flag instead of `--stack`

for example:

```./target/debug/jsl source.jsl --memory```

# Functions
Functions can defined with `fn` keyword, for example

```
import std

fn callme do
  str Helloworld printstr
end

callme
```

output:

```
HelloWorld
```

also you can specify a arguments of a function after name of function for example:

```
fn callme x y z do
  x put
  y put
  z put
end

1 2 3 callme
```

output:

```
3
2
1
```

# Macros
```
macro x
  1 2 add
end
```
if you call macro compiler will run body of macro
for example:
```
x put
```
will return: `3`

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

Also you can use a `Lets` keyword to assing multiple lets in one line

example:

```
1 2 3 lets x y z ok
```

This will equal to
x = 3
y = 2
z = 1

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
import std

macro x str GOOD printstr end
1 1 eq then x
```
this will return 
```
GOOD
```
# Times
`times` is a keyword like `for` loops

times will pop the top of stack and do body of times x times

example:

```
3 times
  1 print
done
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

test
```

will return:

```
HelloWorld
```

lib.jsl:

```
macro test
  str HelloWorld putstr
end
```

OR import standard library

```
import std
import math
```
