# https://youtu.be/oBt53YbR9Kk?t=4200
# Write a function can_sum(target_sum, numbers)
# that takes in a target_sum and an array of numbers

# This function should return a boolean indicating whether or not it
# is possible to generate the target_sum unsing numbers from the array

# You may use an element of the array as many times as needed.

# You may assume that all input numbers are nonnegative.

# m = target sum
# n = array length
# tree height is going to be m
# max branches from a node is n

# O(n^m) time
# O(m) space

def can_sum(target_sum, numbers):
    if target_sum == 0:
        return True
    if target_sum < 0:
        return False
    for n in numbers:
        if can_sum(target_sum - n, numbers) == True:
            return True
    return False

assert(can_sum(7, [2, 3]) == True)
assert(can_sum(7, [5, 3, 4, 7]) == True)
assert(can_sum(7, [2, 4]) == False)
assert(can_sum(8, [2, 3, 5]) == True)


# m = target sum
# n = array length
# tree height is going to be m
# max branches from a node is n
# O(m*n) time
# O(m) space

def memo_can_sum(target_sum, numbers, memo):
    if target_sum in memo:
        return memo[target_sum]
    if target_sum == 0:
        return True
    if target_sum < 0:
        return False
    for n in numbers:
        if memo_can_sum(target_sum - n, numbers, memo) == True:
            memo[target_sum] = True
            return True
    memo[target_sum] = False
    return False

memo = {}
assert(memo_can_sum(7, [2, 3], memo) == True)
memo = {}
assert(memo_can_sum(7, [5, 3, 4, 7], memo) == True)
memo = {}
assert(memo_can_sum(7, [2, 4], memo) == False)
memo = {}
assert(memo_can_sum(8, [2, 3, 5], memo) == True)
memo = {}
assert(memo_can_sum(300, [7, 14], memo) == False)


# Tabulation solution
# https://youtu.be/oBt53YbR9Kk?t=13080
# m = targetSum
# n = number.length
# O(mn) time
# O(m) space

def table_can_sum(target_sum, numbers):
    table = [False for _ in range(target_sum + 1)]
    # table_can_sum(0, [...]) we can always answer target of 0 with True 
    table[0] = True
    for i in range(target_sum + 1):
        if table[i] == True:
            for n in numbers:
                if i + n < len(table):
                    table[i + n] = True
    return table[target_sum]

assert(table_can_sum(7, [2, 3]) == True)
assert(table_can_sum(7, [5, 3, 4, 7]) == True)
assert(table_can_sum(7, [2, 4]) == False)
assert(table_can_sum(8, [2, 3, 5]) == True)
assert(table_can_sum(300, [7, 14]) == False)




