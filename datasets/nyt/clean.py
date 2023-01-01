import pandas as pd

f = pd.read_csv("nyt.csv", usecols=[1, 7, 8, 9, 10, 11, 12, 16, 17, 18])
#f = pd.read_csv("no-keywords.csv")
#f["byline.original"] = f["byline.original"].apply(lambda x: x.replace("By", ""))
#f["byline.original"] = f["byline.original"].apply(lambda x: x.replace("and", ""))
f.rename(columns={"byline.original": "by", "headline.main": "main_headline", "headline.print_headline": "print_headline"}, inplace=True)

f.sort_values('pub_date')
#f.drop('print_section', inplace=True, axis=1)
f.to_csv('clean-sorted.csv')
