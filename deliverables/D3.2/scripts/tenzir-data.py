#!/usr/bin/env python3
import requests
from datetime import datetime

OWNER = "tenzir"
REPO = "docs"
SINCE = "2025-04-01T00:00:00Z"
UNTIL = "2026-06-30T23:59:59Z"

def latex_escape(text):
    replacements = {
        "\\": r"\textbackslash{}",
        "&": r"\&",
        "%": r"\%",
        "$": r"\$",
        "#": r"\#",
        "_": r"\_",
        "{": r"\{",
        "}": r"\}",
        "~": r"\textasciitilde{}",
        "^": r"\textasciicircum{}",
    }
    return "".join(replacements.get(c, c) for c in text)

commits = []
page = 1

while True:
    url = f"https://api.github.com/repos/{OWNER}/{REPO}/commits"
    params = {
        "sha": "main",
        "since": SINCE,
        "until": UNTIL,
        "per_page": 100,
        "page": page,
    }

    response = requests.get(url, params=params)
    response.raise_for_status()

    data = response.json()
    if not data:
        break

    commits.extend(data)
    page += 1

# Oldest first
commits.sort(key=lambda c: c["commit"]["committer"]["date"])

print(r"\begingroup")
print(r"\scriptsize")
print(r"\renewcommand{\arraystretch}{0.9}")
print(r"\setlength{\tabcolsep}{3pt}")
print(r"\begin{center}")
print(r"\begin{tabular}{p{2.0cm}p{2.5cm}p{8.3cm}p{1.1cm}}")
print(r"\hline")
print(r"Date & Repository & Title & Commit \\")
print(r"\hline")

for index, commit in enumerate(commits, start=1):
    date = commit["commit"]["committer"]["date"][:10]
    title = commit["commit"]["message"].splitlines()[0]
    sha = commit["sha"][:7]
    url = commit["html_url"]

    print(
        f"{date} & {REPO} & {latex_escape(title)} & "
        rf"\href{{{url}}}{{\#{index}}} \\"
    )

print(r"\hline")
print(r"\end{tabular}")
print(r"\end{center}")
print(r"\endgroup")
