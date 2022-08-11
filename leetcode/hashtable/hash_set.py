# 0 <= key <= 10^6  (1_000_000)
# At most 10^4 calls will be made to add, remove, and contains. (10_000)

class HashItem:
    def __init__(self, key):
        self.key = key
    def key(self):
        return self.key


class MyHashSet:
    def __init__(self):
        self.storage = [[] for _ in range(100_000)]

    def hash(self, key: int):
        hash = key % 1000
        return hash
    
    def add(self, key: int) -> None:
        h = self.hash(key)
        if not self.contains(key):
            self.storage[h].append(HashItem(key))

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

functions = {
    'add': MyHashSet.add,
    'remove': MyHashSet.remove,
    'contains': MyHashSet.contains,
}

def process_data(set, commands, data):
    results = []
    for c, d in zip(commands, data):
        results.append(functions[c](set, d))
    return results

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

commands = ["add","add","contains","contains","add","contains","remove","contains"]
data = [1,2,1,3,2,2,2,2]
answers = [None,None,True,False,None,True,None,False]
set = MyHashSet()
results = process_data(set, commands, data)
print(results == answers)

commands, data, answers = load_from_file('hash_table.txt')
set = MyHashSet()
results = process_data(set, commands, data)
print(results == answers)





