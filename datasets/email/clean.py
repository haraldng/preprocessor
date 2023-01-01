import pandas as pd

def remove_multi_spaces(x):
	x_str = str(x)
	if x_str == "nan":
		return x	
	if len(x_str) > 0:
		x_str = ' '.join(str(x).split())
	return x_str

def preprocess(x):
	x_str = str(x)
	if x_str == "nan":
		return x
	if len(x_str) > 0:
		x_str = x_str.replace(",", " ").replace("[", "").replace("]", "").replace("'", "")
	return x_str


f = pd.read_csv("emails.csv", usecols=range(0, 12), dtype=str)
f["From"] = f["From"].apply(preprocess)
f["To"] = f["To"].apply(preprocess)
df = f.applymap(remove_multi_spaces)
df.rename(
	columns={"Message-ID": "message_id", "X-From": "x_from", "X-To": "x_to", "X-cc": "x_cc", "X-bcc": "x_bcc", "X-Folder": "x_folder", "X-Origin": "x_origin", "X-FileName": "x_filename", "Date": "date", "From": "from", "To": "to", "Subject": "subject"}, 
	inplace=True
	)

df.sort_values('date', inplace=True)
df.to_csv('clean-sorted.csv')
df.head(500).to_csv('train.csv')
