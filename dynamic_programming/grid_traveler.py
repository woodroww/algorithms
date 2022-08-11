# there is a grid, you can only move down and right
# how many different paths can you take to get to the bottom right?
# https://youtu.be/oBt53YbR9Kk?t=2323
# Make a move and each move is then recursive into a smaller grid

# brute force
# height of binary tree formed by algorithm is height of m + n
# where m = x_size and n = y_size
# O(2^(n+m)) time
# O(n + m) space

def grid_traveler(x_size, y_size):
    if x_size == 0 or y_size == 0:
        return 0
    if x_size == 1 and y_size == 1:
        return 1
    # down and right
    return grid_traveler(x_size, y_size - 1) + grid_traveler(x_size - 1, y_size)

assert(grid_traveler(1, 1) == 1)
assert(grid_traveler(2, 3) == 3)
assert(grid_traveler(3, 2) == 3)
assert(grid_traveler(3, 3) == 6)

# grid_traveler(a, b) == grid_traveler(b, a)

# O(m * n) time
# O(n + m) space
def memo_traveler(x_size, y_size, memo):
    key = str(x_size) + ',' + str(y_size)
    if key in memo:
        return memo[key]
    if x_size == 0 or y_size == 0:
        return 0
    if x_size == 1 and y_size == 1:
        return 1
    memo[key] = memo_traveler(x_size, y_size - 1, memo) \
        + memo_traveler(x_size - 1, y_size, memo)
    return memo[key]

memo = {}
assert(memo_traveler(1, 1, memo) == 1)
memo = {}
assert(memo_traveler(2, 3, memo) == 3)
memo = {}
assert(memo_traveler(3, 2, memo) == 3)
memo = {}
assert(memo_traveler(3, 3, memo) == 6)
memo = {}
assert(memo_traveler(18, 18, memo) == 2333606220)






