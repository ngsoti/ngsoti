# CIRCL Beamer template

## Description

A sleek Beamer template, ready for the future!

It uses the packages:

- minted: for the code blocks (with Pygments).
- awesomebox and fontawesome: modern icons and boxes.

You must have installed:

- texlive-latex-extra
- texlive-fonts-extra

A few slides have been created as examples.

The default layout is 16:9, but you can adjust this easily.


## Compile your presentation

### Using `make`

Simply use the provided Makefile. Example:

```bash
$ make
.
.
.
Output written on presentation.pdf (9 pages, 321589 bytes).
Transcript written on presentation.log.
```

The ``-shell-escape`` option, required for using Minted, is automatically included in ``LATEX_FLAGS``.

The required LaTeX style files are located in the styles folder, and the Makefile handles this configuration.

### Using `latexmk`

`latexmk` is a Perl script designed to automate the process of building LaTeX documents. It is convenient to use when wanting to build the `.tex` file in 
an **IDE** such as **vscode** with **texlab** plugin.

A `.latexmkrc` file is created within the template and includes `latexmk` options to build the project.

```bash
latexmk presentation.tex
```

If we need to **cleanup** build files using `latexmk`

```
latexmk -c
```

## Contribution

Do not hesitate to contribute to this template !


## License