import numpy as np
import matplotlib.pyplot as plt
import os
from vcf2vec import VcfObject

FPATH = "./data/examples/"

FILES = os.listdir(FPATH)

sample_vectors = {}
names = []
for filename in FILES:
    name = filename.split('.')[0]
    vcf = VcfObject(os.path.join(FPATH, filename))
    names.append(name)
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

fig, ax = plt.subplots(1, 1)
im = ax.imshow(similarity_matrix)

cbar = ax.figure.colorbar(im, ax=ax)

#ax.set_xticks(np.arange(len(similarity_matrix)))
ax.set_yticks(np.arange(len(similarity_matrix)))
#ax.set_xticklabels(names)
ax.set_yticklabels(names)

ax.xaxis.tick_top()
ax.xaxis.set_label_position('top')

ax.set_title("Genome Similarity Matrix", y=-0.2)

# Rotate the tick labels and set their alignment
plt.setp(ax.get_xticklabels(), rotation=90, ha="left",
         rotation_mode="anchor")

ax.autoscale(enable=True)

plt.tight_layout()
plt.savefig("./img/similarity-matrix.png", dpi=300)

# Show the plot
plt.show()