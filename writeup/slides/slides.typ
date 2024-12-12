#import "@preview/touying:0.5.3": *
#import "../util/konane.typ" as kn
#import themes.simple: *

#show regex("Konane"): "Kōnane"


#let alpha = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
#let hlabel = (n) => n + 1
#let vlabel = (n) => alpha.at(n)
#let konane(board) = kn.konane(board, hlabel: hlabel, vlabel: vlabel)
#show: simple-theme.with(aspect-ratio: "16-9")

= Computation and Konane

== The Game

- 2 Players (Black and White)
- Rectangular grid (size varies)
- Move: Capture by jumping $arrow.t$, $arrow.b$, $arrow.l$, $arrow.r$
#figure(caption: text(size: 14pt, "Example Games"), supplement: none,
grid(columns: 3, gutter: 40pt,
    konane("
      xoxo_o
      o__x_x
      xo__xo
      ox__ox
      xoxoxo
    "),
    konane("
      xo_o_o
      oox_x
      x
      ox__ox
      xoxoxo
    "),
    konane("
      xo_o_o
      _____x
      _____o
      o___ox
      xoxoxo
    "),
  )
)

#include "gameplay.typ"

= Solid Linear Patterns

== Who Wins?

#let slp-str(n) = "_" + range(0, n).map(n => if calc.rem(n, 2) == 0 { "x" } else { "o" }).join("") + "_"

#let slp-kn(n) = kn.konane.with(vlabel: none, hlabel: (i) => if i == 0 { [] } else if i <= n { align(center)[#i]  } else { [] })

#let slp(n) = slp-kn(n)(slp-str(n))

#align(left + horizon, pad(left: 4em, [
  #slp(7)
  #slp(8)
]))

#pause

- Only _white_ can move if there's an _odd_ number of stones
- _Both_ can move if there's an _even_ number of stones

== Base Case

Now we need to know the outcome of $"SLP"(2)$ and $"SLP"(1)$

#grid(columns: (1fr, 1fr), align: center + horizon)[
  #slp(1)
][
  #slp(2)
]

== Inductive Case

#grid(columns: 3, gutter: 1em)[
    #slp(8)
][#align(center + horizon)[$-->$]][
  #slp-kn(8)("o__xoxoxo_")
]

- After a jump to the left or right the stone moves out of reach

#pause

#grid(columns: 3, gutter: 1em)[
    #slp(8)
][#align(center + horizon)[$-->$]][
  #kn.konane("o__xoxoxo_", vlabel: none, hlabel: n => if n >= 3 and n <= 8 { align(center)[#{n - 2}] } else {[]})
]

- So, we can think of this new position as $"SLP"(6)$
- In fact, any move by either player moves to $"SLP"(N - 2)$


== Putting it Together

- If $n$ is _even_ then $"SLP"(n) = { "SLP"(n - 2) | "SLP"(n - 2)}$
- If $n$ is _odd_ then $"SLP"(n) = { | "SLP"(n - 2)}$
- $"SLP"(2) = *$
- $"SLP"(0) = "SLP"(1) =  0$ 

#pause
#grid(rows: 2, columns: 2, column-gutter: 2em, row-gutter: 1em, align: left + horizon)[
    If $n$ is *odd* then $n - 2$ is still odd
][
  #alternatives[
    #slp(5)
  ][
    #kn.konane("o__xox_", vlabel: none, hlabel: n => if n >= 3 and n <= 5 { align(center)[#{n - 2}] } else {[]})
  ]
][
  If $n$ is *even* then $n - 2$ is still even
][
  #alternatives[
    #slp(6)
  ][
    #kn.konane("o__xoxo_", vlabel: none, hlabel: n => if n >= 3 and n <= 6 { align(center)[#{n - 2}] } else {[]})
  ]
]

== Solid Linear Pattern

- If $n$ is odd, white always wins.
- If $n$ is even, then the game has $n\/2$ moves
  - Each player has the same option on every turn

= Computation

== Representing Konane on Computers
#let game = "xoxo\noxox\nxoxo\noxox"
#let piece(n, expect) = if lower(game.replace("\n", "").at(n)) == expect { "1" } else { "0" }
#align(center)[
  #konane(game)
]
#let letters = "ABCD"
#v(2em)
#let headers = range(0, 16).map(n => [#{letters.at(int(n / 4))}#{calc.rem(n, 4) + 1}])
#let white = range(0, 16).map(n => align(center + horizon)[#piece(n, "o")])
#let black = range(0, 16).map(n => align(center + horizon)[#piece(n, "x")])
#grid(rows: 3, columns: 17, gutter: 12pt,
  [], ..headers,
  [White], ..white,
  [Black], ..black)

== Representing Konane on Computers

#align(center)[
  #konane("xoXo\noxox\nxoxo\noxox")
]
#let highlight(n, arr) = arr.enumerate().map(((i, content)) => if i == n {
  rect(fill: red.lighten(50%), content, width: 100%, height: 100%)
} else {
  content
})

#v(1em)
#grid(rows: (36pt,)*3, columns: (2fr,) + (1fr,)*16, gutter: -0.5pt, align: center + horizon,
  [], ..highlight(2, headers),
  [White], ..highlight(2, white),
  [Black], ..highlight(2, black))

== Solving Games

1. Recursively generate moves

#grid(columns: 5, rows: 2, gutter: 0.5em, align: horizon)[
    #kn.konane("_xoxo_", vlabel: none)
][$-->$][
    #kn.konane("o__xo_", vlabel: none)
][$-->$][
    #kn.konane("o____x", vlabel: none)
][][$-->$][
    #kn.konane("_xo__x", vlabel: none)
][$-->$][
    #kn.konane("o____x", vlabel: none)
]

2. Determine winners of the leaf nodes
3. Determine winners of ancestor nodes

== Conjecturing

#slide(composer: (3fr, 1em, 1fr))[
#align(center + horizon)[
  #table(
    columns: 4,
    rows: 3,
    align: center + horizon,
    inset: 10pt,
    table.header([Games], [\#Black ($I_1$)], [\#White ($I_2$)], [\#Moves ($I_3$)]),
    
    kn.konane("xox\n_", vlabel: none, hlabel: none),
    $2$, $1$, $0$, 
    
    kn.konane("x_o\nxo", vlabel: none, hlabel: none),
    $2$, $2$, $1$, 

    kn.konane("ox\n_xo", vlabel: none, hlabel: none),
    $2$, $2$, $2$, 
  )
]
][][
  #pause
  #align(center + horizon,
$
  I_2 &<= max(I_1, I_2)\
  I_2 &<= I_1 + I_2\
  I_2 &<= I_1 + 1\
  I_2 &<= I_1 * I_2 + 1 +1

$)
]

== Looking at Results

#align(center + horizon)[
#image("assets/notebook.png", fit: "cover", height: 84%)
]

= Questions?
