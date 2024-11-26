#import "@preview/classic-jmlr:0.4.0": jmlr
#import "util/konane.typ": konane
#import "@preview/ctheorems:1.1.3": *
#show: thmrules

// Make Kōnane easier to type
#show regex("Konane"): "Kōnane"


#let affls = (
  smcm-math: (
    department: "Division of Mathematics",
    institution: "St. Mary's College of Maryland",
    location: "St. Mary's City, Maryland",
    country: "USA"),
)

#let authors = (
  (name: "Ian Shehadeh",
   affl: "smcm-math",
   email: "irshehadeh@smcm.edu"),
)

#show: jmlr.with(
  title: [Auto Conjecturing and Konane],
  authors: (authors, affls),
  abstract: include "partials/00-abstract.typ",
  // keywords: ("keyword one", "keyword two", "keyword three"),
  bibliography: bibliography("main.bib", style: "ieee"),
  appendix: none,
)

#include "partials/01-konane.typ"
#include "partials/02-conjecturing.typ"

= Computation Analysis Konane