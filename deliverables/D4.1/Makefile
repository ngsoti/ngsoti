# Variables
TEXFILE = ngsoti-deliverable-4.1
PDFLATEX = pdflatex
BIBTEX = biber
TEMP_FILES = *.aux *.log *.out *.bbl *.blg *.toc *.lot *.lof *.gz *.idx *.ilg *.ind
# Default target
all: $(TEXFILE).pdf

# Compile the PDF
$(TEXFILE).pdf: $(TEXFILE).tex
	$(PDFLATEX) $(TEXFILE)
	$(BIBTEX) $(TEXFILE)
	$(PDFLATEX) $(TEXFILE)
	$(PDFLATEX) $(TEXFILE)

# Clean temporary files
clean:
	rm -f $(TEMP_FILES)

# Clean all generated files, including PDF
cleanall: clean
	rm -f $(TEXFILE).pdf

