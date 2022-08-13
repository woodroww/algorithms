class Solution:
    def isIsomorphic(self, s: str, t: str) -> bool:
        print(f"{s} - {t}")
        replacements = {}
        if len(s) != len(t):
            return False
        for s_char, t_char in zip(s, t):
            print(f"{s_char} ", end="")
            if s_char in replacements:
                print(f"found replacement {replacements[s_char]}")
                if replacements[s_char] != t_char:
                    print("False")
                    return False
            else:
                if t_char not in replacements.values():
                    print(f"no replacement, adding {s_char}: {t_char}")
                    replacements[s_char] = t_char
                else:
                    print(f"{t_char} already used as replacement")
                    print("False")
                    return False
        print("True")
        return True


inputs = [("egg", "add"), ("badc", "baba"), ("foo", "bar"), ("bbbaaaba", "aaabbbba")]
expected = [True, False, False, False]

results = []
for i in inputs:
    sol = Solution()
    results.append(sol.isIsomorphic(i[0], i[1]))

print(expected)
print(results)
print(expected == results) 

