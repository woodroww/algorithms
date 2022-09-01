# https://github.com/joeyajames/Python/blob/master/graph_adjacency-list.py

class Vertex:

    def __init__(self, n):
        self.name = n
        self.neighbors = list()
    
    def add_neighbor(self, v, weight):
        if v not in self.neighbors:
            self.neighbors.append((v, weight))
            self.neighbors.sort()

class Graph:

    vertices = {}

    def add_vertex(self, vertex):
        if isinstance(vertex, Vertex) and vertex.name not in self.vertices:
            self.vertices[vertex.name] = vertex
            return True
        else:
            return False

    def add_edge(self, u, v, weight=0):
        if u in self.vertices and v in self.vertices:
            self.vertices[u].add_neighbor(v, weight)
            self.vertices[v].add_neighbor(u, weight)
            return True
        else:
            return False

    def print_graph(self):
        for key in sorted(list(self.vertices.keys())):
            print(key + " - " + str(self.vertices[key].neighbors))

    def get_paths(self, start, end, path=[]):
        path = path + [start]

        if start == end:
            return [path]

        if start not in self.vertices:
            return []

        paths = []
        for node_tuple in self.vertices[start].neighbors:
            node = node_tuple[0]
            if node not in path:
                new_paths = self.get_paths(node, end, path)
                for p in new_paths:
                    paths.append(p)

        return paths




# https://www.youtube.com/watch?v=uFaZY1dVnGs
# graphs what are they?

class Graph:
 
    def __init__(self, V, E):
        self.E = set(frozenset((u, v)) for u, v in E)
        self._nbrs = {}
        for v in V:
            self.add_vertex(v) 
        for u, v in self.E:
            self.add_edge(u, v)

    def add_vertex(self, v):
        if v not in self._nbrs:
            self._nbrs[v] = set()

    def add_edge(self, u, v):
        self.add_vertex(u)
        self.add_vertex(v)
        self.E.add(frozenset([u, v]))
        self._nbrs[u].add(v)
        self._nbrs[v].add(u)

    def deg(self, v):
        return len(self._nbrs[v])

    def neighbors(self, v):
        return iter(self._nbrs[v])
 
    def remove_edge(self, u, v):
        e = frozenset([u, v])
        if e in self.E:
            self.E.remove(e)
            self._nbrs[u].remove(v)
            self._nbrs[v].remove(u)

    def remove_vertex(self, u):
        todelete = list(self.neighbors(u))
        for v in todelete:
            self.remove_edge(u, v)
        del self._nbrs[u]

    # number of edges
    @property
    def m(self):
        return len(self.E)

    # number of vertices
    @property
    def n(self):
        return len(self._nbrs)

if __name__ == '__main__':

    G = Graph([1, 2, 3], { (1, 2), (2, 3) })
    assert(G.deg(1) == 1)
    assert(G.deg(2) == 2)
    assert(G.deg(3) == 1)
    assert(set(G.neighbors(2)) == {1,3})
    assert(G.n == 3 and G.m == 2)

    G.remove_edge(1, 2)
    assert(G.n == 3 and G.m == 1)
    
    G.remove_edge(1, 3) # invalid edge removal
    assert(G.n == 3 and G.m == 1)

    G.add_edge(1, 2)
    assert(G.n == 3 and G.m == 2)

    G.remove_vertex(2)
    assert(G.n == 2 and G.m == 0)

    print("ok")



