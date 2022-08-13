from collections import OrderedDict

class LRUCache:

    def __init__(self, capacity: int):
        self.size = capacity
        self.cache = OrderedDict()

    def get(self, key: int) -> int:
        if key not in self.cache:
            return -1
        self.cache.move_to_end(key)
        return self.cache[key]

    def put(self, key: int, value: int) -> None:
        if key not in self.cache:
            if len(self.cache) >= self.size:
                self.cache.popitem(last=False)
        else:
            self.cache.move_to_end(key)
        self.cache[key] = value

commands = ["put", "put", "get", "put", "get", "put", "get", "get", "get"]
data = [[1, 1], [2, 2], [1], [3, 3], [2], [4, 4], [1], [3], [4]]
answers = [None, None, 1, None, -1, None, -1, 3, 4]


commands = ["put","put","put","put","get","get"]
data = [[2,1],[1,1],[2,3],[4,1],[1],[2]]
answers = [None,None,None,None,-1,3]


results = []
cache = LRUCache(2) # this 2 removed from data
for c, d in zip(commands, data):
    if c == "put":
        print(f"put {d[0]}, {d[1]}")
        jam = cache.put(d[0], d[1])
        results.append(jam)
    elif c == "get":
        print(f"get {d[0]}")
        jam = cache.get(d[0])
        results.append(jam)
    print(cache.cache)

print()
print(results)
print(answers)
print(results == answers)













