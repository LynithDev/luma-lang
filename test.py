def fib(n):
    if n <= 1:
        return n
    else:
        return fib(n-1) + fib(n-2)

print(fib(25))

# def test(n):
#     if n <= 1:
#         return n
#     else:
#         return test(n - 1)

# print(test(5))
