# https://youtu.be/oBt53YbR9Kk?t=6726

"""
Write a function 'best_sum(target_sum, numbers)' that takes in a
target_sum and an array of numbers as arguments.

The function should return an array containing the shortest
combination of numbers that add up to exactly the target_sum.

If there is a tie for the shortest combination, you may return any
one of the shortest.

"""

# m = target sum
# n = length of numbers
# Brute force

# time: O(n^m * m) *m for copying array in java or whatever this guy uses
# is it the same in python idk i want to say O(n^m)

# space: O(m^2)  the height of the recursion (m)
# every recursive call has a shortest array could be m so m^2

def best_sum(target_sum, numbers):
    if target_sum == 0:
        return []
    if target_sum < 0:
        return None;
    shortest = None
    for num in numbers:
        remainder = target_sum - num
        remainder_combo = best_sum(remainder, numbers)
        if remainder_combo is not None:
            combo = remainder_combo.copy()
            combo.append(num)
            if shortest is None or (len(combo) < len(shortest)):
                shortest = combo.copy()
    return shortest

# the order of the desired result does matter, for comparing arrays
assert(best_sum(7, [5, 3, 4, 7]) == [7])
assert(best_sum(8, [2, 3, 5]) == [5, 3])
assert(best_sum(8, [1, 4, 5]) == [4, 4])


def memo_best_sum(target_sum, numbers, memo):
    if target_sum in memo:
        return memo[target_sum]
    if target_sum == 0:
        return []
    if target_sum < 0:
        return None;
    shortest = None
    for num in numbers:
        remainder = target_sum - num
        remainder_combo = memo_best_sum(remainder, numbers, memo)
        if remainder_combo is not None:
            combo = remainder_combo.copy()
            combo.append(num)
            if shortest is None or (len(combo) < len(shortest)):
                shortest = combo.copy()
    memo[target_sum] = shortest
    return shortest

memo = {}
assert(memo_best_sum(7, [5, 3, 4, 7], memo) == [7])
memo = {}
assert(memo_best_sum(8, [2, 3, 5], memo) == [5, 3])
memo = {}
assert(memo_best_sum(8, [1, 4, 5], memo) == [4, 4])
memo = {}
assert(memo_best_sum(100, [1, 2, 5, 25], memo) == [25, 25, 25, 25])


# best_sum tabulation
# https://youtu.be/oBt53YbR9Kk?t=14843

def tabulation_best_sum(target_sum, numbers):
    table = [None for _ in range(target_sum + 1)]
    table[0] = []
    for i in range(target_sum + 1):
        if table[i] is not None:
            for n in numbers:
                if i+n < len(table):
                    combo = table[i].copy()
                    combo.append(n)
                    if table[i+n] is None or (len(combo) < len(table[i+n])):
                        table[i+n] = combo
    return table[target_sum]

assert(tabulation_best_sum(7, [5, 3, 4, 7]) == [7])
assert(tabulation_best_sum(8, [2, 3, 5]) == [3, 5])
assert(tabulation_best_sum(8, [1, 4, 5]) == [4, 4])
assert(tabulation_best_sum(100, [1, 2, 5, 25]) == [25, 25, 25, 25])
assert(tabulation_best_sum(100, [25, 2, 1, 5]) == [25, 25, 25, 25])



