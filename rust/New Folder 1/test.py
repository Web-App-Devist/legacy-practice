import re

import pandas as pd
import requests
from bs4 import BeautifulSoup

import datetime


def get_current_year():
    """Gets the current year."""

    now = datetime.datetime.now()
    return now.year


def get_current_month_number():
    """Gets the current month number (1-12)."""

    now = datetime.datetime.now()
    return now.month


url = "https://aepos.ap.gov.in/ShopAbstractM.jsp"
headers = {
    "content-type": "application/x-www-form-urlencoded; charset=UTF-8",
}
data = {
    "month": get_current_month_number(),
    "year": get_current_year(),
    "shop_no": "0184139",
    "type": "0",
}

response = requests.post(url, headers=headers, data=data)


# HTMLFileToBeOpened = open("./index.html", "r")

# contents = HTMLFileToBeOpened.read()

beautifulSoupText = BeautifulSoup(response.text, "lxml")

sales = []

keys = [
    "Date",
    "Rice(kg)",
    # "Dal(packets)",
    "Sugar(packets)",
    # "Atta(packets)",
    "Ammount(RS)",
]


for row in beautifulSoupText.find_all("tr"):
    # [<td>17-02-2024</td>, <td align="right">10.000</td>, <td align="right">1.000</td>, <td align="right">0.000</td>, <td align="right">17.00</td>]
    tds = row.find_all(
        lambda tag: tag.name == "td"
        and (
            re.search(r"\d{1,2}-\d{1,2}-\d{4}", tag.text) or tag.get("align") == "right"
        )
    )

    # {'Date': '17-02-2024', 'Rice(kg)': 10, 'Sugar(packets)': 1, 'Atta(packets)': 0, 'Ammount(RS)': 17}
    row_data = {
        keys[i]: (tds[i].text if i == 0 else int(float(tds[i].text)))
        for i in range(min(len(tds), len(keys)))
    }

    # {'Date': '17-02-2024', 'Rice(kg)': 10, 'Sugar(packets)': 1, 'Atta(packets)': 0, 'Ammount(RS)': 17, 'Type': 'Normal'}
    if (
        row_data and len(tds) > 0 and float(tds[2].text) > 1.0
    ):  # the index is 2 because sugar is present at this index
        row_data = {**row_data, **{"Type": "YAP"}}
    elif row_data and len(tds) > 0 and float(tds[2].text) == 1.0:
        if row_data:
            row_data = {**row_data, **{"Type": "Normal"}}

    if row_data:
        sales.append(row_data)

# print(len(sales), sales[4])
sales_df = pd.DataFrame(sales)

# Assuming your DataFrame is named sales_df
# Convert the 'Date' column to datetime format
sales_df["Date"] = pd.to_datetime(sales_df["Date"], format="%d-%m-%Y")
# sales_df["Total_Cards"] = sales_df['Rice(kg)'] + sales_df['Sugar(packets)'] + sales_df['Atta(packets)']
# in the below step, i want to create two more columns which will store the number of "YAP" type and "Normal" type per "Date"
sales_df["YAP"] = sales_df["Type"].apply(lambda x: 1 if x == "YAP" else 0)
sales_df["Normal"] = sales_df["Type"].apply(lambda x: 1 if x == "Normal" else 0)

# Group by 'Date' and calculate the sum for each column
sum_by_date = (
    sales_df.groupby(sales_df["Date"].dt.strftime("%d-%m-%Y"))
    .agg(
        {
            "Rice(kg)": "sum",
            # 'Dal(packets)': 'sum',
            "Sugar(packets)": "sum",
            # "Atta(packets)": 'sum',
            "Ammount(RS)": "sum",
            "YAP": "sum",
            "Normal": "sum",
        }
    )
    .reset_index()
)

# Add a row for the total sum
total_sum = pd.DataFrame(
    sales_df[["Rice(kg)", "Sugar(packets)", "Ammount(RS)", "YAP", "Normal"]].sum()
).T
total_sum["Date"] = "Total"
total_sum = total_sum[
    ["Date", "Rice(kg)", "Sugar(packets)", "Ammount(RS)", "YAP", "Normal"]
]
sum_by_date = pd.concat([sum_by_date, total_sum], ignore_index=True)

# Print the result
print(sum_by_date.to_string(index=False))
# print(sales_df.to_string(index=False))
