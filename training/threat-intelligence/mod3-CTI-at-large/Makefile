all: cti qcti
cti: cti-101.md 
	@echo "Compiling cti presentation"
	pandoc --slide-level 2  -H LaTeX_additional_preamble.tex -t beamer cti-101.md -V theme:Torino -o cti-101.pdf
qcti: cti-101.md
	@echo "Compiling cti html file"
	quarto render cti-101.md
clean:
	@echo "Cleaning up..."
	rm cti-101.pdf
	rm -r .quarto  cti-101_files