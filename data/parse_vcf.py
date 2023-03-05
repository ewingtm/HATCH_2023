import pandas as pd
import os

if __name__ == "__main__":

    F_IN = "./data/ALL.chr14.shapeit2_integrated_snvindels_v2a_27022019.GRCh38.phased.vcf"

    for i in range(9, 109):
        sel_columns = list(range(9)) + [i]
        df = pd.read_table(F_IN, skiprows=19, nrows=10000, low_memory=False, usecols=sel_columns)
        filename = df.columns[-1] + "_10K.csv"
        df.to_csv(os.path.join("./data/examples/", filename), mode='a', index=False)

        print("Writing...", filename)
    
    print("Done.")
