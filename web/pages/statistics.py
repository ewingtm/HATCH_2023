#################################################
# IND[i]GOME: STATISTICS
#################################################


import streamlit as st
from streamlit_multipage import MultiPage
from PIL import Image

def main():

    st.title("Statistics")

    image2 = Image.open('pages/similarity-matrix.png')

    st.image(image2)

    return 1

main()