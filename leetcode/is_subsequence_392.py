class Solution:
    def isSubsequence(self, sub: str, seq: str) -> bool:
        seq_idx = 0
        found_count = 0
        for c in sub:
            for i in range(seq_idx, len(seq)):
                if c == seq[i]:
                    seq_idx = i + 1
                    found_count += 1
                    break;
        return found_count == len(sub)


