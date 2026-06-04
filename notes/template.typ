#import "@preview/fletcher:0.5.8" as fletcher: diagram, node, edge

#let project(title: "Project", authors: (), body) = {
	set document(author: authors, title: title)
	set page(fill: black, numbering: "1", number-align: center)
	set text(fill: white, font: "New Computer Modern", lang: "en")
	set heading(numbering: "1.1")
	set par(justify: true)

	body
}
