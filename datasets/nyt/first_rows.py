import pandas as pd

f = pd.read_csv("clean-sorted.csv")
#f = pd.read_csv("no-keywords.csv")
#f.drop('print_section', inplace=True, axis=1)
f.head(500).to_csv('train.csv')
