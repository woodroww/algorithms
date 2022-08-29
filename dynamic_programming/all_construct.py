# https://youtu.be/oBt53YbR9Kk?t=10060

# Write a function 'all_construct(target, word_bank)' that accepts a
# target string and an array of strings.
# 
# The function should return a 2D array containing all of the ways
# that the 'target' can be constructed by concatenating elements of
# the 'word_bank' array. Each element of the 2D array should represent
# one combination that constructs the 'target'.
# 
# You may reuse elements of 'word_bank' as many times as needed.

# m = len(target)
# n = len(word_bank)
# tree is m height
# worst case
# n^m leaves, so n^m subarrays

# time O(n^m)
# space O(m)

def all_construct(target, word_bank):
    if target == "":
        return [[]]
    result = []
    for word in word_bank:
        if target.find(word) == 0:
            suffix = target[len(word):]
            # array of ways to build the suffix
            suffixWays = all_construct(suffix, word_bank)
            # put this word in the front of every sub array of suffixWays
            for sub in suffixWays:
                sub.insert(0, word)
            # add to our result, but we have to get rid of outer [] from suffixWays
            for sub in suffixWays:
                result.append(sub)
    return result


assert(all_construct("purple", ["purp", "p", "ur", "le", "purpl"]) == [['purp', 'le'], ['p', 'ur', 'p', 'le']])

correct = [
    ['ab', 'cd', 'ef'],
    ['ab', 'c', 'def'],
    ['abc', 'def'],
    ['abcd', 'ef']
]
assert(all_construct("abcdef", ["ab", "abc", "cd", "def", "abcd", "ef", "c"]) == correct)
assert(all_construct("skateboard", ["bo", "rd", "ate", "t", "ska", "sk", "boar"]) == [])


# time and space the same here

def memo_all_construct(target, word_bank, memo):
    if target in memo:
        return memo[target]
    if target == "":
        return [[]]
    result = []
    for word in word_bank:
        if target.find(word) == 0:
            suffix = target[len(word):]
            # array of ways to build the suffix
            suffixWays = memo_all_construct(suffix, word_bank, memo)
            # put this word in the front of every sub array of suffixWays
            for sub in suffixWays:
                sub.insert(0, word)
            # add to our result, but we have to get rid of outer []
            for sub in suffixWays:
                result.append(sub)
    memo[target] = result 
    return result

# I could never get the memo version to work
# https://youtu.be/oBt53YbR9Kk?t=11291
memo = {}
hwat = memo_all_construct("purple", ["purp", "p", "ur", "le", "purpl"], memo)
print(hwat)

[['purp', 'le'], ['p', 'ur', 'p', 'le']]

[['p', 'ur', 'p', 'purp', 'le'], ['p', 'ur', 'p', 'purp', 'le']]



memo = {}
assert(memo_all_construct("purple", ["purp", "p", "ur", "le", "purpl"], memo) == [['purp', 'le'], ['p', 'ur', 'p', 'le']])

memo = {}
correct = [
    ['ab', 'cd', 'ef'],
    ['ab', 'c', 'def'],
    ['abc', 'def'],
    ['abcd', 'ef']
]
assert(memo_all_construct("abcdef", ["ab", "abc", "cd", "def", "abcd", "ef", "c"], memo) == correct)
memo = {}
assert(memo_all_construct("skateboard", ["bo", "rd", "ate", "t", "ska", "sk", "boar"], memo) == [])
memo = {}
assert(memo_all_construct("aaaaaaaaaaaaaaaaaaaaaaaaaaax", ["a", "aa", "aaa", "aaaa", "aaaaa"], memo) == [])




