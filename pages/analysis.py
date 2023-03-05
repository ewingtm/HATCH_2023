#################################################
# IND[i]GOME: ANALYSIS
#################################################

import streamlit as st
from streamlit_multipage import MultiPage
from PIL import Image

def main():

    st.title("Analysis")

    image = Image.open('pages/hamming.png')

    st.image(image, "Toy example of computing hamming distance between two encrypted nucleotide sequences")

    return 1

main()