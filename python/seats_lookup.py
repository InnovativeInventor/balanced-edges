import pandas as pd
import typer
import us

def main(filename: str, chamber: str = ""):
    state = filename.split("/")[-1].split("-")[0].lower()

    apportionment = pd.read_excel("config-data/apportionment-2020-table01.xlsx", skiprows=3)
    state_seats = pd.read_csv("config-data/house-v-sen.csv")

    state_row = state_seats[state_seats["Name"] == us.states.lookup(state).name].iloc[0]
    state_house = state_row["House"]
    state_senate = state_row["Senate"]

    seats = None
    for c, row in apportionment.iterrows():
        possible_state = us.states.lookup(row["STATE"])
        if possible_state and str(possible_state.abbr).strip().lower() == state:
            seats = row["NUMBER OF APPORTIONED REPRESENTATIVES BASED ON \n2020 CENSUS2"]
            break
    if chamber == "congress":
        print(int(seats))
    elif chamber == "senate":
        print(int(state_senate))
    elif chamber == "house":
        print(int(state_house))
    else:
        print(int(seats), int(state_senate), int(state_house))

if __name__ == "__main__":
    typer.run(main)
