#!/bin/bash
CONTENTDIR="content"
BUILDDIR="build"
SLIDESNAME="slides"
PAPERNAME="paper"
ASSETSDIR="assets"

download_csl() {
	mkdir "${ASSETSDIR}" -p
	wget -O "${ASSETSDIR}/citation-style.csl" \
		"https://raw.githubusercontent.com/citation-style-language/styles/master/harvard-anglia-ruskin-university.csl"
}

build() {
	mkdir -p "${BUILDDIR}"
	echo "Creating paper output"
	pandoc "${CONTENTDIR}/${PAPERNAME}.md" \
		--resource-path="${CONTENTDIR}" \
		--citeproc \
		--csl="${ASSETSDIR}/citation-style.csl" \
		--from="markdown+tex_math_single_backslash+tex_math_dollars+raw_tex" \
		--to="latex" \
		--output="${BUILDDIR}/output_paper.pdf" \
		--pdf-engine="xelatex" \
		--include-in-header="layouts/print.tex"
	echo "Creating slides output"
	pandoc "${CONTENTDIR}/${SLIDESNAME}.md" \
		--resource-path="${CONTENTDIR}" \
		--citeproc \
		--csl="${ASSETSDIR}/citation-style.csl" \
		--from="markdown+tex_math_single_backslash+tex_math_dollars+raw_tex" \
		--to="beamer" \
		--output="${BUILDDIR}/output_slides.pdf" \
		--pdf-engine="xelatex"
}

# Allows to call a function based on arguments passed to the script
# Example: `./build.sh pdf_print`
$*
