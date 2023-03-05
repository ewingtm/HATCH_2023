
#################################################
# WELCOME TO IND[i]GOME
# HATCH 2023
# CHALLENGE #3
#
# Team Members:
#   Phillip Ewing, Jr.
#   Ethan Jones
#   Tiffany Ewing
#################################################

import pickle
from pathlib import Path

import sys
import os
import matplotlib
import streamlit as st
import streamlit_authenticator as stauth
from streamlit_multipage import MultiPage
from streamlit.source_util import get_pages

#Pages
import pages.statistics as statistics
import pages.analysis as analysis
import pages.admin as admin
import pages.FAQ as FAQ

#---------------------------------------------
def main():

    st.set_page_config(layout="wide", page_title="IND[i]GOME")

    #Sidebar magic
    st.sidebar.header("MENU")
    st.markdown("""<style>.css-1iyw2u1 {display: none;}</style>""", unsafe_allow_html=True)

    #Specify font style
    with open( "style.css" ) as css:
        st.markdown( f'<style>{css.read()}</style>' , unsafe_allow_html= True)

    #Title
    st.markdown("<h1 style='text-align: center; color: white;'>IND[i]GOME</h1>", unsafe_allow_html=True)

    #Spacing between Title and Login
    for row in range(0,3):
        st.text("")

#### User Authentication
    companies = ["Company A", "Company B"]
    employees = ["Phillip Ewing", "Ethan Jones"]
    error_count = 0

    file_path = Path(__file__).parent / "hashed_pw.pkl"
    with file_path.open("rb") as file:
        hashed_employee_ids = pickle.load(file)

    authenticator = stauth.Authenticate(companies, employees, hashed_employee_ids, "first", "second", cookie_expiry_days=0)
    company, authentication_status, employee = authenticator.login("Login", "main")

    if authentication_status == None:
        st.warning("Please Enter your username and password")

    elif authentication_status == False:
        st.error("Invalid Login")
        error_count += 1 
        if error_count >= 3:
            return 0

    elif authentication_status:
        #Title
        st.markdown("<h1 style='text-align: center; color: white;'>WELCOME</h1>", unsafe_allow_html=True)

    return 1
#------------------------------------------------
# def diag_callback():
#     st.session_state.button_clicked = True
#     diagnostics.main()

# #------------------------------------------------
# def stat_callback():
#     st.session_state.button_clicked = True
#     stats.main()

#------------------------------------------------
# def View():

#     c1, c2, c3, c4, c5 = st.columns(5)
#     with c2:
#         diag = st.button("Diagnostics", on_click=diag_callback())

#     with c4:
#         stat = st.button("Statistics", on_click=stat_callback())

#     return 1
#---------------------------------------------
main()
