all: pdf

pdf: slides.md
	pandoc -t beamer slides.md  --pdf-engine=xelatex -o slides.pdf

clean:
	@echo "Cleaning up..."
	rm slides.pdf