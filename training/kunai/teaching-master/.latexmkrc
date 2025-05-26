# .latexmkrc
# Set up the style directory
$ENV{"TEXINPUTS"} = "./styles/:" . $ENV{"TEXINPUTS"};
# same as running latexmk -pdf
$pdf_mode = 1;
# set the pdflatex command to use
$pdflatex = 'pdflatex -shell-escape -synctex=1 -interaction=nonstopmode';