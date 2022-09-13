.PHONY:
	clean

all:
	pdflatex coursework.tex
	biber coursework
	pdflatex coursework.tex
	pdflatex coursework.tex
	make clean

clean:
	rm -f *.aux *.bbl *.bcf *.blg *.log *.out *.run.xml *.toc *.lof *.lot *.idx *.ilg *.ind *.synctex.gz
