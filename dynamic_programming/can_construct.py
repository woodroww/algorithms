# https://youtu.be/oBt53YbR9Kk?t=7966

# Write a function 'can_construst(target, word_bank)' that accepts a
# target string and an array of string.
# 
# The function should return a boolean indicating whether or not the
# 'target' can be constructed by concatenating elements of the
# 'word_bank' array.
# 
# You may reuse elements of 'word_bank' as many times as needed.

# can_construst(skateboard, [bo, rd, ate, t, ska, sk, boar]) -> false
# can_construst(abcdef, [ab, abc, cd, def, abcd]) -> true

# Complexity
# m = len(target)
# n = len(word_bank)

# height of tree is m,
# worst case with all single letters in word_bank

# branching factor, n,
# worst case if every element in word_back could be a matching prefix

# target slicing to copy? suffix *m

# O(n^m * m) time

# Height of tree is m
# if suffix is a copy, idk in python
# O(m^2) space

def can_construct(target, word_bank):
    if target == "":
        return True
    for word in word_bank:
        if target.find(word) == 0:
            suffix = target[len(word):]
            if can_construct(suffix, word_bank) == True:
                return True
    return False

assert(can_construct("", ["cat", "dog", "mouse"]) == True)
assert(can_construct("abcdef", ["ab", "abc", "cd", "def", "abcd"]) == True)
assert(can_construct("skateboard", ["bo", "rd", "ate", "t", "ska", "sk", "boar"]) == False)
assert(can_construct("enterapotentpot", ["a", "p", "ent", "enter", "ot", "o", "t"]) == True)

# m = len(target)
# n = len(word_bank)

# no exploring duplicate subtrees
# the m^2 because of the suffix again
# O(n*m^2) time
# O(m^2) space

def memo_can_construct(target, word_bank, memo):
    if target in memo:
        return memo[target]
    if target == "":
        return True
    for word in word_bank:
        if target.find(word) == 0:
            suffix = target[len(word):]
            if memo_can_construct(suffix, word_bank, memo)== True:
                memo[target] = True
                return True
    memo[target] = False
    return False

memo = {}
assert(memo_can_construct("", ["cat", "dog", "mouse"], memo) == True)
memo = {}
assert(memo_can_construct("abcdef", ["ab", "abc", "cd", "def", "abcd"], memo) == True)
memo = {}
assert(memo_can_construct("skateboard", ["bo", "rd", "ate", "t", "ska", "sk", "boar"], memo) == False)
memo = {}
assert(memo_can_construct("enterapotentpot", ["a", "p", "ent", "enter", "ot", "o", "t"], memo) == True)
memo = {}
assert(memo_can_construct("eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeef", [
    "e",
    "ee",
    "eee",
    "eeee",
    "eeeee",
    "eeeeee",
    ], memo) == False)

