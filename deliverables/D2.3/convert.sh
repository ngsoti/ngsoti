#!/bin/bash
# Usage: ./getmd.sh https://your-hedgedoc/note-id
URL="$1"

DOCUMENT="Deliverable_D2_3"

# Remove trailing slash AND trailing '#'
URL="${URL%/}"
URL="${URL%\#}"
DL_URL="${URL}/download"
CONTENT=$(curl -sL "$DL_URL")
[ -d "./picts" ] || mkdir -p "./picts"

if echo "$CONTENT" | grep -qi "<html"; then
    echo "Error: The note is not public or the server returned HTML."
        exit 1
        fi
        echo "$CONTENT" > $DOCUMENT.md
        echo "Saved as $DOCUMENT.md"
pandoc $DOCUMENT.md --columns=10  --pdf-engine=xelatex -V colorlinks=true -V linkcolor=blue  -V urlcolor=red  -V toccolor=gray --number-sections -V toc-own-page=true -V footnotes-pretty=true -V table-use-row-color=true --template eisvogeleuropa  -o ./$DOCUMENT.pdf -F mermaid-filter --toc --lof --data-dir=./eu_template --extract-media=./picts 
# --lua-filter=table-nowrap.lua --extract-media ./picts
rm mermaid-filter.err
echo "$DOCUMENT.pdf generated"
evince "$DOCUMENT.pdf"
