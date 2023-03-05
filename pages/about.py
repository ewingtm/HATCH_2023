#################################################
# IND[i]GOME: ABOUT
#################################################

import streamlit as st
from streamlit_multipage import MultiPage

def main():

    #st.markdown("<h1 style='text-align: center; color: white;'>About</h1>", unsafe_allow_html=True)
    st.title("About")

    #Spacing between Title and Login
    for row in range(0,3):
        st.text("")

    st.text("HATCH 2023: Security One")
    st.text("Phillip Ewing, Jr., Tiffany Ewing, Ethan Jones")


    return 1

main()