// #import "@preview/ctheorems:1.1.3": *

// #let definition = thmbox("definition", "Definition", inset: (x: 1.2em, top: 1em))

// #let example = thmbox("example", "Example", inset: (x: 1.2em, top: 1em))

#import "@preview/lemmify:0.1.6": *
#let (
  theorem, lemma, corollary, definition,
  remark, proposition, example,
  proof, rules: thm-rules
) = default-theorems("thm-group", lang: "en", thm-numbering: thm-numbering-heading)