
#################################################
# IND[i]GOME: generate_keys
#################################################

import pickle
from pathlib import Path

import streamlit_authenticator as stauth

companies = ["Company A", "Company B"]
employees = ["Phillip Ewing", "Ethan Jones"]
passwords = ["Ewing", "Jones"]

hashed_employee_ids = stauth.Hasher(passwords).generate()

file_path = Path(__file__).parent / "hashed_pw.pkl"
with file_path.open("wb") as file:
    pickle.dump(hashed_employee_ids, file)
