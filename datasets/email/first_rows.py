import pandas as pd


df = pd.read_csv("clean-sorted.csv", dtype=str)
#f.replace('\n', ' ', inplace = True)
#df = f.applymap(lambda x: ' '.join(str(x).split()) if len(str(x)) > 0 else x)
#f["subject"] = f["subject"].apply(lambda x: str(x).replace('\n ', ' '))
#f["to"] = f["to"].apply(lambda x: str(x).replace("  ", " "))

#f = pd.read_csv("no-keywords.csv")
#f.drop('print_section', inplace=True, axis=1)
df.head(500).to_csv('train.csv')
