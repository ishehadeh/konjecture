#import "@preview/classic-jmlr:0.4.0": jmlr
#import "util/konane.typ": konane
#import "util/bib.typ": bib
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
  bibliography: bib(),
  appendix: none,
)

#include "partials/01-konane.typ"
#include "partials/02-conjecturing.typ"
#include "partials/03-computation.typ"
#include "partials/04-existing-research.typ"

= Results

We've investigated several different Konane patterns, the most interesting so far are those with a stack of $2 times 1$ alternating pieces, with each layer shifted by one.

#figure(caption: "Stairs n=3")[
  #konane("
  ______
  _xo___
  __xo__
  ___xo_
  ______
  ")
]

So far, computational analysis has show the canonical form for $n$ between $1 "and" 10$. Past 10, the analysis slows down enough that it quickly becomes difficult to compute.

#figure(caption: "Stair Canonical Form")[
  #table(
    columns: 2,
    table.header("Height", "Canonical Form"),
    [1], $*$,
    [2], $*2$,
    [3], $*3$,
    [4], $*$,
    [5], $0$,
    [6], $*$,
    [7], $0$,
    [8], $*$,
    [9], $0$,
    [10], $*3$,
  )
]

== Maximum Penetration of Stair Pattern

It is likely that no matter the height of this pattern, the furthest a piece can ever move vertically or horizontally outside of the bounding box of the staircase (top left to bottom left piece) is $1$. This is true in all cases tested,
futhermore, conisder the game in @fig-borken-stair, this is the result of white jumping left on the bottom stair. It's trivial to show that each piece can only move outside the bounding box by $1$. The same is true for @fig-borken-stair1, even if the pattern continues up to the left. Furthermore, in this configuration the bottom of the staircase is now inaccessible, given the top part has a penetration of $1$. I suspect there's a (relatively easy) inductive proof of this.

#grid(columns: 3, gutter: 1em,[
#figure(caption: "Broken Stair")[
  #konane("
    __
    _xo
    _o__
    __")
]<fig-borken-stair>],
[
#figure(caption: "Broken Stair")[
  #konane("
    xox
    _x__
    ___o_
    __")
]<fig-borken-stair1>])

= Remaining Facts

Below are incomplete sections I would like to add for the final draft.

== Analysis for Stair Case Outcome Class

Even if no general formula is found based on the height of the stair case, I'd like to dig deeper into the pattern.

== Summary of Conjecturing Results

Summary of how conjecturing was used, and the kind of results it gave, even if they weren't useful

== (Maybe) $*2$ 4x3 Games

I'm like ~65% sure that $2*$ configurations within a $4 times 3$ rectangle all more-or-less just the game stairs with $n=2$. It seems like even in games that don't look like it all other pieces are superfluous, even if there are other moves available. Basically, its always that stair configuration that makes the value $*2$, and even if there are other moves those are strictly worse for both players.

This proof seems hard though, and I'm not actually sure its true.