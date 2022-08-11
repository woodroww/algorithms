# 0 <= key <= 10^6  (1_000_000)
# At most 10^4 calls will be made to add, remove, and contains. (10_000)

class HashItem:
    def __init__(self, key: int, value: int):
        self.key = key
        self.value = value
    def key(self):
        return self.key
    def value(self):
        return self.value
    def set_value(self, value):
        self.value = value;


class MyHashMap:
    def __init__(self):
        self.storage = [[] for _ in range(100_000)]
    def hash(self, key: int):
        hash = key % 1000
        return hash
    # inserts a (key, value) pair into the HashMap. If the key already exists
    # in the map, update the corresponding value.
    def put(self, key: int, value: int) -> None:
        h = self.hash(key)
        if not self.contains(key):
            self.storage[h].append(HashItem(key, value))
        else:
            storage_at_hash = list(map(HashItem.key, iter(self.storage[h])))
            self.storage[h][storage_at_hash.index(key)].set_value(value)
    # returns the value to which the specified key is mapped,
    # or -1 if this map contains no mapping for the key.
    def get(self, key: int) -> int:
        if self.contains(key):
            h = self.hash(key)
            storage_at_hash = list(map(HashItem.key, iter(self.storage[h])))
            return self.storage[h][storage_at_hash.index(key)].value
        else:
            return -1
    # removes the key and its corresponding value if the map contains the
    # mapping for the key.
    def remove(self, key: int) -> None:
        if self.contains(key):
            h = self.hash(key) 
            index = -1
            for i, item in enumerate(self.storage[h]):
                if key == item.key:
                    index = i
                    break;
            if index != -1:
                self.storage[h].pop(index)
            else:
                print(f"why didn't we find the key")
    def contains(self, key: int) -> bool:
        h = self.hash(key)
        if key in list(map(HashItem.key, iter(self.storage[h]))):
            return True
        return False


def wrong_indicies(results, answers):
    results = []
    for i, result in enumerate(results):
        if result != answers[i]:
            results.append(i)
    return results

def convert_answer_strings(answers):
    result = []
    for s in answers:
        if s == 'null':
            result.append(None)
        elif s == 'false':
            result.append(False)
        elif s == 'true':
            result.append(True)
        else:
            print("ERROR answer string invalid")
            return
    return result

def load_from_file(file_name):
    with open(file_name, 'r') as reader:
        lines = reader.readlines()
    assert(len(lines) == 3)
    command_line = lines[0]
    data_line = lines[1]
    answers_line = lines[2]
    commands = command_line.split('","')
    data_strings = data_line.split('],[')
    answer_strings = answers_line.split(',')
    assert(len(commands) == len(data_strings) == len(answer_strings))
    # remove the \n at the ends
    commands[-1] = commands[-1][:-1]
    data_strings[-1] = data_strings[-1][:-1]
    answer_strings[-1] = answer_strings[-1][:-1]
    data = [int(d) for d in data_strings]
    answers = convert_answer_strings(answer_strings)
    return commands, data, answers

functions = {
    'put': MyHashMap.put,
    'get': MyHashMap.get,
    'remove': MyHashMap.remove,
}

def process_data(hash_map, commands, data):
    results = []
    for c, d in zip(commands, data):
        if c == "put":
            hash_map.put(d[0], d[1])
            results.append(None)
        elif c == "get":
            results.append(hash_map.get(d[0]))
        elif c == "remove":
            hash_map.remove(d[0])
            results.append(None)
        else:
            print("something is wrong really wrong")
    return results

commands = ["put","put","get","get","put","get","remove","get"]
data = [[1,1],[2,2],[1],[3],[2,1],[2],[2],[2]]
answers = [None,None,1,-1,None,1,None,-1]

set = MyHashMap()
results = process_data(set, commands, data)
print(answers)
print(results)
print(results == answers)


commands, data, answers = load_from_file('hash_table.txt')
set = MyHashSet()
results = process_data(set, commands, data)
print(results == answers)





