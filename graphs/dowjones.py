import pandas as pd
import networkx as nx
import matplotlib.pyplot as plt
import yfinance as yf
import os
import webbrowser

dow_jone_feather = "/Users/matt/ml_data/dow_jones_inst_holders.feather"

def getTickers():
    tickers = pd.read_html("https://en.m.wikipedia.org/wiki/Dow_Jones_Industrial_Average")[1]
    tickers = tickers["Symbol"].to_list()
    tickers_list = []
    for ticker in tickers:
        tickers_list.append(yf.Ticker(ticker))
    return tickers_list

# now for everything
def downloadHolders(tickers):
    frames = []
    for ticker in tickers:
        frame = ticker.institutional_holders
        print(f"{ticker.ticker}")
        frame["company"] = ticker.ticker
        frames.append(frame)
    return frames

def downloadSaveDowJones():
    tickers = getTickers()
    frames = downloadHolders(tickers)
    all_tickers_df = pd.concat(frames)
    all_tickers_df = all_tickers_df.reset_index()
    all_tickers_df.to_feather(dow_jone_feather)

if not os.path.isfile(dow_jone_feather):
    downloadSaveDowJones()
else:
    all_tickers_df = pd.read_feather(dow_jone_feather)

G = nx.from_pandas_edgelist(all_tickers_df, source="Holder", target="company")
#nx.draw(G, with_labels=True)
#plt.show()
companies = all_tickers_df["company"].unique()

for i in list(G.nodes):
    if i in companies:
        G.nodes[i]["color"] = '#83a598'
    else:
        G.nodes[i]["color"] = '#98971a'

chrome_path = 'open -a /Applications/Google\ Chrome.app %s'
base_url = "/Users/matt/Documents/Programming/graphs/html"
def saveShow(net, file_name):
    net.show(os.path.join(base_url, file_name))
    webbrowser.get(chrome_path).open(os.path.join(base_url, file_name))

def makePyvisGraph(networkx_graph):
    # https://gist.github.com/quadrismegistus/92a7fba479fc1e7d2661909d19d4ae7e
    from pyvis import network as net
    # make a pyvis network
    pyvis_graph = net.Network(height='750px', width='100%', bgcolor='#32302f', font_color='#689d6a')
    # for each node and its attributes in the networkx graph
    for node, node_attrs in networkx_graph.nodes(data=True):
        pyvis_graph.add_node(str(node),**node_attrs)
    # for each edge and its attributes in the networkx graph
    for source,target,edge_attrs in networkx_graph.edges(data=True):
        # if value/width not specified directly, and weight is specified, set 'value' to 'weight'
        if not 'value' in edge_attrs and not 'width' in edge_attrs and 'weight' in edge_attrs:
            # place at key 'value' the weight of the edge
            edge_attrs['value']=edge_attrs['weight']
        # add the edge
        pyvis_graph.add_edge(str(source),str(target),**edge_attrs)
    # return and also save
    return pyvis_graph


options = """
var options = {
  "physics": {
    "hierarchicalRepulsion": {
      "centralGravity": 0
    },
    "maxVelocity": 21,
    "minVelocity": 0.75,
    "solver": "hierarchicalRepulsion"
  }
}
"""
# pyvis_graph.set_options(options)
pyvis_graph = makePyvisGraph(G)
pyvis_graph.show_buttons(filter_=['physics'])
saveShow(pyvis_graph, "companies.html")



company_count_per_holder = all_tickers_df[["Holder", "company"]].groupby(["Holder"]).count().sort_values("company")
company_count_per_holder.index[0]
company_count_per_holder.iloc[0]

all_tickers_df[all_tickers_df["Holder"] == company_count_per_holder.index[0]]

black_rock = all_tickers_df[all_tickers_df["Holder"] == company_count_per_holder.index[-1]]
print(f"{black_rock['Value'].sum():,}")

value_sums = all_tickers_df[["Holder", "Value"]].groupby(["Holder"]).sum().sort_values("Value")
value_sums['readable_value'] = value_sums['Value'].apply(lambda x: "{:,}".format(x))


jammy.unstack()
jammy.pivot(columns=["Value"])

filt = (all_tickers_df["Holder"] == "Wellington Management Group, LLP")
all_tickers_df[filt][["Holder", "company", "Value"]]



edges = pd.DataFrame({
        "source": [0, 1, 2],
        "target": [2, 2, 3],
        "weight": [3, 4, 5],
        "color": ["red", "blue", "blue"], })
example = nx.from_pandas_edgelist(edges, source="source", target="target", edge_attr=True)
example.nodes
example.nodes(data=True)
example.edges(data=True)


