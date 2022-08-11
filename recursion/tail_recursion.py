# https://www.youtube.com/watch?v=_JtPhF8MshA&t=752s

def factorial(n):
    if n == 0:
        return 1
    return n * factorial(n - 1)
# not tail recursive because after the recursive call there is still more to be
# done, namely the multiplication by n

factorial(1)
factorial(2)
factorial(3)
factorial(4)

# easy peasy
# fac n = n * fac n - 1

# fac 4 = 4 * fac 3
#       = 4 * (3 * fac 2)
#       = 4 * (3 * (2 * fac 1))
#       = 4 * (3 * (2 * 1))
#       = 4 * (3 * 2)
#       = 4 * 6
#       = 24

# memory inefficient
# see how the stack grows like the triangle out to the right

# now with tail recursion
# redefine factorial function with a helper function go
# a - accumulator

# fac n = go n 1
# go 1 a = a
# go n a = go (n - 1)(a * n)

# fac 4 = go 4 1
#       = go (4 - 1)(1 * 4)
#       = go 3 4
#       = go (3 - 1)(4 * 3)
#       = go 2 12
#       = go (2 - 1)(12 * 2)
#       = go 1 24

def go(n, acc):
    if n == 1:
        return acc
    return go((n - 1), (acc * n))

def tail_factorial(n):
    return go(n, 1)

for i in range(1, 7):
    print(f"tail factorial of {i} = {tail_factorial(i)}")
    print(f"     factorial of {i} = {factorial(i)}")


# Fibonacci sequence
# 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144
# inefficient
# - double recursion, two fib calls
# - no tail recursion, must add after the recursive calls
# - also the Fibonacci numbers are being repeatedly calculated

# fib 0 = 0
# fib 1 = 1
# fib n = fib(n-1) + fib(n-2)

# standard fibonacci with exponential complexity
# O(2^n) time
# O(2^n) space

def fibonacci(n):
    if n == 0:
        return 0
    if n == 1:
        return 1
    return fibonacci(n - 1) + fibonacci(n - 2)

for i in range(0, 8):
    print(f"fibonacci {i} = {fibonacci(i)}")

#              (current, next)
# fib n = go n (0, 1)
#
# go 0 (a,b) = a
# go 1 (a,b) = b
# go 2 (a,b) = go (n-1)(b, a+b)

# like a sliding window
# (2, 3) -> (3, 5)

# fib 4 = go 4 (0, 1)
#       = go 3 (1, 1)
#       = go 2 (1, 2)
#       = go 1 (2, 3)
#       = 3

def go_fib(n, a, b):
    if n == 0:
        return a
    return go_fib(n - 1, b, a + b)

def tail_fibonacci(n):
    return go_fib(n, 0, 1)

for i in range(0, 11):
    print(f"fibonacci      {i} = {fibonacci(i)}")
    print(f"tail_fibonacci {i} = {tail_fibonacci(i)}")
    print()




# To demonstrate time and space complexity
# O(2^n) time
# O(n) space
def dib(n):
    if n <= 1:
        return
    dib(n - 1)
    dib(n - 1)


# https://youtu.be/oBt53YbR9Kk?t=210
# memoization
# O(n) time
# O(n) space
def fibo_memo(n, memo):
    if n in memo:
        return memo[n]
    if n == 0:
        return 0
    if n == 1:
        return 1
    memo[n] = fibo_memo(n - 1, memo) + fibo_memo(n - 2, memo)
    return memo[n]

memo = {}
for i in range(0, 8):
    print(f"fibo_memo {i} = {fibo_memo(i, memo)}")
print(memo)

print(f"fibo_memo {50} = {fibo_memo(50, memo)}")
print(memo)












# aaaaaaaaaaaa
