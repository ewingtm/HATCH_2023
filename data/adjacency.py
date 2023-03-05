import matplotlib.pyplot as plt
import networkx as nx
import pandas as pd

FPATH = "./data/adj_sample.csv"

# Create an empty graph
G = nx.Graph()

# Add nodes to the graph
genomes = pd.read_csv(FPATH, header=None, index_col=None)

print(genomes)

G.add_nodes_from(list(range(len(genomes))))

epsilon = 5

# Add edges to the graph
for i, node_adjacencies in enumerate(genomes.iterrows()):
    for j in range(i, len(node_adjacencies[1])):
        if (node_adjacencies[1][j] <= epsilon) and (i != j):
            print(node_adjacencies[1][j])
            G.add_edge(i, j)

# Draw the graph
pos = nx.spring_layout(G)
nx.draw_networkx_nodes(G, pos, node_color='blue', node_size=500)
nx.draw_networkx_edges(G, pos, edge_color='red')
nx.draw_networkx_labels(G, pos, font_color='white')
plt.axis('off')

plt.savefig("adjacency-graph.png", dpi=300)
plt.show() 