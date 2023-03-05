import pandas as pd
import re
from enum import Enum

class VcfObject(object):

    NUCLEOTIDE = {
        'A': 1,
        'T': 2,
        'C': 3,
        'G': 4
    }

    def __init__(self, filepath: str):
        self.data = pd.read_csv(filepath, header=0)
        self.vector = []
        self.to_vector()

    def to_vector(self):
        for _, row in self.data.iterrows():
            print(row)
            idx_A, idx_B = (int(n) for n in re.split('[/|]', row[self.data.columns[-1]]))
            gt_A = self.NUCLEOTIDE[row["ALT"][idx_A - 1][0] if idx_A != 0 else row["REF"][0]]
            gt_B = self.NUCLEOTIDE[row["ALT"][idx_B - 1][0] if idx_B != 0 else row["REF"][0]]

            # NOTE: This is just an arbitrary assumption about the nucleotide's
            # distance from the reference genome value; to be revisited in
            # future iterations!
            self.vector.append(max(gt_A, gt_B))

# if __name__ == "__main__":
#     FPATH = "./data/examples/HG00096_10K.csv"
#     vcf = VcfObject(FPATH)
#     print(vcf.vector)