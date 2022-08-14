# https://youtu.be/oBt53YbR9Kk?t=6726

"""
Write a function 'best_sum(target_sum, numbers)' that takes in a
target_sum and an array of numbers as arguments.

The function should return an array containing the shortest
combination of numbers that add up to exactly the target_sum.

If there is a tie for the shortest combination, you may return any
one of the shortest.

"""

def best_sum(target_sum, numbers):
    print(f"best_sum({target_sum}, {numbers})")
    if target_sum == 0:
        return []
    if target_sum < 0:
        return None;
    shortest = None
    for num in numbers:
        remainder = target_sum - num
        remainder_combo = best_sum(remainder, numbers)
        print(f"remainder_combo {remainder_combo}")
        if remainder_combo != None:
            combo = remainder_combo.append(num)
            if shortest is not None:
                if len(combo) < len(shortest):
                    shortest = combo
            else:
                shortest = combo
    print(f"\t{shortest}")
    return shortest

result = best_sum(7, [5, 3, 4, 7])
print(result)
assert(result == [7])

assert(best_sum(8, [2, 3, 5]) == [3, 5])
assert(best_sum(8, [1, 4, 5]) == [4, 4])
assert(best_sum(100, [1, 2, 5, 25]) == [25, 25, 25, 25])







