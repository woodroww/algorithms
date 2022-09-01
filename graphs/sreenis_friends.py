#!/usr/bin/env python
# coding: utf-8

# https://www.youtube.com/watch?v=PtbnC5qaXpE&t=417s
# https://pyvis.readthedocs.io/en/latest/
# pip install pyvis
# NOT pyviz DON'T DO IT

import sys
print(sys.version)
import os
from IPython.core.display import display, HTML
from pyvis.network import Network
import webbrowser
import pandas as pd
import numpy as np

chrome_path = 'open -a /Applications/Google\ Chrome.app %s'
base_url = "/Users/matt/Documents/Programming/graphs/html"

def saveShow(net, file_name):
    net.show(os.path.join(base_url, file_name))
    webbrowser.get(chrome_path).open(os.path.join(base_url, file_name))


net = Network()
net.add_node(1, label="Sreeni") # node id = 1 and label Sreeni
net.add_node(2) # node id and label = 2 id must be unique
saveShow(net, "nodes.html")

net1 = Network()
nodes = ["a", "b", "c", "d"]
labels = ["Sreeni", "Mike", "Imran", "Syed"]
net1.add_nodes(nodes, label=labels)
saveShow(net1, "nodes1.html")

#same but with colors
net2 = Network()
nodes = ["a", "b", "c", "d"]
labels = ["Sreeni", "Mike", "Imran", "Syed"]
colors = ["#FF0000", "#00CC00", "#0000CC", "#FFCC00"]
net2.add_nodes(nodes, label=labels, color=colors)
saveShow(net2, "nodes2.html")

# add some edges/relationships
net2.add_edges([("a", "b"), ("a", "d")])
saveShow(net2, "edges1.html")

# add weight with the edges
net3 = Network()
nodes = ["a", "b", "c", "d"]
labels = ["Sreeni", "Mike", "Imran", "Syed"]
colors = ["#FF0000", "#00CC00", "#0000CC", "#FFCC00"]
net3.add_nodes(nodes, label=labels, color=colors)
# just add a weight parameter to the tuple
net3.add_edges([("a", "b", 1), ("a", "d", 5)])
saveShow(net3, "edges2.html")

# the physics like behavior
net3.repulsion(node_distance=90, spring_length=200)
saveShow(net3, "edges3.html")

my_friends = [
    ("Sreeni", "Mike"),
    ("Sreeni", "Imran"),
    ("Sreeni", "Mustafa"),
    ("Sreeni", "Syed"),
    ("Sreeni", "Satya"),
    ("Sreeni", "Vamsi"),
    ("Imran", "Mustafa"),
    ("Imran", "Mike"),
    ("Mustafa", "Satya"),
    ("Mustafa", "Vamsi"),
    ("Syed", "Mike"),
    ("Syed", "Satya"),
]

friends = pd.DataFrame(my_friends, columns=["friend1", "friend2"])
friends

# the easiest way to get the unique friends from each column
# then find the union of both
a = set([1, 1, 1, 2, 3, 4, 4, 5, 5, 5])
b = set([4, 4, 5, 6, 7, 7, 8])
print(a)
print(b)
print("Union a.union(b)")
print(a.union(b))

# unique people
people = list(set(friends.friend1).union(set(friends.friend2)))
print(people)

friends_net = Network()
friends_net.add_nodes(people)

my_friends_list = friends.values.tolist()
my_friends_list

friends_net.add_edges(my_friends_list)
saveShow(friends_net, "my_friends_map.html")


## Game of Thrones example character network

got_net = Network(
        height="750px",
        width="100%",
        bgcolor="#222222",
        font_color="white")

# set a physics layout call barnes_hut
got_net.barnes_hut()

got_data = pd.read_csv("http://www.macalester.edu/~abeverid/data/stormofswords.csv")
got_data.head()

sources = got_data["Source"]
targets = got_data["Target"]
weights = got_data["Weight"]

edge_data = zip(sources, targets, weights)
for e in edge_data:
    src = e[0]
    dst = e[1]
    w = e[2]
    got_net.add_node(src, src, title=src)
    got_net.add_node(dst, dst, title=dst)
    got_net.add_edge(src, dst, value=w)

# add in hover text
neighbor_map = got_net.get_adj_list() # dictionary mapping of node id to list of node ids
for node in got_net.nodes:
    node["title"] += " Neighbors:<br>" + "<br>".join(neighbor_map[node["id"]])
    node["value"] = len(neighbor_map[node["id"]])

saveShow(got_net, "gameofthrones.html")










