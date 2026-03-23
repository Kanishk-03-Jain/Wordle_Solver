import pandas as pd

answers = pd.read_csv("src/answers1.csv")
guesses = pd.read_csv("src/words.csv")

final = answers.merge(guesses, on="word", how="left")
final["count"] = final["count"].fillna(1000).astype(int)
final.to_csv("src/final.csv", index=False)
