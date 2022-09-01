# https://github.com/codebasics/data-structures-algorithms-python/blob/master/data_structures/10_Graph/graph.py
# https://www.youtube.com/watch?v=j0IYCyBdzfA&t=714s

# https://pyvis.readthedocs.io/en/latest/

import os
from IPython.core.display import display, HTML
from pyvis.network import Network
import webbrowser

chrome_path = 'open -a /Applications/Google\ Chrome.app %s'
base_url = "/Users/matt/Documents/Programming/graphs/html/codebasics/"

def saveShow(net, file_name):
    net.show(os.path.join(base_url, file_name))
    webbrowser.get(chrome_path).open(os.path.join(base_url, file_name))

class Graph:
    def __init__(self, edges):
        self.edges = edges
        self.graph_dict = {}
        self.unique_nodes = []
        for start, end in edges:
            if start not in self.unique_nodes:
                self.unique_nodes.append(start)
            if end not in self.unique_nodes:
                self.unique_nodes.append(end)

            if start in self.graph_dict:
                self.graph_dict[start].append(end)
            else:
                self.graph_dict[start] = [end]
        print("Graph Dict:", self.graph_dict)

    def visualize(self, file_name):
        net = Network()
        unique_node_id = 0
        label_id_dict = {}
        for node in self.unique_nodes:
            label_id_dict[node] = unique_node_id
            net.add_node(unique_node_id, label=node)
            unique_node_id += 1
        for key in self.graph_dict:
            for destination in self.graph_dict[key]:
                net.add_edge(label_id_dict[key], label_id_dict[destination])
        saveShow(net, file_name)
        return net

    def get_paths(self, start, end, path=[]):
        path = path + [start]
        if start == end:
            return [path]
        if start not in self.graph_dict:
            return []
        paths = []
        for node in self.graph_dict[start]:
            if node not in path:
                new_paths = self.get_paths(node, end, path)
                for p in new_paths:
                    paths.append(p)
        return paths

    # only returns one path there could be two or more paths of the same length
    def get_shortest_path(self, start, end, path=[]):
        path = path + [start]
        if start == end:
            return path
        if start not in self.graph_dict:
            return None
        shortest_path = None
        for node in self.graph_dict[start]:
            if node not in path:
                sp = self.get_shortest_path(node, end, path)
                if sp:
                    if shortest_path is None or len(sp) < len(shortest_path):
                        shortest_path = sp
        return shortest_path

def display_paths(start, end, route_graph):
    print(f"All paths between: {start} and {end}:")
    paths = route_graph.get_paths(start, end)
    for path_list in paths:
        print(path_list)
    print(f"Shortest path between {start} and {end}:")
    paths = route_graph.get_shortest_path(start, end)
    print(paths)



if __name__ == '__main__':

    routes = [
        ("Mumbai","Pune"),
        ("Mumbai", "Surat"),
        ("Surat", "Bangaluru"),
        ("Pune","Hyderabad"),
        ("Pune","Mysuru"),
        ("Hyderabad","Bangaluru"),
        ("Hyderabad", "Chennai"),
        ("Mysuru", "Bangaluru"),
        ("Chennai", "Bangaluru")
    ]

    routes = [
        ("Mumbai", "Paris"),
        ("Mumbai", "Dubai"),
        ("Paris", "Dubai"),
        ("Paris", "New York"),
        ("Dubai", "New York"),
        ("New York", "Toronto"),
    ]

    route_graph = Graph(routes)
    route_graph.graph_dict

    visNet = route_graph.visualize("map.html")

    start = "Mumbai"
    end = "New York"
    display_paths(start, end, route_graph)

    start = "Dubai"
    end = "New York"
    display_paths(start, end, route_graph)

    start = "Mumbai"
    end = "Toronto"
    display_paths(start, end, route_graph)
