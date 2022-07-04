# Jsl
Just a stack based language

# Functions
```
fn x
  1 2 add
end
```
you call run function by calling function name
for example:
```
x put
```
will return: `3`

# Strings
```
str HelloWorld putstr
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
fn x str GOOD putstr end
1 1 eq then x
```
this will return 
```
GOOD
```
# Var
`var` is much like functions, but var holds value not expression, when its called value of var will pushed to the stack
example:
```
var x 1

x put
```
this will return `1`
