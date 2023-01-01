import pandas as pd

def remove_multi_spaces(x):
	x_str = str(x)
	if len(x_str) > 0:
		x_str = ' '.join(str(x).split())
	return x_str


f = pd.read_csv("emails.csv", usecols=range(0, 12))
f["From"] = f["From"].apply(lambda x: x.replace(",", " "))
f["From"] = f["From"].apply(lambda x: x.replace("[", ""))
f["From"] = f["From"].apply(lambda x: x.replace("]", ""))
f["From"] = f["From"].apply(lambda x: x.replace("'", ""))

f["To"] = f["To"].apply(lambda x: str(x).replace(",", " "))
f["To"] = f["To"].apply(lambda x: str(x).replace("[", ""))
f["To"] = f["To"].apply(lambda x: str(x).replace("]", ""))
f["To"] = f["To"].apply(lambda x: x.replace("'", ""))
f["To"] = f["To"].apply(lambda x: x.replace("  ", " "))

df = f.applymap(remove_multi_spaces)

df.rename(
	columns={"Message-ID": "message_id", "X-From": "x_from", "X-To": "x_to", "X-cc": "x_cc", "X-bcc": "x_bcc", "X-Folder": "x_folder", "X-Origin": "x_origin", "X-FileName": "x_filename", "Date": "date", "From": "from", "To": "to", "Subject": "subject"}, 
	inplace=True
	)

#f = pd.read_csv("no-keywords.csv")
df.sort_values('date')
#f.drop('print_section', inplace=True, axis=1)
df.to_csv('clean-sorted.csv')
