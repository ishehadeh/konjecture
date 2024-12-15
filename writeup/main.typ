#import "@preview/classic-jmlr:0.4.0": jmlr
#import "util/konane.typ": konane
#import "util/theorems.typ": thm-rules


// Make Kōnane easier to type
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
  bibliography: bibliography("main.bib", style: "chicago-author-date"),
  appendix: none,
)

#show: thm-rules
#show regex("Konane"): "Kōnane"


#include "partials/01-konane.typ"
#include "partials/02-conjecturing.typ"
#include "partials/03-computation.typ"
#include "partials/04-existing-research.typ"
#include "partials/05-lot1.typ"

= Conclusion

We did not find any significant results analyzing Konane with Conjecturing.
However, the project had a few interesting side-effects.
The most interesting of which was using Jupyter Note Books and Sage Math for combinatorial game theory research; Creating an efficient implementation of Konane, which integrates with some CGT tools; and small improvements to those tools.

== Further Work

At the time of writing, there is no well-known Sage Math package for combinatorial game theory. Such a package could be useful, since it would allow people reasearching combinatorial game theory to leverage the existing tools that integrate with Sage more easily (for example Conjecturing).
