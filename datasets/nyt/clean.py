import pandas as pd

def remove_multi_spaces(x):
	x_str = str(x)
	if x_str == "nan":
		return x	
	if len(x_str) > 0:
		x_str = ' '.join(str(x).split())
	return x_str

def remove_by(x):
	x_str = str(x)
	if x_str == "nan":
		return x	
	x_str = x_str.replace("By", "")
	return x_str

f = pd.read_csv("nyt.csv", usecols=[1, 8, 9, 10, 11, 12, 16, 17, 18])
#f = pd.read_csv("no-keywords.csv")
#f["byline.original"] = f["byline.original"].apply(lambda x: x.replace("By", ""))
#f["byline.original"] = f["byline.original"].apply(lambda x: x.replace("and", ""))
f.rename(columns={"byline.original": "by", "headline.main": "main_headline", "headline.print_headline": "print_headline"}, inplace=True)
f["main_headline"] = f["main_headline"].apply(remove_multi_spaces)
f["print_headline"] = f["print_headline"].apply(remove_multi_spaces)
f["by"] = f["by"].apply(remove_by)
f["by"] = f["by"].apply(remove_multi_spaces)

f.sort_values('pub_date', inplace=True)
f.drop(f.tail(2).index,inplace=True) # drop not properly formatted last 2 rows
#f.drop('print_section', inplace=True, axis=1)
f.to_csv('clean-sorted.csv')
f.head(500).to_csv('train.csv')