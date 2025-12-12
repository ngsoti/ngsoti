#!/bin/bash
# Usage: ./getmd.sh https://your-hedgedoc/note-id
DOCUMENT="Deliverable_D2_1"


URL="$1"
# Remove trailing slash AND trailing '#' or query
URL=`echo $URL | cut -d '?' -f1`
URL=`echo $URL | cut -d '#' -f1`
URL="${URL%/}"
DL_URL="${URL}/download"
CONTENT=$(curl -sL "$DL_URL")

# Create Picts output if needed
[ -d "./picts" ] || mkdir -p "./picts"

# Cleanup old pdf
if [ -f ./$DOCUMENT.pdf  ]; then
    echo "Cleaning $DOCUMENT.pdf"
    rm "$DOCUMENT.pdf"
    fi

if echo "$CONTENT" | grep -qi "<html"; then
    echo "Error: The note is not public or the server returned HTML."
        exit 1
        fi
        echo "$CONTENT" > $DOCUMENT-ORI.md
        echo "Downloaded as $DOCUMENT.md"

echo -n "Retrieving pics..."
grep -Pzo '(?s)^---\n(.*?)\n---\n' $DOCUMENT-ORI.md | strings > header.tmp
echo "---" > sep.tmp
pandoc $DOCUMENT-ORI.md -o ./body.tmp --extract-media=./picts --to=gfm
cat sep.tmp header.tmp sep.tmp body.tmp > $DOCUMENT.md
rm header.tmp body.tmp sep.tmp
echo -n "Generating..."
pandoc $DOCUMENT-ORI.md --columns=10  --pdf-engine=xelatex -V colorlinks=true -V linkcolor=blue  -V urlcolor=red  -V toccolor=gray --number-sections -V toc-own-page=true -V footnotes-pretty=true -V table-use-row-color=true --template eisvogeleuropa  -o ./$DOCUMENT.pdf -F mermaid-filter --toc --lof --data-dir=./eu_template --extract-media=./picts --reference-links=true
# rm mermaid-filter.err
echo "$DOCUMENT.pdf Generated"
rm $DOCUMENT-ORI.md
evince "$DOCUMENT.pdf"
