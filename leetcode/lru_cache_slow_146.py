
class LRUCache:

    def __init__(self, capacity: int):
        self.cache = {}
        self.length = capacity
        self.get_count = 1

    def get(self, key: int) -> int:
        if key in self.cache:
            self.cache[key]['used'] = self.get_count
            self.get_count += 1
            return self.cache[key]['value']
        return -1

    def put(self, key: int, value: int) -> None:
        if key in self.cache:
            self.cache[key]['value'] = value
            self.cache[key]['used'] = self.get_count 
            self.get_count += 1
        else:
            if len(self.cache) < self.length: 
                self.cache[key] = { 'value': value, 'used': self.get_count }
                self.get_count += 1
            else:
                first_key = list(self.cache.keys())[0]
                used = self.cache[first_key]['used']
                remove_key = first_key
                for k in self.cache:
                    use_count = self.cache[k]['used']
                    if use_count < used:
                        used = use_count
                        remove_key = k
                print(f"remove_key {remove_key}")
                self.cache.pop(remove_key)        
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













