# limesherbet

This is an interpreter for my programming language.

Here is an example of the syntax:

```rs
func fib(n) {
    if n < 2 {
        return n;
    }
    
    return fib(n-2)+fib(n-1);
}
print(fib(30));
assert_eq fib(15), 610;
```

First clone the repo, then the interpreter can be run using the following command:
`cargo run --path <file path>`
