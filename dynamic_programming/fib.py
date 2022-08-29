# https://youtu.be/oBt53YbR9Kk?t=11471

# Write a function 'fib(n)' that takes in a number as an argument.
# The function should return the n-th number of the Fibonacci sequence.
# 
# The 0th number is 0.
# The 1st number is 1.
# 
# n:      0, 1, 2, 3, 4, 5, 6,  7,  8,  9,
# fin(n): 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, ...

# tabulation 
# O(n) time
# O(n) space
def fib(n):
    table = [0 for _ in range(n + 2)]
    table[1] = 1
    for i in range(0, n):
        table[i + 1] += table[i];
        table[i + 2] += table[i];
    return table[n]


# https://youtu.be/oBt53YbR9Kk?t=210
# recursive memoization
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
    print(f"fib       {i} = {fib(i)}")

print(f"fibo_memo {50} = {fibo_memo(50, memo)}")
print(f"fib       {50} = {fib(50)}")


