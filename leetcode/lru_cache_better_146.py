class LRUCache:

    def __init__(self, capacity: int):
        self.keys = []
        self.length = capacity
        self.cache = {}

    def get(self, key: int) -> int:
        if key in self.keys:
            i = self.keys.index(key)
            self.keys.pop(i)
            self.keys.append(key)
            return self.cache[key]
        return -1

    def put(self, key: int, value: int) -> None:
        if key in self.keys:
            self.cache[key] = value
            i = self.keys.index(key)
            self.keys.pop(i)
            self.keys.append(key)
        else:
            if len(self.cache) < self.length:
                self.keys.append(key)
                self.cache[key] = value
            else:
                old_key = self.keys.pop(0)
                del self.cache[old_key]
                self.put(key, value)

functions = {
    'get': LRUCache.get,
    'put': LRUCache.put,
}

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













