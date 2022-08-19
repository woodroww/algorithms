# https://youtu.be/oBt53YbR9Kk?t=9517

# Write a function 'count_construct(target, word_bank)' that accepts a
# target string and an array of strings.
# 
# The function should return the number of ways that the 'target' can
# be constructed by concatenating elements of the 'word_bank' array.
# 
# You may reuse elements of 'word_bank' as many times as needed.

# count_construct(abcdef, [ab, abc, cd, def, abcd]) -> 1
# count_construct(purple, [purp, p, ur, le, purpl]) -> 2


# m = len(target)
# n = len(word_bank)

# O(n^m*m) time
# O(m^2) space

def count_construct(target, word_bank):
    if target == "":
        return 1
    total = 0
    for word in word_bank:
        if target.find(word) == 0:
            suffix = target[len(word):]
            sub_count = count_construct(suffix, word_bank)
            total += sub_count
    return total


assert(count_construct("", ["cat", "dog", "mouse"]) == 1)
assert(count_construct("purple", ["purp", "p", "ur", "le", "purpl"]) == 2)
assert(count_construct("abcdef", ["ab", "abc", "cd", "def", "abcd"]) == 1)
assert(count_construct("skateboard", ["bo", "rd", "ate", "t", "ska", "sk", "boar"]) == 0)
assert(count_construct("enterapotentpot", ["a", "p", "ent", "enter", "ot", "o", "t"]) == 4)

# O(n*m^2) time
# O(m^2) space
def memo_count_construct(target, word_bank, memo):
    if target in memo:
        return memo[target]
    if target == "":
        return 1
    total = 0
    for word in word_bank:
        if target.find(word) == 0:
            suffix = target[len(word):]
            sub_count =memo_count_construct(suffix, word_bank, memo)
            total += sub_count
    memo[target] = total
    return total

memo = {}
assert(memo_count_construct("", ["cat", "dog", "mouse"], memo) == 1)
memo = {}
assert(memo_count_construct("purple", ["purp", "p", "ur", "le", "purpl"], memo) == 2)
memo = {}
assert(memo_count_construct("abcdef", ["ab", "abc", "cd", "def", "abcd"], memo) == 1)
memo = {}
assert(memo_count_construct("skateboard", ["bo", "rd", "ate", "t", "ska", "sk", "boar"], memo) == 0)
memo = {}
assert(memo_count_construct("enterapotentpot", ["a", "p", "ent", "enter", "ot", "o", "t"], memo) == 4)
memo = {}
assert(memo_count_construct("eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeef", [
    "e",
    "ee",
    "eee",
    "eeee",
    "eeeee",
    "eeeeee",
    ], memo) == 0)


