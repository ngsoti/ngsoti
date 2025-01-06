# Variables
TEXFILE = D1.1-NGSOTI-deployment-status-report-1
PDFLATEX = pdflatex
BIBTEX = biber
TEMP_FILES = *.aux *.log *.out *.bbl *.blg *.toc *.lot *.lof *.gz *.idx *.ilg *.ind
INCLUDES = D1.1-NGSOTI-deployment-status-report-1-content.tex
# Default target
all: $(TEXFILE).pdf

# Compile the PDF
$(TEXFILE).pdf: $(TEXFILE).tex $(INCLUDES)
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

