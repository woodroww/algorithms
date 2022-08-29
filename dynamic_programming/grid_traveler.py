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



# https://youtu.be/oBt53YbR9Kk?t=12140
# grid tabulation

# Say that you are a traveler on a 2D grid. You begin in the
# top-left corner and your goal is to travel to the bottom-right
# corner. You may only move down or right.
# 
# In how many ways can you travel to the goal on a grid with
# dimensions m * n?
# 
# Write a function 'grid_traveler(m, n)' that calculates this.

# (rows, cols)
# grid_traveler(3, 3) -> 6

# O(mn)
# O(mn)

# use a grid with an extra col on the right and an extra row on the top
# then add the current to the right and lower cells
def table_grid_traveler(rows, cols):
    table = [[0 for _ in range(cols + 1)] for _ in range(rows + 1)] 
    table[1][1] = 1
    for i in range(rows + 1):
        for j in range(cols + 1):
            current = table[i][j]
            if j + 1 <= cols:
                table[i][j+1] += current
            if i + 1 <= rows:
                table[i+1][j] += current
    return table[rows][cols]

table_grid_traveler(1, 1) # 1
table_grid_traveler(2, 3) # 3
table_grid_traveler(3, 2) # 3
table_grid_traveler(3, 3) # 6
table_grid_traveler(18, 18) # 2333606220





