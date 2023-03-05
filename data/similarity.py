import numpy as np
import matplotlib.pyplot as plt
import os
from vcf2vec import VcfObject
import itertools

FPATH = "./data/examples/"

FILES = os.listdir(FPATH)

sample_vectors = {}

for filename in FILES:
    name = filename.split('.')[0]
    vcf = VcfObject(os.path.join(FPATH, filename))
    sample_vectors[name] = vcf.vector

similarity_lists = []

for ref_key in sample_vectors.keys():
    similarity_list = []
    for comp_key in sample_vectors.keys():
        comp_values = list(zip(sample_vectors[ref_key], sample_vectors[comp_key]))
        similarity = sum([abs(value[0] - value[1]) for value in comp_values])
        similarity_list.append(similarity)
    similarity_lists.append(similarity_list)

print(similarity_lists)

similarity_matrix = similarity_lists

fig, ax = plt.subplots()
im = ax.imshow(similarity_matrix)

cbar = ax.figure.colorbar(im, ax=ax)

ax.set_xticks(np.arange(len(similarity_matrix)))
ax.set_yticks(np.arange(len(similarity_matrix)))
ax.set_xticklabels(np.arange(1, len(similarity_matrix)+1))
ax.set_yticklabels(np.arange(1, len(similarity_matrix)+1))
ax.set_title("Genome Similarity Matrix")

# Rotate the tick labels and set their alignment
plt.setp(ax.get_xticklabels(), rotation=45, ha="right",
         rotation_mode="anchor")

plt.savefig("similarity-matrix.png", dpi=300)

# Show the plot
plt.show()
