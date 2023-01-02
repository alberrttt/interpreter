def fib(n):
    pred = n < 2
    if pred:
        return n
    return fib(n-2)+fib(n-1)
print(fib(30))