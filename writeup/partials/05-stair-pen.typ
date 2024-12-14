#import "@preview/lemmify:0.1.6": *
#import "../util/konane.typ": *
#let (
  theorem, lemma, corollary,
  remark, proposition, example,
  proof, rules: thm-rules
) = default-theorems("thm-group", lang: "en")
#show: thm-rules

#let STAIR="STAIR"

#pagebreak()

#lemma[
  The game $STAIR(1)$ has a maximum penetration of $1$.
]<stair1pen>

#let konane-lb = konane.with(hlabel: (n) => n + 1, vlabel: (a) => "ABCDEFGHIJKLMNOPQRSTUVWXYZ".at(a))
#let inflbl = (base, n) => if n == 0 { "..." } else { text(size: 8pt, [n+#{base+n}]) };
#let konane-lb-inf(x, y) = konane.with(hlabel: (n) => inflbl(x, n), vlabel: (n) => rotate(270deg, inflbl(y, n)))

#let knfig(caption, game) = {
  figure(supplement: none, numbering: (_) => counter(figure.where(kind: "game")).display(), caption: caption, kind: "game")[#konane-lb(game)]
}
#grid(columns: (1.5fr, 1fr), gutter: 10pt)[
  #proof[
    Consider the game $STAIR(1)$. Both the left and right player have a single move, shown in games @s1L, and @s1R. In both cases, each player can only move to cell 4, and 1, respectively, which is still within a single cell of the original arrangement. With no other pieces in play, they cannot move further.
  ]
][
  #grid(columns: 2, gutter: 10pt)[
  #knfig("STAIR(1)", "
_xo_
")
  ][][
  #knfig("Right Move", "
___x
")<s1L>
  ][
  #knfig("Left Move", "
o___
")<s1R>
  ]
]

#lemma[
  The game $STAIR(2)$ has a maximum penetration of $1$.
]<stair2pen>

#let konane-lb(x, y) = konane.with(hlabel: (n) => n + 1 + x, vlabel: (a) => "ABCDEFGHIJKLMNOPQRSTUVWXYZ".at(a + y))
#let knfig(caption, game, x: 0, y: 0) = {
  figure(supplement: none, numbering: (_) => counter(figure.where(kind: "game")).display(), caption: caption, kind: "game")[#konane-lb(x,y)(game)]
}
#grid(columns: (1.5fr, 1fr), gutter: 10pt, align: center)[
  #proof[
    Consider the game $STAIR(2)$. This game can play out in two ways. If white jumps to the left, then a $STAIR(1)$ game will remain in C3, C4. And, if black jumps right, then $STAIR(1)$ is left in B2, B3. By @stair1pen we know that neither of these instances of $STAIR(1)$ can interact with the moved piece. Also, we know the remaining moves can have a penetration of 1 in either direction, so it follows the max penetration of $STAIR(2)$ is $1$.
    
    If either player moves vertically, then there neither player can move afterward. For example, if white moves we arrive at the position shown in @s2L1. So, each player can only move out side the bounds of this pattern by one space vertically.
  ]
][
  #grid(columns: 2, gutter: 10pt, align: center + horizon)[
  #knfig("STAIR(2)", "
xo
_xo
", x: 1, y: 1)
  ][  #knfig("Right Jump", "

xo_
___x
", x: 1, y: 1)<s2R1>
  ][  #knfig("Down Jump", "

x__
__o
_o
", x: 1, y: 1)<s2L1>
  ][  #knfig("Left Jump", "

o___
__xo
", x: 0, y: 1)<s2L2>
  ]
]


#lemma[
  The game $STAIR(n)$ has a maximum penetration of $1$.
]<stairNpen>

#proof[
  Consider a position, $STAIR(n + 1)$:
  #konane-lb-inf(1, 1)("xo_\n_xo\n__xo_\n_")
  Assume $STAIR(n)$ (everything up and tothe left of cell $<n + 2, space n + 2>$) has a maximum penetration of $1$ in every direction. Then it follows a stone may land in cell $<n + 3, n + 2>$, or cell $<n + 2, n + 2>$, but no spaces in column $n + 4$, or row $n + 3$. 
]

#theorem(name: "Broken Stairs")[
  Assuming that $STAIR(m)$ has a maximum penetration of $1$, then 
]