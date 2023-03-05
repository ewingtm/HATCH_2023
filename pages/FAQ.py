#################################################
# IND[i]GOME: FAQ
#################################################

import streamlit as st
from streamlit_multipage import MultiPage

def main():

    st.title("FAQ")

    #Spacing between Title and Login
    for row in range(0,3):
        st.text("")

    #Question 1
    st.text("What is homomorphic encryption?")
    st.text("Is a recent development in cryptography that allows computations to be applied to ")
    st.text("ciphertext and persists to eventual decryption.")

    #Spacing between Title and Login
    for row in range(0,3):
        st.text("")

    #Question 2
    st.text("Who discovered homographic encryption?")
    st.text("Craig Gentry proposed the first implentation in 2009. Work on lattice schemes")
    st.text("has been continuous for the past few decades, but has picked up due to NISTQC.")

    return 1

main()