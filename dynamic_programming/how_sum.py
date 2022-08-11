# https://youtu.be/oBt53YbR9Kk?t=5375
# Write a function 'how_sum(target_sum, numbers)' that takes in a
# target_sum and an array of numbers as arguments.
# 
# The function should return an array containing any combination of
# elements that add up to exactly the target_sum. If there is no
# combination that adds up to the target_sum, then return None.
# 
# If there are multiple combinations possible, you may return any
# single one.

# m = target_sum, depth of tree
# n = len(numbers), branches
# 
# O(n^m * m) time  (*m) for the copying of the result in the java of the video
# O(m) space

def how_sum(target_sum, numbers):
    if target_sum == 0:
        return []
    if target_sum < 0:
        return None
    for n in numbers:
        remainder = target_sum - n
        result = how_sum(remainder, numbers)
        if result is not None:
            result.append(n)
            return result
    return None

how_sum(7, [2, 3]) # [3, 2, 2]
how_sum(7, [5, 3, 4, 7]) # [3, 4], [4, 3] or [7]
how_sum(8, [2, 3, 5]) # [2, 2, 2, 2] or [3, 5]
how_sum(7, [2, 4]) # None
how_sum(0, [1, 2, 3]) # [] when target is 0

# O(n*m^2)
# O(m^2)
def memo_how_sum(target_sum, numbers, memo):
    if target_sum in memo:
        return memo[target_sum]
    if target_sum == 0:
        return []
    if target_sum < 0:
        return None
    for n in numbers:
        remainder = target_sum - n
        result = memo_how_sum(remainder, numbers, memo)
        if result is not None:
            result.append(n)
            memo[target_sum] = result
            return result
    memo[target_sum] = None
    return None

memo = {}
memo_how_sum(7, [2, 3], memo) # [3, 2, 2]
print(memo)
memo = {}
memo_how_sum(7, [5, 3, 4, 7], memo) # [3, 4], [4, 3] or [7]
print(memo)
memo = {}
memo_how_sum(8, [2, 3, 5], memo) # [2, 2, 2, 2] or [3, 5]
print(memo)
memo = {}
memo_how_sum(7, [2, 4], memo) # None
print(memo)
memo = {}
memo_how_sum(0, [1, 2, 3], memo) # [] when target is 0
print(memo)
memo = {}
memo_how_sum(300, [7, 14], memo) # None
print(memo)





